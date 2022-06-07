import { createDir, writeBinaryFile } from '@tauri-apps/api/fs'
import { join as pathJoin } from '@tauri-apps/api/path'
import { CheerioAPI } from 'cheerio'
import sanitize from 'sanitize-filename'
import { Response } from '@tauri-apps/api/http'
import { Page, usePageAPI } from '~~/src/apis/usePageAPI'
import { Queue, useQueueAPI } from '~~/src/apis/useQueueAPI'
import { Site, SiteQuery } from '~~/src/apis/useSiteAPI'
import { useProcessLogger } from '~~/src/composables/useProcessLogger'
import ParseUtil from '~~/src/utils/ParseUtil'
import HttpUtil from '~~/src/utils/HttpUtil'

export const useWalker = (
  processLogger: ReturnType<typeof useProcessLogger>,
  pageAPI: ReturnType<typeof usePageAPI>,
  queueAPI: ReturnType<typeof useQueueAPI>,
) => {
  // TODO: 統計を別クラスに分ける
  const selectedQueue = ref<Queue>()
  const execQuery = ref<SiteQuery>()
  const nowTask = ref<number>(0)
  const maxTask = ref<number>(0)
  const latestBlob = ref<Buffer>()

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

  const reset = async (site: Site) => {
    // ページ（とキュー）全てを削除する
    selectedQueue.value = null
    await pageAPI.clear(site.id)

    // ページを作成して、キューに追加する
    await queueAPI.add(site.id, {
      url: site.url,
      priority: 0,
    })
  }

  const execute = async (site: Site) => {
    processLogger.event('Execute')
    selectedQueue.value = null
    execQuery.value = null
    nowTask.value = 0
    maxTask.value = 1

    // peek する
    const queue = await peek(site)
    selectedQueue.value = queue

    // ページを取り出す
    const page = selectedQueue.value.page
    processLogger.info(`Deque > [${page.id}] ${page.url}`)

    // http を叩いて取ってくる
    // TODO: reffer
    const $ = await HttpUtil.fetchBody(page.url, undefined, (res: Response<string>) => {
      processLogger.info(`Fetch > ${res.data.length.toLocaleString()} byte`)
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
    })

    // キューからページを削除する
    await queueAPI.remove(queue.id)
    processLogger.event(`Complete > [${page.id}] ${page.url}`)
  }

  /// ////////////////////////////////////////////////////////////

  const handleQueries = async (queries: SiteQuery[], $: CheerioAPI, site: Site, page: Page) => {
    for (const query of queries) {
      // パターンと一致しているか確認
      const pattern = new RegExp(query.url_pattern)
      if (!pattern.test(page.url)) { continue }

      // モード別に処理する
      execQuery.value = query
      processLogger.info(`Query > ${query.key}`)
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
    }
  }

  const handleExtract = async (query: SiteQuery, $: CheerioAPI, site: Site, page: Page) => {
    nowTask.value = 0
    maxTask.value = 1

    // URL を全て抜き出す
    const links = ParseUtil.extractLinks($, query.dom_selector, query.url_filter)
    processLogger.debug(`Extract > ${links.length} links`)

    // キューに追加する（失敗する可能性あり）
    let alreadyCount = 0
    for (const link of links) {
      const res = await queueAPI.add(site.id, {
        url: link,
        priority: query.priority,
        parent_page_id: page.id,
      })
      if (res === false) { alreadyCount++ }
    }

    nowTask.value = 1

    processLogger.debug(`Enque > ${links.length - alreadyCount} links (already: ${alreadyCount})`)
  }

  const handledownload = async (query: SiteQuery, $: CheerioAPI, site: Site, page: Page) => {
    nowTask.value = 0
    maxTask.value = 1

    // 親を取り出す // TODO:
    // const parentPage = await pageAPI.get(page.parent_page_id)

    // URL を全て抜き出す
    const links = ParseUtil.extractLinks($, query.dom_selector, query.url_filter)
    maxTask.value = links.length
    processLogger.debug(`Extract > ${links.length} links`)

    // 画像を保存する
    for (const link of links) {
      // 画像を取得する
    // TODO: reffer
      const blob = await HttpUtil.fetchBlob(link, undefined, (res: Response<Buffer>) => {
        processLogger.info(`Fetch > ${res.data.length.toLocaleString()} byte`)
      })
      latestBlob.value = blob

      // ディレクトリチェック
      const dirPath = await pathJoin(
        'temp',
        sanitize(site.title || 'unknown'),
        sanitize(page.title || 'unknown'),
      )
      await createDir(dirPath, { recursive: true })

      // パス生成
      const name = ParseUtil.urlLastName(link)
      const filePath = await pathJoin(
        dirPath,
        sanitize(name || new Date().getTime().toString()),
      )
      processLogger.debug(`Path > ${filePath}`)

      // バイナリを保存する
      await writeBinaryFile({
        path: filePath,
        contents: blob,
      })
      nowTask.value++
    }

    // TODO: 画像重複チェック

    processLogger.debug(`Download > ${links.length} links`)
  }

  return {
    selectedQueue,
    execQuery,
    nowTask,
    maxTask,
    latestBlob,

    peek,
    reset,
    execute,
  }
}
