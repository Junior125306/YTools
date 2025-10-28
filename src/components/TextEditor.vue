<template>
  <div class="text-editor-container">
    <!-- 保存状态指示器 -->
    <transition name="fade">
      <div v-if="showSaveStatus" class="save-status-indicator">
        <NSpin v-if="saveStatus === 'saving'" :size="18" />
        <NIcon v-else-if="saveStatus === 'saved'" size="20" color="#52c41a">
          <CheckmarkCircleOutline />
        </NIcon>
        <NIcon v-else-if="saveStatus === 'failed'" size="20" color="#f5222d">
          <CloseCircleOutline />
        </NIcon>
      </div>
    </transition>
    
    <textarea
      ref="textareaRef"
      v-model="localValue"
      :placeholder="placeholder"
      :style="{
        height: height,
        fontSize: fontSize + 'px',
        fontFamily: fontFamily,
        lineHeight: lineHeight
      }"
      class="text-editor"
      @input="handleInput"
      @wheel="handleWheel"
    ></textarea>
  </div>
</template>

<script setup lang="ts">
import { ref, watch, onMounted } from 'vue';
import { NIcon, NSpin } from 'naive-ui';
import { CheckmarkCircleOutline, CloseCircleOutline } from '@vicons/ionicons5';

interface Props {
  modelValue: string;
  placeholder?: string;
  height?: string;
  fontSize?: number;
  fontFamily?: string;
  lineHeight?: number;
  saveStatus?: string;
}

interface Emits {
  (e: 'update:modelValue', value: string): void;
  (e: 'change', value: string): void;
  (e: 'update:fontSize', value: number): void;
}

const props = withDefaults(defineProps<Props>(), {
  placeholder: '开始输入你的笔记...',
  height: '100%',
  fontSize: 16,
  fontFamily: "Consolas, 'Courier New', monospace",
  lineHeight: 1.6,
  saveStatus: ''
});

const emit = defineEmits<Emits>();

const textareaRef = ref<HTMLTextAreaElement>();
const localValue = ref(props.modelValue);
const fontSize = ref(props.fontSize);
const fontFamily = ref(props.fontFamily);
const lineHeight = ref(props.lineHeight);
const showSaveStatus = ref(false);
let hideTimer: number | null = null;

// 监听保存状态变化
watch(() => props.saveStatus, (newStatus) => {
  if (newStatus) {
    showSaveStatus.value = true;
    
    // 清除之前的定时器
    if (hideTimer) {
      clearTimeout(hideTimer);
      hideTimer = null;
    }
    
    // 只有成功状态才自动隐藏
    if (newStatus === 'saved') {
      hideTimer = window.setTimeout(() => {
        showSaveStatus.value = false;
        hideTimer = null;
      }, 450);  // 450ms 后隐藏
    }
  } else {
    showSaveStatus.value = false;
  }
});

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

// 监听外部字体大小变化
watch(() => props.fontSize, (newSize) => {
  if (fontSize.value !== newSize) {
    fontSize.value = newSize;
  }
});

// 监听外部字体族变化
watch(() => props.fontFamily, (newFamily) => {
  if (fontFamily.value !== newFamily) {
    fontFamily.value = newFamily;
  }
});

// 监听外部行高变化
watch(() => props.lineHeight, (newHeight) => {
  if (lineHeight.value !== newHeight) {
    lineHeight.value = newHeight;
  }
});

// 处理滚轮事件 (Ctrl + 滚轮调整字体大小)
const handleWheel = (e: WheelEvent) => {
  if (e.ctrlKey) {
    e.preventDefault();
    const delta = e.deltaY > 0 ? -1 : 1;
    const newSize = Math.max(12, Math.min(32, fontSize.value + delta));
    if (newSize !== fontSize.value) {
      fontSize.value = newSize;
      emit('update:fontSize', newSize);
    }
  }
};

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
  fontSize.value = props.fontSize;
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

/* 保存状态指示器 */
.save-status-indicator {
  position: absolute;
  top: 1rem;
  right: 1rem;
  z-index: 10;
  display: flex;
  align-items: center;
  justify-content: center;
  pointer-events: none;
}

/* 淡入淡出动画 */
.fade-enter-active,
.fade-leave-active {
  transition: opacity 0.3s ease;
}

.fade-enter-from,
.fade-leave-to {
  opacity: 0;
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
  /* font-size is now controlled by inline style */
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
    /* font-size is now controlled by inline style */
  }
}
</style>

