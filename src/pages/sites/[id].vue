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

      <Button class="m-1" label="Peek" @click="onPeek" />

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
import { load as cheerioLoad } from 'cheerio'
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

const selectedQueue = ref<Queue>()

const onPeek = async () => {
  loading.value = true

  try {
    // 先頭のキューを取り出す
    const queues = await queueAPI.list({
      siteId: siteId.value,
      page: 1,
      perPage: 1,
      order: 'priority',
      desc: true,
    })
    selectedQueue.value = queues.at(0)
  } catch (err) {
    toast.add({ severity: 'error', summary: 'エラーが発生しました', detail: err })
  } finally {
    loading.value = false
  }
}

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

const onExecute = async () => {
  // peek したキューを取り出す // TODO:
  const queue = selectedQueue.value
  if (!queue) { throw new Error('Peeked value is empty') }
  const page = queue.page

  // http を叩いて取ってくる
  const data = await fetch(page.url, {
    method: 'GET',
    responseType: ResponseType.Text,
  })

  // dom変換する
  const body = data.data as string
  const $ = cheerioLoad(body)

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
            await queueAPI.add(site.value.id, {
              url: link,
              priority: query.priority,
            })
          }
        })()
        break
      case 'image':
      default:
        throw new Error(`Illegal process : ${query.processor}`)
    }
  }

  // タイトルを追記する
  const title = $('title').text()
  page.title = title

  // ページを保存する
  await pageAPI.update(page.id, {
    site_id: page.site_id,
    parent_id: page.parent_id,
    url: page.url,
    title: page.title,
  })

  // キューからページを削除する
  await queueAPI.remove(queue.id)
  selectedQueue.value = null

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
