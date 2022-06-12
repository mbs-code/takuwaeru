<template>
  <DataTable
    v-model:rows="perPage"
    class="h-full"
    frozen
    lazy
    paginator
    paginator-position="top"
    responsive-layout="scroll"
    :rows-per-page-options="[10, 20, 50, 100]"
    :scroll-height="`${height - 130}px`"
    :scrollable="true"
    :total-records="total"
    :value="items"
    @page="onChange"
  >
    <template #paginatorstart>
      <Button
        class="p-button-info"
        icon="pi pi-plus"
        type="button"
        @click="openEditDialog(null)"
      />
    </template>

    <Column field="id" header="ID" />
    <Column field="url" header="URL" />
    <Column field="title" header="タイトル" />
    <Column field="queryCount" header="登録クエリ数" />

    <Column>
      <template #body="{ data }">
        <nuxt-link class="mx-1 no-underline-all" :to="{ name: 'sites-id', params: { id: data.id } }">
          <Button class="p-button-info" icon="pi pi-folder" type="button" />
        </nuxt-link>

        <Button
          class="mx-1 p-button-success"
          icon="pi pi-pencil"
          type="button"
          @click="openEditDialog(data)"
        />
      </template>
    </Column>
  </DataTable>

  <SiteEditDialog
    v-model:show="showEditDialog"
    :site="selectedSite"
    @removed="fetchSites"
    @saved="fetchSites"
  />
</template>

<script setup lang="ts">
import { useToast } from 'primevue/usetoast'
import { DataTablePageEvent } from 'primevue/datatable'
import { useWindowSize } from 'vue-window-size'
import { Site, useSiteAPI } from '@/apis/useSiteAPI'

const siteAPI = useSiteAPI()
const toast = useToast()
const { height } = useWindowSize()

/// ////////////////////////////////////////////////////////////

const sites = ref<Site[]>([])
const items = computed(() => sites.value.map((site: Site) => ({
  ...site,
  queryCount: site.site_queries.length,
})))

const loading = ref<boolean>(false)
const page = ref<number>(1)
const total = ref<number>(0)
const perPage = ref<number>(10)

onMounted(async () => { await fetchSites() })

const fetchSites = async () => {
  loading.value = true

  try {
    total.value = await siteAPI.count()
    sites.value = await siteAPI.list({
      page: page.value,
      perPage: perPage.value,
    // order?: string,
    // desc?: boolean,
    })
  } catch (err) {
    toast.add({ severity: 'error', summary: 'エラーが発生しました', detail: err })
  } finally {
    loading.value = false
  }
}

const onChange = async (event: DataTablePageEvent) => {
  page.value = event.page + 1
  await fetchSites()
}

/// ////////////////////////////////////////////////////////////

const selectedSite = ref<Site>()

const showEditDialog = ref<boolean>(false)
const openEditDialog = (selected?: Site) => {
  selectedSite.value = selected
  showEditDialog.value = true
}
</script>
