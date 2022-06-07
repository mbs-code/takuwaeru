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

      <div>{{ processResult.selectedQueue }}</div>
      <div v-for="(result, key) in processResult.queryResults.value" :key="key">
        <span>{{ key }} {{ result.query.key }} {{ result.status }}</span>&nbsp;
        <span>{{ result.task }} / {{ result.maxTask }}</span>
        <div v-if="result.maxTask !== 0">
          <ProgressBar :value="perTask(result.task, result.maxTask)" />
        </div>
      </div>
    </template>
  </Card>
</template>

<script setup lang="ts">
import { Site } from '~~/src/apis/useSiteAPI'

const props = defineProps<{
  site: Site,
  walker: ReturnType<typeof useWalker>,
  processResult: ReturnType<typeof useProcessResult>,
}>()

// eslint-disable-next-line func-call-spacing
const emit = defineEmits<{
  (event: 'onEdit'): void,
  (event: 'onReset'): void,
  (event: 'onExecute'): void,
}>()

const perTask = (num: number, deno: number) => parseFloat((num / deno * 100).toFixed(1))

</script>
