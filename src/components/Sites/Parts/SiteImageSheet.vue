<template>
  <div class="surface-100" :style="`height: ${height}px`">
    <img
      v-if="props.blob && props.blob.length > 0"
      alt="Image Text"
      class="block w-full"
      :src="imageUrl"
      style="object-fit: contain"
      :style="`height: ${height}px`"
    >
    <div v-else class="align-items-center block flex h-full justify-content-center w-full">
      <div>No Image</div>
    </div>
  </div>
</template>

<script setup lang="ts">
const props = defineProps<{
  blob: number[],
  height: number,
}>()

const imageUrl = computed(() => 'data:image/jpg;base64,' +
btoa(new Uint8Array(props.blob).reduce(function (data, byte) {
  return data + String.fromCharCode(byte)
}, '')))
</script>
