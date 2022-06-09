<template>
  <Card>
    <template #content>
      <div class="flex-column grid">
        <div class="col">
          <div class="flex">
            <template v-if="!loading">
              <Button
                class="mx-2"
                :disabled="queueCount === 0"
                icon="pi pi-play"
                label="Once"
                @click="emit('onExecute')"
              />

              <Button
                class="mx-2"
                :disabled="queueCount === 0"
                icon="pi pi-forward"
                label="Run"
                @click="emit('onExecuteLoop')"
              />
            </template>

            <template v-if="loading && queue">
              <Button
                class="mx-2"
                disabled
                icon="pi pi-spin pi-spinner"
              />

              <Button
                class="mx-2 p-button-danger"
                :disabled="!loading"
                icon="pi pi-pause"
                @click="emit('onInterrupt')"
              />
            </template>

            <div class="flex-grow-1" />

            <Button
              class="mx-2 p-button-warning"
              :disabled="loading"
              label="Clear"
              @click="emit('onClear')"
            />

            <Button
              class="mx-2 p-button-danger"
              :disabled="loading"
              label="Reset"
              @click="emit('onReset')"
            />
          </div>
        </div>

        <div class="col py-0">
          <hr>
        </div>

        <div class="col">
          <div class="align-items-baseline flex">
            <div class="white-space-nowrap">
              <span>Queue</span>
              <span class="px-2 text-4xl">
                {{ queueCount.toLocaleString() }}
              </span>
            </div>

            <div class="white-space-nowrap">
              <span>Page</span>
              <span class="px-2 text-4xl">
                {{ pageCount.toLocaleString() }}
              </span>
            </div>
          </div>

          <div v-show="queueCount === 0" class="text-red-500">
            キューが空です。
          </div>
        </div>

        <div class="col">
          <div class="p-2 simple-border">
            <div
              class="border-left-3 pl-1"
              :class="queueStatusColor"
            >
              <div class="align-items-center flex m-2">
                <i class="mr-1 pi pi-file" />
                <span>{{ (queue && queue.page.url) || '----' }}</span>
                <Button
                  v-if="queue"
                  class="p-0 p-button-secondary p-button-text"
                  icon="pi pi-link"
                  @click="openBrowser(queue.page.url)"
                />
              </div>

              <div class="align-items-center flex m-2">
                <i class="mr-1 pi pi-angle-right" />
                <span>{{ (queue && queue.page.title) || '----' }}</span>
              </div>
            </div>
          </div>
        </div>

        <div class="col">
          <table class="result-table w-full">
            <template v-for="(result, key) in processResult.queryResults.value" :key="key">
              <tr>
                <td>
                  <i class="mr-1 pi" :class="queryStatusMap[result.status]" />
                </td>
                <td>{{ result.query.key }}</td>
                <td>{{ result.maxTask ? `${result.task} / ${result.maxTask}` : '-' }}</td>
              </tr>

              <tr v-if="loading && result.status === 'exec' && result.maxTask !== 0">
                <td colspan="3">
                  <ProgressBar :value="perTask(result.task, result.maxTask)" />
                </td>
              </tr>
            </template>
          </table>
        </div>
      </div>
    </template>
  </Card>
</template>

<script setup lang="ts">
import { open } from '@tauri-apps/api/shell'
import { QueryStatus } from '~~/src/composables/useProcessResult'

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
  (event: 'onExecuteLoop'): void,
  (event: 'onInterrupt'): void,
  (event: 'onClear'): void,
  (event: 'onReset'): void,
}>()

/// ////////////////////////////////////////////////////////////

const queue = computed(() => props.processResult.selectedQueue.value)
const perTask = (num: number, deno: number) => parseFloat((num / deno * 100).toFixed(1))

const queueStatusColor = computed(() => {
  if (!props.loading) {
    return (props.processResult.selectedQueue.value)
      ? 'border-green-500'
      : 'border-500'
  } else {
    return 'border-blue-500'
  }
})

const queryStatusMap: { [key in QueryStatus]: string } = {
  wait: 'pi-ellipsis-h text-500',
  exec: 'pi-play text-blue-500',
  success: 'pi-check-circle text-green-500',
  skip: 'pi-minus-circle text-yellow-500',
}

/// ////////////////////////////////////////////////////////////

const openBrowser = async (url: string) => {
  await open(url)
}
</script>

<style scoped lang="scss">
@import 'primeflex/primeflex.scss';

.result-table {
  border: solid 1px var(--surface-400);
  border-radius: .25rem;

  th,td:not(:last-child) {
    border-right: solid 1px var(--surface-400);
  }

  td {
    padding: 0.25rem 0.5rem;
  }
}

</style>
