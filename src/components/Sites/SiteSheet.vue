<template>
  <SiteControlSheet
    v-if="tabIndex === 0"
    :loading="loading"
    :page-count="pageCount"
    :process-result="processResult"
    :queue-count="queueCount"
  />

  <SiteImageSheet
    v-if="tabIndex === 1"
    :blob="processResult.latestBlob.value"
  />

  <SiteLogSheet
    v-if="tabIndex === 2"
    :logs="processLogger.logs.value"
  />

  <SiteQueueSheet v-if="tabIndex === 3" />
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
</script>
