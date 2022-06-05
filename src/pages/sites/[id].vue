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
    </div>

    <hr>

    <div class="m-2">
      {{ selectedQueue }}
    </div>

    <hr>

    <div class="m-2">
      {{ site }}
    </div>

    <div class="m-2">
      {{ pages }}
    </div>

    <div class="m-2">
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
import { useToast } from 'primevue/usetoast'
import { Site, useSiteAPI } from '@/apis/useSiteAPI'
import { Page, usePageAPI } from '~~/src/apis/usePageAPI'
import { Queue, useQueueAPI } from '~~/src/apis/useQueueAPI'

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

    // タイトルを作成する
    const page = await pageAPI.create({
      site_id: site.value.id,
      url: site.value.url,
      title: site.value.title, // TODO: 仮追加
    })

    // キューに追加する
    await queueAPI.create({
      site_id: site.value.id,
      page_id: page.id,
      priority: 0,
    })
  } catch (err) {
    toast.add({ severity: 'error', summary: 'エラーが発生しました', detail: err })
  } finally {
    loading.value = false
  }

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
