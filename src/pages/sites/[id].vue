<template>
  <div>
    <div class="grid m-0">
      <div class="col-12 md:col-6">
        <SiteInfoPanel :site="site" />

        <div class="h-1rem" />
        <SiteImagePanel :blob="processResult.latestBlob.value" />
      </div>

      <div class="col-12 md:col-6">
        <SiteControlPanel
          :loading="loading"
          :page-count="pageCount"
          :process-result="processResult"
          :queue-count="queueCount"
          @onClear="onClear"
          @onEdit="showEditModal = true"
          @onExecute="onExecute"
          @onInterrupt="onInterrupt"
          @onReset="onReset"
        />

        <div class="h-1rem" />
        <SiteLogPanel :logs="processLogger.logs.value" />
      </div>
    </div>

    <hr>

    <div class="m-2">
      <div>サイト：</div>
      {{ site }}
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
import { usePageAPI } from '~~/src/apis/usePageAPI'
import { useQueueAPI } from '~~/src/apis/useQueueAPI'
import { useProcessLogger } from '~~/src/composables/useProcessLogger'

const route = useRoute()
const router = useRouter()
const toast = useToast()

const processLogger = useProcessLogger()
const processResult = useProcessResult()

const siteAPI = useSiteAPI()
const pageAPI = usePageAPI()
const queueAPI = useQueueAPI()

const walker = useWalker(processLogger, processResult, pageAPI, queueAPI)

/// ////////////////////////////////////////////////////////////

const site = ref<Site>()
const queueCount = ref<number>(0)
const pageCount = ref<number>(0)

const siteId = ref<number>()
const loading = ref<boolean>(false)

onMounted(async () => {
  const idStr = route.params.id as unknown as string
  siteId.value = parseInt(idStr)
  await fetchSite()

  processResult.init(site.value)
})

const fetchSite = async () => {
  loading.value = true

  try {
    site.value = await siteAPI.get(siteId.value)

    queueCount.value = await queueAPI.count(siteId.value)
    pageCount.value = await pageAPI.count(siteId.value)
  } catch (err) {
    toast.add({ severity: 'error', summary: 'エラーが発生しました', detail: err })
  } finally {
    loading.value = false
  }
}

/// ////////////////////////////////////////////////////////////

const onClear = async () => {
  loading.value = true

  try {
    walker.clear(site.value)
    await fetchSite()
  } catch (err) {
    toast.add({ severity: 'error', summary: 'エラーが発生しました', detail: err })
  } finally {
    loading.value = false
  }
}

const onReset = async () => {
  loading.value = true

  try {
    walker.reset(site.value)
    await fetchSite()
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
    await fetchSite()
  } catch (err) {
    toast.add({ severity: 'error', summary: 'エラーが発生しました', detail: err })
  } finally {
    loading.value = false
  }
}

const onInterrupt = () => {
  walker.interrupt.value = true
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
