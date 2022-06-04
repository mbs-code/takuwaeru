<template>
  <div class="p-inputgroup">
    <InputText v-model="_value" v-bind="$attrs" />
    <Button
      icon="pi pi-bolt"
      label="Regex"
      @click="openRegexDialog"
    />
  </div>

  <Dialog
    v-model:visible="showRegexDialog"
    class="org-dialog"
    :style="{ width: '480px' }"
    @blur="onBlur"
  >
    <template #header>
      <div>
        <Avatar
          class="mx-1"
          :class="{ 'info-color': true }"
          icon="pi pi-bolt"
        />
        <span class="mx-1 p-dialog-title">
          正規表現ヘルパ
        </span>
      </div>
    </template>

    <div class="flex-column grid m-0">
      <div class="col">
        <label>正規表現</label>
        <InputText
          v-model="_value"
          autofocus
          class="block w-full"
        />
      </div>

      <div class="col">
        <label>マッチング （行ごと）</label>

        <div class="flex max-h-10rem overflow-y-auto simple-border">
          <Textarea
            v-model="diffResultText"
            auto-resize
            class="border-noround surface-200 w-3rem"
            disabled
            rows="8"
          />
          <Textarea
            v-model="diffText"
            auto-resize
            class="border-noround w-full"
            rows="8"
          />
        </div>
      </div>
    </div>
  </Dialog>
</template>

<script lang="ts">
export default {
  inheritAttrs: false,
}
</script>

<script setup lang="ts">
const props = defineProps<{ modelValue: string }>()
const emit = defineEmits<{(event: 'update:modelValue', val: string): void }>()

const _value = computed({
  get: () => props.modelValue,
  set: (val: string) => emit('update:modelValue', val)
})

/// //////////////////////////////////////////////////

const showRegexDialog = ref<boolean>(false)
const openRegexDialog = () => {
  showRegexDialog.value = true
}

const onBlur = () => {
  showRegexDialog.value = false
}

const diffText = ref<string>()
const diffResultText = computed(() => {
  try {
    const regex = new RegExp(_value.value)
    return (diffText.value ?? '')
      .split('\n')
      .map(line => line ? (regex.test(line) ? '○' : '✕') : '')
      .join('\n')
  } catch (err) {
    return (diffText.value ?? '')
      .split('\n')
      .map(line => 'Err').join('\n')
  }
})

watch(showRegexDialog, () => {
  diffText.value = ''
})
</script>
