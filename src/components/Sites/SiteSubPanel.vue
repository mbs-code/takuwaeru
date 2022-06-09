<template>
  <TabMenu v-model:activeIndex="tabIndex" :model="tabHeaders" />

  <Card>
    <template #content>
      <SiteImageSheet
        v-if="tabIndex === 0"
        :blob="processResult.latestBlob.value"
      />

      <SiteLogSheet
        v-if="tabIndex === 1"
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
}>()

const tabIndex = ref<number>(0)
const tabHeaders = ref<MenuItem[]>([
  { label: 'サムネイル' },
  { label: 'ログ' },
])
</script>
