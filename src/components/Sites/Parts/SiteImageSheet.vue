<template>
  <div class="surface-100" :style="`height: ${height}px`">
    <img
      v-if="srcUrl"
      alt="Image Text"
      class="block w-full"
      :src="srcUrl"
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

const srcUrl = ref<string>('')

watch(() => props.blob, () => {
  if (props.blob) {
    try {
      const url = 'data:image/jpg;base64,' + btoa(new Uint8Array(props.blob).reduce(function (data, byte) {
        return data + String.fromCharCode(byte)
      }, ''))
      srcUrl.value = url
    } catch (err) {
      /** 握りつぶす */
      srcUrl.value = null
    }
  }
})
</script>
