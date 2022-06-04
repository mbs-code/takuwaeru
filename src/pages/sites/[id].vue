<template>
  <div>
    <div>
      <Button class="m-1" label="Edit" @click="showEditModal = true" />

      <Button class="m-1" label="Reset" @click="onReset" />
    </div>

    <div>{{ site }}</div>
    <div>{{ pages }}</div>

    <!-- <p>{{ $route.params.id }}</p>
    <p>{{ site }}</p>

    <div class="card card-bordered card-compact border-sky-500 border-1">
      <div class="card-body">
        <h2 class="card-title">
          <div class="badge badge-secondary">
            {{ site.key }}
          </div>
          {{ site.title }}
        </h2>
        <p>{{ site.url }}</p>
      </div>
    </div>

    <div v-for="(query, _) in site.site_queries" :key="_">
      {{ query }}
    </div> -->

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

const route = useRoute()
const router = useRouter()
const toast = useToast()

const siteAPI = useSiteAPI()
const pageAPI = usePageAPI()

/// ////////////////////////////////////////////////////////////

const site = ref<Site>()
const pages = ref<Page[]>()
const loading = ref<boolean>(false)

onMounted(async () => {
  const idStr = route.params.id as unknown as string
  const id = parseInt(idStr)
  await fetchSite(id)
})

const fetchSite = async (siteId: number) => {
  loading.value = true

  try {
    site.value = await siteAPI.get(siteId)
    pages.value = await pageAPI.list({
      siteId: site.value.id,
      page: 1,
      perPage: 100,
      order: 'id',
    })
  } catch (err) {
    toast.add({ severity: 'error', summary: 'エラーが発生しました', detail: err })
  } finally {
    loading.value = false
  }
}

/// ////////////////////////////////////////////////////////////

const onReset = async () => {
  // 全てを削除する
  await pageAPI.clear(site.value.id)

  // タイトルを作成する
  await pageAPI.create({
    site_id: site.value.id,
    url: site.value.url,
    title: site.value.title, // TODO: 仮追加
  })

  // 更新
  await fetchSite(site.value.id)
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
