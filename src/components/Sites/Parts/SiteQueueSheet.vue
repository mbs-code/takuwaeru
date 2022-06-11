<template>
  <DataTable
    class="p-datatable-sm"
    frozen
    lazy
    responsive-layout="stack"
    :scrollable="false"
    :value="queues"
  >
    <Column field="id" header="queue ID" />
    <Column field="priority" header="Priority" />
    <Column field="page.url" header="URL" />
    <Column field="quantity" header="Quantity" />
  </DataTable>
</template>

<script setup lang="ts">
import { Queue } from '~~/src/apis/useQueueAPI'

const queues = inject<Queue[]>('queues')
</script>
