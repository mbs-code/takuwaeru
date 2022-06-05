<template>
  <div>
    <div>
      <Button
        class="m-1 p-button-success"
        label="Edit"
        @click="showEditModal = true"
      />

      <Button
        class="m-1 p-button-danger"
        label="Reset"
        @click="onReset"
      />
      <Button class="m-1" label="Execute" @click="onExecute" />
    </div>

    <hr>

    <div class="m-2">
      <div>選択中：</div>
      {{ selectedQueue }}
    </div>

    <hr>

    <div class="m-2">
      <div>サイト：</div>
      {{ site }}
    </div>

    <div class="m-2">
      <div>ページ：</div>
      {{ pages }}
    </div>

    <div class="m-2">
      <div>キュー：</div>
      {{ queues }}
    </div>

    <SiteEditDialog
      v-model:show="showEditModal"
      :site="site"
      @removed="onRemoved"
      @saved="onSaved"
    />
  </div>
</template>

<script setup lang="ts">
import { fetch, ResponseType } from '@tauri-apps/api/http'
import { createDir, writeBinaryFile } from '@tauri-apps/api/fs'
import { join as pathJoin } from '@tauri-apps/api/path'
import { load as cheerioLoad } from 'cheerio'
import sanitize from 'sanitize-filename'
import { useToast } from 'primevue/usetoast'
import { Site, useSiteAPI } from '@/apis/useSiteAPI'
import { Page, usePageAPI } from '~~/src/apis/usePageAPI'
import { Queue, useQueueAPI } from '~~/src/apis/useQueueAPI'
import ParseUtil from '~~/src/utils/ParseUtil'

const route = useRoute()
const router = useRouter()
const toast = useToast()

const siteAPI = useSiteAPI()
const pageAPI = usePageAPI()
const queueAPI = useQueueAPI()

/// ////////////////////////////////////////////////////////////

const site = ref<Site>()
const pages = ref<Page[]>()
const queues = ref<Queue[]>()

const siteId = ref<number>()
const loading = ref<boolean>(false)

onMounted(async () => {
  const idStr = route.params.id as unknown as string
  siteId.value = parseInt(idStr)
  await fetchSite()
})

const fetchSite = async () => {
  loading.value = true

  try {
    site.value = await siteAPI.get(siteId.value)
    pages.value = await pageAPI.list({
      siteId: siteId.value,
      page: 1,
      perPage: 100,
    })
    queues.value = await queueAPI.list({
      siteId: siteId.value,
      page: 1,
      perPage: 100,
      order: 'priority',
      desc: true,
    })
  } catch (err) {
    toast.add({ severity: 'error', summary: 'エラーが発生しました', detail: err })
  } finally {
    loading.value = false
  }
}

/// ////////////////////////////////////////////////////////////

const onReset = async () => {
  loading.value = true

  try {
    // ページ（とキュー）全てを削除する
    selectedQueue.value = null
    await pageAPI.clear(site.value.id)

    // ページを作成して、キューに追加する
    await queueAPI.add(site.value.id, {
      url: site.value.url,
      priority: 0,
    })

    // 更新
    await fetchSite()
  } catch (err) {
    toast.add({ severity: 'error', summary: 'エラーが発生しました', detail: err })
  } finally {
    loading.value = false
  }
}

/// ////////////////////////////////////////////////////////////

const selectedQueue = ref<Queue>()

const onExecute = async () => {
  // peek する
  const queues = await queueAPI.list({
    siteId: siteId.value,
    page: 1,
    perPage: 1,
    order: 'priority',
    desc: true,
  })
  const queue = queues.at(0)
  selectedQueue.value = queue

  // peek が空なら終了
  if (!selectedQueue.value) { throw new Error('Peeked value is empty') }

  // http を叩いて取ってくる
  const page = selectedQueue.value.page
  const data = await fetch(page.url, {
    method: 'GET',
    responseType: ResponseType.Text,
  })

  // dom変換する
  const body = data.data as string
  const $ = cheerioLoad(body)

  // タイトルを取得する
  const title = $('title').text()
  page.title = title

  // クエリを実行する
  const queries = site.value.site_queries
  for (const query of queries) {
    // パターンと一致しているか
    const pattern = new RegExp(query.url_pattern)
    if (!pattern.test(page.url)) { continue }

    // モードによって抽出する
    switch (query.processor) {
      case 'extract':
        await (async () => {
          // URL を全て抜き出す
          const links = ParseUtil.extractLinks($, query.dom_selector, query.url_filter)

          // キューに追加する（失敗する可能性あり）
          for (const link of links) {
            // TODO: 失敗したら false
            await queueAPI.add(site.value.id, {
              url: link,
              priority: query.priority,
              parent_page_id: page.id,
            })
          }
        })()
        break
      case 'download':
        await (async () => {
          // 親を取り出す // TODO:
          // const parentPage = await pageAPI.get(page.parent_page_id)

          // URL を全て抜き出す
          const links = ParseUtil.extractLinks($, query.dom_selector, query.url_filter)

          // 画像を保存する
          for (const link of links) {
            // 画像を取得する
            const blobRes = await fetch(link, {
              method: 'GET',
              responseType: ResponseType.Binary,
              headers: {
                Referer: page.url
              }
            })

            // ファイル名を生成する
            const u = new URL(link)
            const pathname = u.pathname
            const lastname = pathname.slice(Math.max(0, pathname.lastIndexOf('/') + 1))

            // ディレクトリチェック
            const dirPath = await pathJoin(
              sanitize(site.value.title || 'unknown'),
              sanitize(page.title || 'unknown'),
            )
            await createDir(dirPath, { recursive: true })

            // バイナリを保存する
            const filePath = await pathJoin(
              dirPath,
              sanitize(lastname || new Date().getTime().toString()),
            )
            await writeBinaryFile({
              path: filePath,
              contents: blobRes.data as Iterable<number>
            })
          }
        })()
        break
      default:
        throw new Error(`Illegal process : ${query.processor}`)
    }
  }

  // ページを保存する
  await pageAPI.update(page.id, {
    site_id: page.site_id,
    parent_page_id: page.parent_page_id,
    url: page.url,
    title: page.title,
  })

  // キューからページを削除する
  await queueAPI.remove(queue.id)

  // 更新
  await fetchSite()
}

/// ////////////////////////////////////////////////////////////

const showEditModal = ref<boolean>(false)
const onSaved = (newSite: Site) => {
  site.value = newSite
}

const onRemoved = () => {
  router.push({ name: 'index' })
}
</script>
