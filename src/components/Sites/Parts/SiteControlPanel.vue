<template>
  <Card>
    <template #content>
      <div class="grid">
        <Button
          class="m-1"
          :disabled="loading || queueCount === 0"
          :icon="loading ? 'pi pi-spin pi-spinner' : ''"
          :label="loading ? '' : 'Execute'"
          @click="emit('onExecute')"
        />

        <Button
          v-if="loading"
          class="m-1  p-button-danger  p-button-outlined"
          :disabled="!loading"
          label="Interrupt"
          @click="emit('onInterrupt')"
        />

        <div class="flex-grow-1" />

        <Button
          class="m-1 p-button-success"
          :disabled="loading"
          label="Edit"
          @click="emit('onEdit')"
        />

        <Button
          class="m-1 p-button-warning"
          :disabled="loading"
          label="Clear"
          @click="emit('onClear')"
        />

        <Button
          class="m-1 p-button-danger"
          :disabled="loading"
          label="Reset"
          @click="emit('onReset')"
        />
      </div>

      <div v-show="queueCount === 0" class="text-red-500">
        キューが空です。
      </div>

      <div>残りキュー：{{ queueCount }}</div>
      <div>処理ページ：{{ pageCount }}</div>

      <div>処理中：</div>
      <div v-if="queue">
        <div class="align-items-center flex">
          <span>{{ queue.page.url }}</span>
          <Button
            class="p-0 p-button-secondary p-button-text"
            icon="pi pi-link"
            @click="openBrowser(queue.page.url)"
          />
        </div>
        <div>{{ queue.page.title || '-' }}</div>
      </div>

      <div v-for="(result, key) in processResult.queryResults.value" :key="key">
        <span>{{ key }} {{ result.query.key }} {{ result.status }}</span>&nbsp;
        <span>{{ result.task }} / {{ result.maxTask }}</span>

        <div v-if="result.status === 'exec' && result.maxTask !== 0">
          <ProgressBar :value="perTask(result.task, result.maxTask)" />
        </div>
      </div>
    </template>
  </Card>
</template>

<script setup lang="ts">
import { open } from '@tauri-apps/api/shell'

const props = defineProps<{
  processResult: ReturnType<typeof useProcessResult>,
  queueCount: number,
  pageCount: number,
  loading: boolean,
}>()

// eslint-disable-next-line func-call-spacing
const emit = defineEmits<{
  (event: 'onEdit'): void,
  (event: 'onExecute'): void,
  (event: 'onInterrupt'): void,
  (event: 'onClear'): void,
  (event: 'onReset'): void,
}>()

const queue = computed(() => props.processResult.selectedQueue.value)
const perTask = (num: number, deno: number) => parseFloat((num / deno * 100).toFixed(1))

///

const openBrowser = async (url: string) => {
  await open(url)
}
</script>
