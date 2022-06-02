<template>
  <Dialog
    v-model:visible="_show"
    class="org-dialog"
    modal
    :draggable="false"
    :style="{ width: '480px' }"
  >
    <template #header>
      <div>
        <Avatar class="mx-2 edit-color" icon="pi pi-database" />
        <span class="mx-2 p-dialog-title">サイト情報編集</span>
      </div>
    </template>

    <div class="my-4">
      <label class="block">識別記号* (英数字)</label>
      <InputText v-model="form.key" class="block w-full" :disabled="loading" />
    </div>

    <div class="my-4">
      <label class="block">URL*</label>
      <InputText v-model="form.url" class="block w-full" :disabled="loading" />
    </div>

    <div class="my-4">
      <label class="block">タイトル*</label>
      <InputText v-model="form.title" class="block w-full" :disabled="loading" />
    </div>

    <template #footer>
      <div class="flex">
        <Button
          label="リセット"
          class="mx-2 p-button-sm p-button-outlined p-button-secondary"
          :disabled="loading"
          @click="onReset"
        />

        <div class="flex-grow-1" />

        <Button
          label="保存"
          class="mx-2 p-button-sm p-button-success"
          :icon="loading ? 'pi pi-spin pi-spinner' :'pi pi-save'"
          :disabled="loading"
          @click="onSubmit"
        />
      </div>
    </template>
  </Dialog>
</template>

<script setup lang="ts">
import { computed } from 'vue'
import { Site, SiteParam, useSiteAPI } from '@/apis/useSiteAPI'

const props = defineProps<{
  show: boolean,
  site: Site,
}>()

// eslint-disable-next-line func-call-spacing
const emit = defineEmits<{
  (event: 'update:show', show: boolean): void,
  (event: 'saved', site: Site): void,
}>()
const siteAPI = useSiteAPI()

const _show = computed({
  get: () => props.show,
  set: (value: boolean) => emit('update:show', value),
})

const onClose = () => { _show.value = false }

///

const isEdit = computed(() => props.site?.id > 0)

const loading = ref<boolean>(false)
const form = reactive<SiteParam>({
  key: '',
  url: '',
  title: '',
  site_queries: [],
})

watch(_show, () => onReset())

const onReset = () => {
  form.key = props.site?.key ?? ''
  form.url = props.site?.url ?? ''
  form.title = props.site?.title ?? ''
  form.site_queries = props.site?.site_queries.map(query => ({
    key: query.key ?? '',
    url_pattern: query.url_pattern ?? '',
    processor: query.processor ?? 'extract',
    url_filter: query.url_filter ?? '',
    priority: query.priority ?? 0,
  })) ?? []
}

const onSubmit = async () => {
  loading.value = true

  try {
    const newSite = isEdit.value
      ? await siteAPI.update(props.site?.id, form)
      : await siteAPI.create(form)

    emit('saved', newSite)
    onClose()
  } catch (err) {
    console.log(err)
  } finally {
    loading.value = false
  }
}

</script>
