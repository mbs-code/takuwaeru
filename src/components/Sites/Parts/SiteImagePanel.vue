<template>
  <Card>
    <template #content>
      <div class="surface-100">
        <img
          v-if="props.blob"
          alt="Image Text"
          class="block h-30rem w-full"
          :src="imageUrl"
          style="object-fit: contain"
        >
        <div v-else class="align-items-center block flex h-30rem justify-content-center w-full">
          <div>No Image</div>
        </div>
      </div>
    </template>
  </Card>
</template>

<script setup lang="ts">
const props = defineProps<{
  blob: number[],
}>()

const imageUrl = computed(() => 'data:image/jpg;base64,' +
btoa(new Uint8Array(props.blob).reduce(function (data, byte) {
  return data + String.fromCharCode(byte)
}, '')))
</script>
