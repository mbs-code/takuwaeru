<template>
  <div
    ref="scrollRef"
    class="overflow-x-hidden overflow-y-scroll pr-2"
    :style="`height: ${height}px`"
  >
    <div
      v-for="(log, _) of items"
      :key="_"
      :class="log.color"
    >
      <span class="inline-block w-5rem">
        {{ log.dateStr }}
      </span>
      <span :class="log.color">{{ log.text }}</span>
    </div>
  </div>
</template>

<script setup lang="ts">
import { format } from 'date-fns'
import { ProcessLog, ProcessLogType } from '~~/src/composables/useProcessLogger'

const props = defineProps<{
  logs: ProcessLog[],
  height: number,
}>()

///

const colorMap: { [key in ProcessLogType]: string } = {
  info: 'text-blue-500',
  debug: 'text-500',
  error: 'text-red-600',
  event: 'text-green-500',
}

const scrollRef = ref<HTMLDivElement>()
watch(props.logs, () => {
  const ref = scrollRef.value
  ref?.scrollTo({
    top: ref.scrollHeight + 200, behavior: 'smooth'
  })
})

const items = computed(() => props.logs.map((log) => {
  return {
    ...log,
    dateStr: format(log.date, 'HH:mm:ss'),
    color: colorMap[log.type],
  }
}))
</script>
