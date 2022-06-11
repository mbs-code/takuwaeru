<template>
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
    @scrollToBottom="emit('scrollToBottom')"
  />
</template>

<script setup lang="ts">
defineProps<{
  processLogger: ReturnType<typeof useProcessLogger>,
  processResult: ReturnType<typeof useProcessResult>,
  queueCount: number,
  pageCount: number,
  loading: boolean,
  tabIndex: number,
}>()

// eslint-disable-next-line func-call-spacing
const emit = defineEmits<{
  (event: 'onExecute'): void,
  (event: 'onExecuteLoop'): void,
  (event: 'onInterrupt'): void,
  (event: 'onClear'): void,
  (event: 'onReset'): void,
  (event: 'scrollToBottom'): void,
}>()
</script>
