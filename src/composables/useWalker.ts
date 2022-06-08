import { createDir, writeBinaryFile } from '@tauri-apps/api/fs'
import { join as pathJoin } from '@tauri-apps/api/path'
import { CheerioAPI } from 'cheerio'
import sanitize from 'sanitize-filename'
import { Response } from '@tauri-apps/api/http'
import { Page, usePageAPI } from '~~/src/apis/usePageAPI'
import { useQueueAPI } from '~~/src/apis/useQueueAPI'
import { Site, SiteQuery } from '~~/src/apis/useSiteAPI'
import { useProcessLogger } from '~~/src/composables/useProcessLogger'
import ParseUtil from '~~/src/utils/ParseUtil'
import HttpUtil from '~~/src/utils/HttpUtil'
import { InterruptError } from '~~/src/errors/InterruptError'

export const useWalker = (
  processLogger: ReturnType<typeof useProcessLogger>,
  processResult: ReturnType<typeof useProcessResult>,
  pageAPI: ReturnType<typeof usePageAPI>,
  queueAPI: ReturnType<typeof useQueueAPI>,
) => {
  const interrupt = ref<boolean>(false)

  const peek = async (site: Site) => {
    const queues = await queueAPI.list({
      siteId: site.id,
      page: 1,
      perPage: 1,
      order: 'priority',
      desc: true,
    })

    // peek が空なら終了
    const queue = queues.at(0)
    if (!queue) { throw new Error('Peeked value is empty') }

    return queue
  }

  const clear = async (site: Site) => {
    processLogger.event('Clear')
    processResult.init(site)

    // 永続化したページ以外の、ページ（とキュー）全てを削除する
    await pageAPI.clear(site.id, true)

    // ページを作成して、キューに追加する
    await queueAPI.add(site.id, {
      url: site.url,
      priority: 0,
      is_persist: false,
    })
  }

  const reset = async (site: Site) => {
    processLogger.event('Reset')
    processResult.init(site)

    // ページ（とキュー）全てを削除する
    await pageAPI.clear(site.id, false)

    // ページを作成して、キューに追加する
    await queueAPI.add(site.id, {
      url: site.url,
      priority: 0,
      is_persist: false,
    })
  }

  const execute = async (site: Site) => {
    interrupt.value = false
    processLogger.event('Execute')
    processResult.init(site)

    // peek する
    const queue = await peek(site)
    processResult.setQueue(queue)

    // ページを取り出す
    const page = queue.page
    processLogger.info(`Deque > [${page.id}] ${page.url}`)

    // http を叩いて取ってくる
    // TODO: reffer
    const $ = await HttpUtil.fetchBody(page.url, undefined, (res: Response<string>) => {
      processLogger.debug(`Fetch > ${res.data.length.toLocaleString()} byte`)
    })

    // タイトルを取得する
    const title = $('title').text()
    page.title = title
    processLogger.info(`Title > ${title}`)

    // クエリを実行する
    await handleQueries(site.site_queries, $, site, page)

    // ページを保存する
    await pageAPI.update(page.id, {
      site_id: page.site_id,
      parent_page_id: page.parent_page_id,
      url: page.url,
      title: page.title,
      is_persist: page.is_persist,
    })

    // キューからページを削除する
    await queueAPI.remove(queue.id)
    processLogger.event(`Complete > [${page.id}] ${page.url}`)
  }

  /// ////////////////////////////////////////////////////////////

  const handleQueries = async (queries: SiteQuery[], $: CheerioAPI, site: Site, page: Page) => {
    for (const query of queries) {
      processResult.setQueryStatus(query, 'exec')

      // パターンと一致しているか確認（いなければ終了）
      const pattern = new RegExp(query.url_pattern)
      if (!pattern.test(page.url)) {
        processResult.setQueryStatus(query, 'skip')
        continue
      }

      // モード別に処理する
      processLogger.info(`[Query] > ${query.key}`)
      switch (query.processor) {
        case 'extract':
          await handleExtract(query, $, site, page)
          break
        case 'download':
          await handledownload(query, $, site, page)
          break
        default:
          throw new Error(`Illegal process : ${query.processor}`)
      }

      processResult.setQueryStatus(query, 'success')
      if (interrupt.value) { throw new InterruptError() }
    }
  }

  const handleExtract = async (query: SiteQuery, $: CheerioAPI, site: Site, page: Page) => {
    // URL を全て抜き出す
    const links = ParseUtil.extractLinks($, query.dom_selector, query.url_filter)
    processLogger.debug(`Extract > ${links.length} links`)
    processResult.setQueryTaskCnt(query, links.length)

    // キューに追加する（失敗する可能性あり）
    let alreadyCount = 0
    for (const link of links) {
      const res = await queueAPI.add(site.id, {
        url: link,
        parent_page_id: page.id,
        priority: query.priority,
        is_persist: query.is_persist,
      })
      if (res === false) { alreadyCount++ }

      processResult.setQueryTaskIncrement(query)
      if (interrupt.value) { throw new InterruptError() }
    }

    processLogger.debug(`Enque > ${links.length - alreadyCount} links (already: ${alreadyCount})`)
  }

  const handledownload = async (query: SiteQuery, $: CheerioAPI, site: Site, page: Page) => {
    // 親を取り出す // TODO:
    // const parentPage = await pageAPI.get(page.parent_page_id)

    // URL を全て抜き出す
    const links = ParseUtil.extractLinks($, query.dom_selector, query.url_filter)
    const linkCnt = links.length
    processLogger.debug(`Extract > ${linkCnt} links`)
    processResult.setQueryTaskCnt(query, linkCnt)

    // 画像を保存する
    for (const [i, link] of links.entries()) {
      processLogger.info(`[${i + 1}/${linkCnt}] Link > ${link}`)

      // ページが存在するか確認する
      const exist = await pageAPI.count(site.id, link)
      if (exist) {
        processLogger.error(`[${i + 1}/${linkCnt}] Already processed.`)
        processResult.setQueryTaskIncrement(query)
        continue
      }

      // 画像を取得する
      // TODO: reffer
      const blob = await HttpUtil.fetchBlob(link, undefined, (res: Response<number[]>) => {
        processLogger.debug(`Fetch > ${res.data.length.toLocaleString()} byte`)
      })
      processResult.latestBlob.value = blob

      // ディレクトリチェック
      const dirPath = await pathJoin(
        'temp',
        sanitize(site.title || 'unknown'),
        sanitize(page.title || 'unknown'),
      )
      await createDir(dirPath, { recursive: true })

      // パス生成
      const fileName = ParseUtil.urlLastName(link)
      const filePath = await pathJoin(
        dirPath,
        sanitize(fileName || new Date().getTime().toString()),
      )
      processLogger.debug(`Path > ${filePath}`)

      // バイナリを保存する
      await writeBinaryFile({
        path: filePath,
        contents: blob,
      })

      // ページとして保存する
      await pageAPI.create({
        site_id: site.id,
        parent_page_id: page.id,
        url: link,
        title: fileName,
        is_persist: query.is_persist,
      })

      processResult.setQueryTaskIncrement(query)
      if (interrupt.value) { throw new InterruptError() }
    }

    // TODO: 画像重複チェック

    processLogger.debug(`Download > ${links.length} links`)
  }

  return {
    interrupt,

    peek,
    clear,
    reset,
    execute,
  }
}
