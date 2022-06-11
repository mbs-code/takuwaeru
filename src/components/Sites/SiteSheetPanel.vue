<template>
  <Card>
    <template #content>
      <TabMenu
        v-if="showTabs"
        v-model:activeIndex="tabIndex"
        class="mb-3"
        :model="tabHeaders"
      />

      <SiteSheet
        :height="height"
        :loading="loading"
        :page-count="pageCount"
        :process-logger="processLogger"
        :process-result="processResult"
        :queue-count="queueCount"
        :tab-index="tabIndex"
      />
    </template>
  </Card>
</template>

<script setup lang="ts">
import { MenuItem } from 'primevue/menuitem'

const props = defineProps<{
  processLogger: ReturnType<typeof useProcessLogger>,
  processResult: ReturnType<typeof useProcessResult>,
  queueCount: number,
  pageCount: number,
  loading: boolean,
  height: number,
  showTabs: boolean,
  defaultTab: number,
}>()

const tabIndex = ref<number>(props.defaultTab)
const tabHeaders = ref<MenuItem[]>([
  { label: 'コンパネ' },
  { label: 'サムネイル' },
  { label: 'ログ' },
  { label: 'キュー' },
])

const scrollRef = ref<HTMLDivElement>()
const scrollToBottom = () => {
  const ref = scrollRef.value
  ref?.scrollTo({
    top: ref.scrollHeight + 200, behavior: 'smooth'
  })
}

provide('scrollToBottom', scrollToBottom)
</script>
