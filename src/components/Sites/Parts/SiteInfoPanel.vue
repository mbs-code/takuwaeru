<template>
  <Card v-if="site">
    <template #content>
      <div class="align-items-center grid">
        <div class="col flex-grow-1">
          <Tag class="bg-bluegray-500 m-1" icon="pi pi-tag" :value="site.key" />

          <div class="align-items-center flex">
            <i class="pi pi-angle-right pr-2 text-2xl" />
            <span class="text-2xl">{{ site.title + 'asdasdasdasdasd' }}</span>
          </div>

          <div class="ml-5">
            {{ site.url }}
          </div>
        </div>

        <div class="col flex-grow-0 h-full">
          <div class="flex-column justify-content-between">
            <Button
              class="m-1 p-button-success"
              :disabled="loading"
              icon="pi pi-pencil"
              @click="emit('onEdit')"
            />

            <Button
              class="m-1 p-button-secondary"
              :disabled="loading"
              icon="pi pi-link"
              @click="openBrowser(site.url)"
            />
          </div>
        </div>
      </div>
    </template>
  </Card>
</template>

<script setup lang="ts">
import { open } from '@tauri-apps/api/shell'
import { Site } from '@/apis/useSiteAPI'

defineProps<{
  site: Site,
  loading: boolean,
}>()

// eslint-disable-next-line func-call-spacing
const emit = defineEmits<{
  (event: 'onEdit'): void,
}>()

const openBrowser = async (url: string) => {
  await open(url)
}
</script>
