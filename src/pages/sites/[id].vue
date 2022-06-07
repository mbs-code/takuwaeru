<template>
  <div>
    <div class="grid m-0">
      <div class="col-12 md:col-6">
        <SiteInfoPanel :site="site" />
        queue: {{ queueCount }}

        <div class="h-1rem" />
        <SiteImagePanel :blob="walker.latestBlob.value" />
      </div>

      <div class="col-12 md:col-6">
        <SiteLogPanel :logs="processLogger.logs.value" />

        <div class="h-1rem" />
        <SiteControlPanel :site="site" :walker="walker" />
      </div>
    </div>

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
      {{ walker.selectedQueue }}
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
import { useToast } from 'primevue/usetoast'
import { Site, useSiteAPI } from '@/apis/useSiteAPI'
import { Page, usePageAPI } from '~~/src/apis/usePageAPI'
import { Queue, useQueueAPI } from '~~/src/apis/useQueueAPI'
import { useProcessLogger } from '~~/src/composables/useProcessLogger'

const route = useRoute()
const router = useRouter()
const toast = useToast()

const processLogger = useProcessLogger()

const siteAPI = useSiteAPI()
const pageAPI = usePageAPI()
const queueAPI = useQueueAPI()

const walker = useWalker(processLogger, pageAPI, queueAPI)
const perTask = computed(() =>
  parseFloat((walker.nowTask.value / walker.maxTask.value * 100).toFixed(1))
)

/// ////////////////////////////////////////////////////////////

const site = ref<Site>()
const pages = ref<Page[]>()
const queues = ref<Queue[]>()

const queueCount = ref<number>(0)

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

    queueCount.value = await queueAPI.count(siteId.value)
  } catch (err) {
    toast.add({ severity: 'error', summary: 'エラーが発生しました', detail: err })
  } finally {
    loading.value = false
  }
}

/// ////////////////////////////////////////////////////////////

const onRefresh = async () => {
  await fetchSite()
}

const onReset = async () => {
  loading.value = true

  try {
    walker.reset(site.value)
    await onRefresh()
  } catch (err) {
    toast.add({ severity: 'error', summary: 'エラーが発生しました', detail: err })
  } finally {
    loading.value = false
  }
}

const onExecute = async () => {
  loading.value = true

  try {
    await walker.execute(site.value)
    await onRefresh()
  } catch (err) {
    toast.add({ severity: 'error', summary: 'エラーが発生しました', detail: err })
  } finally {
    loading.value = false
  }
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
