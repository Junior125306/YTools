<script setup lang="ts">
import { ref, watch } from 'vue'
import { NModal, NInput, NSpace } from 'naive-ui'

interface Props {
  modelValue: boolean
  title?: string
  placeholder?: string
}

const props = withDefaults(defineProps<Props>(), {
  title: '输入',
  placeholder: '请输入...'
})

const emit = defineEmits<{
  (e: 'update:modelValue', value: boolean): void
  (e: 'confirm', value: string): void
  (e: 'cancel'): void
}>()

const inputValue = ref('')
const showModal = ref(false)

// 同步 modelValue 和 showModal
watch(() => props.modelValue, (newValue) => {
  showModal.value = newValue
  if (!newValue) {
    inputValue.value = ''
  }
})

watch(showModal, (newValue) => {
  emit('update:modelValue', newValue)
})

// 处理确认
const handleConfirm = () => {
  const value = inputValue.value.trim()
  if (value) {
    emit('confirm', value)
    inputValue.value = ''
    showModal.value = false
  }
}

// 处理取消
const handleCancel = () => {
  emit('cancel')
  inputValue.value = ''
  showModal.value = false
}
</script>

<template>
  <NModal
    v-model:show="showModal"
    preset="dialog"
    :title="title"
    :positive-text="'确定'"
    :negative-text="'取消'"
    @positive-click="handleConfirm"
    @negative-click="handleCancel"
  >
    <NSpace vertical>
      <NInput
        v-model:value="inputValue"
        :placeholder="placeholder"
        @keyup.enter="handleConfirm"
      />
    </NSpace>
  </NModal>
</template>

