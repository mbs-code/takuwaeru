<template>
  <Dialog
    v-model:visible="_show"
    class="org-dialog"
    :draggable="false"
    modal
    :style="{ width: '960px' }"
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

    <div class="grid m-0">
      <div class="col-12 md:col-4">
        <div class="flex-column grid">
          <div class="col">
            <label>キー*</label>
            <InputText
              v-model="form.key"
              autofocus
              class="block w-full"
              :disabled="loading"
            />
          </div>

          <div class="col">
            <label>URL*</label>
            <InputText
              v-model="form.url"
              class="block w-full"
              :disabled="loading"
            />
          </div>

          <div class="col">
            <label>タイトル*</label>
            <InputText
              v-model="form.title"
              class="block w-full"
              :disabled="loading"
            />
          </div>
        </div>
      </div>

      <div class="col-12 md:col-8">
        <div class="flex-column grid">
          <div class="col w-full">
            <label>クエリ （{{ form.site_queries.length }}）</label>
            <TabView
              v-model:active-index="queryTabIndex"
              class="p-1 simple-border"
              scrollable
            >
              <TabPanel
                v-for="(query, _) in form.site_queries"
                :key="_"
                :header="`${_ + 1}: ${queryTabNames[_] || 'new'}`"
              >
                <div class="flex-column grid">
                  <div class="col">
                    <div class="align-items-center flex">
                      <label class="w-8rem">キー*</label>
                      <div class="align-items-center flex w-full">
                        <InputText
                          v-model="query.key"
                          class="block w-full"
                          :disabled="loading"
                          @change="updateQuerytabNames"
                        />

                        <Button
                          class="ml-2 p-button-danger p-button-rounded p-button-text"
                          :disabled="loading"
                          icon="pi pi-minus-circle"
                          type="button"
                          @click="removeSiteQuery(_)"
                        />
                      </div>
                    </div>
                  </div>

                  <div class="col">
                    <div class="align-items-center flex">
                      <label class="w-8rem">対象URL*</label>
                      <InputTextRegex
                        v-model="query.url_pattern"
                        :disabled="loading"
                      />
                    </div>
                  </div>

                  <div class="col">
                    <div class="align-items-center flex">
                      <label class="w-8rem">モード*</label>
                      <SelectButton
                        v-model="query.processor"
                        class="w-full"
                        option-label="name"
                        option-value="value"
                        :options="processotTypes"
                      />
                    </div>
                  </div>
                  <div class="col">
                    <div class="align-items-center flex">
                      <label class="w-8rem">対象DOM</label>
                      <InputText
                        v-model="query.dom_selector"
                        class="block w-full"
                        :disabled="loading"
                        placeholder="[href],[src]"
                      />
                    </div>
                  </div>

                  <div class="col">
                    <div class="align-items-center flex">
                      <label class="w-8rem">抽出URL*</label>
                      <InputTextRegex
                        v-model="query.url_filter"
                        :disabled="loading"
                      />
                    </div>
                  </div>

                  <div class="col">
                    <div class="align-items-center flex">
                      <label class="w-8rem">ファイル名</label>
                      <InputTextRegex
                        v-model="query.filename"
                        :disabled="loading"
                        placeholder="{filename}"
                      />
                    </div>
                  </div>

                  <div class="col">
                    <div class="align-items-center flex">
                      <label class="w-8rem">永続化*</label>
                      <div class="w-full">
                        <InputSwitch v-model="query.is_persist" />
                      </div>
                    </div>
                  </div>

                  <div class="col">
                    <div class="align-items-center flex">
                      <label class="w-8rem">ｷｭｰ優先度*</label>

                      <InputNumber
                        v-model="query.priority"
                        class="w-full"
                        mode="decimal"
                        show-buttons
                        :step="10"
                      />
                    </div>
                  </div>
                </div>
              </TabPanel>
            </TabView>
          </div>

          <div class="col">
            <Button
              class="p-button-outlined w-full"
              icon="pi pi-plus"
              label="追加"
              @click="addSiteQuery"
            />
          </div>
        </div>
      </div>
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
import { Site, FormSite, useSiteAPI, ProcessorType } from '@/apis/useSiteAPI'

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

const processotTypes = ref<{ name: string, value: ProcessorType }[]>([
  { name: 'リンク抽出', value: 'extract' },
  { name: 'ダウンロード', value: 'download' },
])

const isEdit = computed(() => props.site?.id > 0)

const loading = ref<boolean>(false)
const form = reactive<FormSite>({
  key: '',
  url: '',
  title: '',
  analysis_count: 0,
  download_count: 0,
  site_queries: [],
})

watch(_show, () => onReset())

const queryTabIndex = ref<number>(0)
const queryTabNames = ref<string[]>()

const updateQuerytabNames = () => {
  queryTabNames.value = form.site_queries.map(query => query.key)
}

const addSiteQuery = () => {
  form.site_queries.push({
    key: '',
    url_pattern: '',
    processor: 'extract',
    dom_selector: '',
    url_filter: '',
    filename: '',
    is_persist: false,
    priority: 0,
  })
}

const removeSiteQuery = (id: number) => {
  form.site_queries.splice(id, 1)
  queryTabIndex.value = 0
}

/// //////////////////////////////////////////////////

const onReset = () => {
  form.key = props.site?.key ?? ''
  form.url = props.site?.url ?? ''
  form.title = props.site?.title ?? ''
  form.analysis_count = props.site?.analysis_count ?? 0
  form.download_count = props.site?.download_count ?? 0
  form.site_queries = props.site?.site_queries.map(query => ({
    id: query.id ?? 0,
    key: query.key ?? '',
    url_pattern: query.url_pattern ?? '',
    processor: query.processor ?? 'extract',
    dom_selector: query.dom_selector ?? '',
    url_filter: query.url_filter ?? '',
    filename: query.filename ?? '',
    is_persist: query.is_persist ?? false,
    priority: query.priority ?? 0,
  })) ?? []

  queryTabIndex.value = 0
  updateQuerytabNames()
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

const onClose = () => {
  _show.value = false
}
</script>
