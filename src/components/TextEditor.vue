<template>
  <div class="text-editor-container">
    <textarea
      ref="textareaRef"
      v-model="localValue"
      :placeholder="placeholder"
      :style="{ height: height }"
      class="text-editor"
      @input="handleInput"
    ></textarea>
  </div>
</template>

<script setup lang="ts">
import { ref, watch, onMounted } from 'vue';

interface Props {
  modelValue: string;
  placeholder?: string;
  height?: string;
}

interface Emits {
  (e: 'update:modelValue', value: string): void;
  (e: 'change', value: string): void;
}

const props = withDefaults(defineProps<Props>(), {
  placeholder: '开始输入你的笔记...',
  height: '100%'
});

const emit = defineEmits<Emits>();

const textareaRef = ref<HTMLTextAreaElement>();
const localValue = ref(props.modelValue);

// 处理输入事件
const handleInput = () => {
  emit('update:modelValue', localValue.value);
  emit('change', localValue.value);
};

// 监听外部值变化
watch(() => props.modelValue, (newValue) => {
  if (localValue.value !== newValue) {
    localValue.value = newValue;
  }
});

// 获取当前值
const getValue = () => {
  return localValue.value;
};

// 设置值
const setValue = (value: string) => {
  localValue.value = value;
  emit('update:modelValue', value);
};

// 插入内容
const insertValue = (value: string) => {
  if (textareaRef.value) {
    const start = textareaRef.value.selectionStart;
    const end = textareaRef.value.selectionEnd;
    const text = localValue.value;
    localValue.value = text.substring(0, start) + value + text.substring(end);
    emit('update:modelValue', localValue.value);
    
    // 设置光标位置
    setTimeout(() => {
      if (textareaRef.value) {
        const newPos = start + value.length;
        textareaRef.value.selectionStart = newPos;
        textareaRef.value.selectionEnd = newPos;
        textareaRef.value.focus();
      }
    }, 0);
  }
};

// 聚焦编辑器
const focus = () => {
  textareaRef.value?.focus();
};

// 获取选中内容
const getSelection = () => {
  if (textareaRef.value) {
    const start = textareaRef.value.selectionStart;
    const end = textareaRef.value.selectionEnd;
    return localValue.value.substring(start, end);
  }
  return '';
};

// 导出方法
defineExpose({
  getValue,
  setValue,
  insertValue,
  focus,
  getSelection
});

onMounted(() => {
  localValue.value = props.modelValue;
});
</script>

<style scoped>
.text-editor-container {
  height: 100%;
  width: 100%;
  position: relative;
  display: flex;
  flex-direction: column;
}

.text-editor {
  width: 100%;
  height: 100%;
  padding: 16px;
  border: 1px solid var(--color-border);
  border-radius: 8px;
  background: var(--color-surface);
  color: var(--color-text);
  font-family: -apple-system, BlinkMacSystemFont, 'Segoe UI', 'PingFang SC', 'Hiragino Sans GB', 'Microsoft YaHei', 'Helvetica Neue', Helvetica, Arial, sans-serif;
  font-size: 14px;
  line-height: 1.6;
  resize: none;
  outline: none;
  box-sizing: border-box;
  transition: border-color 0.2s ease, box-shadow 0.2s ease;
}

.text-editor:focus {
  border-color: var(--color-accent);
  box-shadow: 0 0 0 2px rgba(129, 140, 248, 0.1);
}

.text-editor::placeholder {
  color: var(--color-text-muted);
  opacity: 0.6;
}

/* 滚动条样式 */
.text-editor::-webkit-scrollbar {
  width: 8px;
  height: 8px;
}

.text-editor::-webkit-scrollbar-track {
  background: var(--color-surface-2);
  border-radius: 4px;
}

.text-editor::-webkit-scrollbar-thumb {
  background: var(--color-border);
  border-radius: 4px;
  transition: background 0.2s ease;
}

.text-editor::-webkit-scrollbar-thumb:hover {
  background: var(--color-text-muted);
}

/* 响应式设计 */
@media (max-width: 768px) {
  .text-editor {
    padding: 12px;
    font-size: 13px;
  }
}
</style>

