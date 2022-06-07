<template>
  <Card>
    <template #content>
      <div>{{ walker.selectedQueue.value }}</div>
      <div v-for="(item, _) of items" :key="_">
        <div>
          <span>{{ _ }}</span>
          <span>{{ item.key }}</span>
          <span>{{ item.processor }}</span>
          <span>{{ item.result ? '○' : '→' }}</span>
        </div>
      </div>

      <ProgressBar :value="perTask" />
    </template>
  </Card>
</template>

<script setup lang="ts">
import { Site } from '~~/src/apis/useSiteAPI'

const props = defineProps<{
  site: Site,
  walker: ReturnType<typeof useWalker>,
}>()

const items = computed(() => {
  const queries = props.site?.site_queries || []
  const nowIndex = queries.findIndex(query => query.id === props.walker.execQuery.value.id)
  return queries.map((query, index) => {
    return {
      key: query.key,
      processor: query.processor,
      result: index < nowIndex,
    }
  })
})

const perTask = computed(() =>
  parseFloat((props.walker.nowTask.value / props.walker.maxTask.value * 100).toFixed(1))
)
</script>
