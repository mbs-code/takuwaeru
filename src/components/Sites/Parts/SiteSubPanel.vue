<template>
  <TabMenu v-model:activeIndex="tabIndex" :model="tabHeaders" />

  <Card>
    <template #content>
      <SiteControlSheet
        v-if="tabIndex === 0"
        :loading="loading"
        :page-count="pageCount"
        :process-result="processResult"
        :queue-count="queueCount"
        @onClear="emit('onClear')"
        @onExecute="emit('onExecute')"
        @onExecuteLoop="emit('onExecute')"
        @onInterrupt="emit('onInterrupt')"
        @onReset="emit('onReset')"
      />

      <SiteImageSheet
        v-if="tabIndex === 1"
        :blob="processResult.latestBlob.value"
      />

      <SiteLogSheet
        v-if="tabIndex === 2"
        :logs="processLogger.logs.value"
      />
    </template>
  </Card>
</template>

<script setup lang="ts">
import { MenuItem } from 'primevue/menuitem'

defineProps<{
  processLogger: ReturnType<typeof useProcessLogger>,
  processResult: ReturnType<typeof useProcessResult>,
  queueCount: number,
  pageCount: number,
  loading: boolean,
}>()

// eslint-disable-next-line func-call-spacing
const emit = defineEmits<{
  (event: 'onExecute'): void,
  (event: 'onExecuteLoop'): void,
  (event: 'onInterrupt'): void,
  (event: 'onClear'): void,
  (event: 'onReset'): void,
}>()

const tabIndex = ref<number>(0)
const tabHeaders = ref<MenuItem[]>([
  { label: 'コンパネ' },
  { label: 'サムネイル' },
  { label: 'ログ' },
])
</script>
