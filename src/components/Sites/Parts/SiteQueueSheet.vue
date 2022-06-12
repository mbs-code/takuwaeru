<template>
  <div :style="`height: ${height}px`">
    <DataTable
      class="p-datatable-sm"
      frozen
      height
      lazy
      responsive-layout="stack"
      :scroll-height="`${height}px`"
      :scrollable="true"
      :value="queues"
    >
      <Column field="id" header="queue ID" />
      <Column field="priority" header="Priority" />
      <Column field="page.url" header="URL" />
    </DataTable>
  </div>
</template>

<script setup lang="ts">
import { Queue } from '~~/src/apis/useQueueAPI'

defineProps<{
  height: number,
}>()

const queues = inject<Queue[]>('queues')
</script>
