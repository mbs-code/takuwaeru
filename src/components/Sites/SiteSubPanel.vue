<template>
  <Card>
    <template #content>
      <TabMenu
        v-model:activeIndex="tabIndex"
        class="mb-3"
        :model="tabHeaders"
      />

      <div
        ref="scrollRef"
        class="overflow-x-hidden overflow-y-scroll pr-2"
        :style="`height: ${height - 190}px`"
      >
        <SiteSheet
          :loading="loading"
          :page-count="pageCount"
          :process-logger="processLogger"
          :process-result="processResult"
          :queue-count="queueCount"
          :tab-index="tabIndex"
          @onClear="emit('onClear')"
          @onExecute="emit('onExecute')"
          @onExecuteLoop="emit('onExecute')"
          @onInterrupt="emit('onInterrupt')"
          @onReset="emit('onReset')"
          @scrollToBottom="scrollToBottom"
        />
      </div>
    </template>
  </Card>
</template>

<script setup lang="ts">
import { MenuItem } from 'primevue/menuitem'
import { useWindowSize } from 'vue-window-size'

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

const { height } = useWindowSize()

const tabIndex = ref<number>(0)
const tabHeaders = ref<MenuItem[]>([
  { label: 'コンパネ' },
  { label: 'サムネイル' },
  { label: 'ログ' },
])

const scrollRef = ref<HTMLDivElement>()
const scrollToBottom = () => {
  const ref = scrollRef.value
  ref?.scrollTo({
    top: ref.scrollHeight + 200, behavior: 'smooth'
  })
}
</script>
