<template>
  <Card>
    <template #content>
      <div ref="scrollRef" class="h-10rem overflow-y-scroll">
        <div
          v-for="(log, _) of items"
          :key="_"
          :class="log.color"
        >
          <span class="inline-block w-3rem">
            {{ log.dateStr }}
          </span>
          <span :class="log.color">{{ log.text }}</span>
        </div>
      </div>
    </template>
  </Card>
</template>

<script setup lang="ts">
import { format } from 'date-fns'
import { ProcessLog, ProcessLogType } from '~~/src/composables/useProcessLogger'

const props = defineProps<{
  logs: ProcessLog[],
}>()

const colorMap: { [key in ProcessLogType]: string } = {
  info: 'text-blue-500',
  debug: 'text-500',
  error: 'text-red-600',
  event: 'text-green-500',
}

const scrollRef = ref<HTMLDivElement>()

watch(props.logs, () => {
  const ref = scrollRef.value
  if (ref) {
    ref.scrollTo({ top: ref.scrollHeight + 100, behavior: 'smooth' })
  }
})

const items = computed(() => props.logs.map((log) => {
  return {
    ...log,
    dateStr: format(log.date, 'HH:mm'),
    color: colorMap[log.type],
  }
}))
</script>
