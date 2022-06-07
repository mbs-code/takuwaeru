<template>
  <Card>
    <template #content>
      <div>
        <Button
          class="m-1 p-button-success"
          label="Edit"
          @click="emit('onEdit')"
        />

        <Button
          class="m-1 p-button-danger"
          label="Reset"
          @click="emit('onReset')"
        />
        <Button
          class="m-1"
          label="Execute"
          @click="emit('onExecute')"
        />
      </div>

      <div>残りキュー：{{ queueCount }}</div>

      <div>処理中：</div>
      <div v-if="queue">
        <div>{{ queue.page.url }}</div>
        <div>{{ queue.page.title ?? '-' }}</div>
      </div>

      <div v-for="(result, key) in processResult.queryResults.value" :key="key">
        <span>{{ key }} {{ result.query.key }} {{ result.status }}</span>&nbsp;
        <span>{{ result.task }} / {{ result.maxTask }}</span>

        <div v-if="result.status === 'exec' && result.maxTask !== 0">
          <ProgressBar :value="perTask(result.task, result.maxTask)" />
        </div>
      </div>
    </template>
  </Card>
</template>

<script setup lang="ts">
const props = defineProps<{
  processResult: ReturnType<typeof useProcessResult>,
  queueCount: number,
}>()

// eslint-disable-next-line func-call-spacing
const emit = defineEmits<{
  (event: 'onEdit'): void,
  (event: 'onReset'): void,
  (event: 'onExecute'): void,
}>()

const queue = computed(() => props.processResult.selectedQueue.value)

const perTask = (num: number, deno: number) => parseFloat((num / deno * 100).toFixed(1))

</script>
