<template>
  <Card v-if="site">
    <template #content>
      <Tag class="m-2" icon="pi pi-tag" :value="site.key" />

      <div class="align-items-center flex">
        <i class="pi pi-angle-right pr-2 text-2xl" />
        <span class="text-2xl">{{ site.title }}</span>
      </div>

      <div class="align-items-center flex">
        <span>{{ site.url }}</span>
        <Button
          class="p-button-secondary p-button-text"
          icon="pi pi-link"
          @click="openBrowser(site.url)"
        />
      </div>
    </template>
  </Card>
</template>

<script setup lang="ts">
import { open } from '@tauri-apps/api/shell'
import { Site } from '@/apis/useSiteAPI'

defineProps<{
  site: Site,
}>()

const openBrowser = async (url: string) => {
  await open(url)
}
</script>
