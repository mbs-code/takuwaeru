<template>
  <Dialog
    v-model:visible="_show"
    class="org-dialog"
    :draggable="false"
    modal
    :style="{ width: '480px' }"
    @keydown.enter.ctrl="onSubmit"
  >
    <template #header>
      <div>
        <Avatar
          class="mx-1"
          :class="{ 'info-color': !isEdit, 'success-color': isEdit }"
          icon="pi pi-database"
        />
        <span class="mx-1 p-dialog-title">
          {{ isEdit ? 'サイト設定の編集' : '新規サイト設定' }}
        </span>
      </div>
    </template>

    <div class="my-4">
      <label class="block">識別記号* (英数字)</label>
      <InputText
        v-model="form.key"
        autofocus
        class="block w-full"
        :disabled="loading"
      />
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
          class="p-button-danger"
          :disabled="loading"
          icon="pi pi-minus"
          type="button"
          @click="openDeleteDialog"
        />

        <Button
          class="p-button-outlined p-button-secondary"
          :disabled="loading"
          label="リセット"
          type="button"
          @click="onReset"
        />

        <div class="flex-grow-1" />

        <Button
          :class="{ 'p-button-info': !isEdit, 'p-button-success': isEdit }"
          :disabled="loading"
          :icon="loading ? 'pi pi-spin pi-spinner' :'pi pi-save'"
          label="保存"
          type="submit"
          @click="onSubmit"
        />
      </div>
    </template>
  </Dialog>
</template>

<script setup lang="ts">
import { computed } from 'vue'
import { useToast } from 'primevue/usetoast'
import { useConfirm } from 'primevue/useconfirm'
import { Site, FormSite, useSiteAPI } from '@/apis/useSiteAPI'

const siteAPI = useSiteAPI()
const toast = useToast()
const confirm = useConfirm()

const props = defineProps<{
  show: boolean,
  site: Site,
}>()

// eslint-disable-next-line func-call-spacing
const emit = defineEmits<{
  (event: 'update:show', show: boolean): void,
  (event: 'saved', site: Site): void,
  (event: 'removed', site: Site): void,
}>()

const _show = computed({
  get: () => props.show,
  set: (value: boolean) => emit('update:show', value),
})

/// //////////////////////////////////////////////////

const isEdit = computed(() => props.site?.id > 0)

const loading = ref<boolean>(false)
const form = reactive<FormSite>({
  key: '',
  url: '',
  title: '',
  site_queries: [],
})

watch(_show, () => onReset())

/// //////////////////////////////////////////////////

const openDeleteDialog = () => {
  confirm.require({
    header: 'サイト設定の削除',
    message: '全ての記録を削除します。よろしいですか？',
    icon: 'pi pi-info-circle',
    acceptClass: 'p-button-danger',
    acceptLabel: '削除',
    acceptIcon: 'pi pi-trash',
    rejectLabel: 'キャンセル',
    accept: async () => await onRemove(),
  })
}

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

    toast.add({
      severity: 'success',
      summary: 'サイト情報を保存しました',
      detail: `[${newSite.id}] ${newSite.key}: ${newSite.title}`,
      life: 3000
    })

    emit('saved', newSite)
    onClose()
  } catch (err) {
    toast.add({ severity: 'error', summary: 'エラーが発生しました', detail: err })
  } finally {
    loading.value = false
  }
}

const onRemove = async () => {
  loading.value = true

  try {
    const site = props.site
    await siteAPI.remove(props.site?.id)

    toast.add({
      severity: 'success',
      summary: 'サイト情報を削除しました',
      detail: `[${site.id}] ${site.key}: ${site.title}`,
      life: 3000
    })

    emit('removed', site)
    onClose()
  } catch (err) {
    toast.add({ severity: 'error', summary: 'エラーが発生しました', detail: err })
  } finally {
    loading.value = false
  }
}

const onClose = () => {
  _show.value = false
}
</script>
