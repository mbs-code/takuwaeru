<template>
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
</template>

<script setup lang="ts">
import { format } from 'date-fns'
import { ProcessLog, ProcessLogType } from '~~/src/composables/useProcessLogger'

const props = defineProps<{
  logs: ProcessLog[],
}>()

// eslint-disable-next-line func-call-spacing
const emit = defineEmits<{
  (event: 'scrollToBottom'): void,
}>()

///

const colorMap: { [key in ProcessLogType]: string } = {
  info: 'text-blue-500',
  debug: 'text-500',
  error: 'text-red-600',
  event: 'text-green-500',
}

watch(props.logs, () => {
  emit('scrollToBottom')
})

const items = computed(() => props.logs.map((log) => {
  return {
    ...log,
    dateStr: format(log.date, 'HH:mm:ss'),
    color: colorMap[log.type],
  }
}))
</script>
