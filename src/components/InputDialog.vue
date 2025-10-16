<template>
  <Teleport to="body">
    <Transition name="dialog-fade">
      <div v-if="modelValue" class="dialog-overlay" @click="handleOverlayClick">
        <div class="dialog-container" @click.stop>
          <div class="dialog-header">
            <h3 class="dialog-title">{{ title }}</h3>
            <button class="dialog-close" @click="handleCancel" title="关闭 (ESC)">
              ×
            </button>
          </div>
          <div class="dialog-body">
            <input
              ref="inputRef"
              v-model="inputValue"
              type="text"
              :placeholder="placeholder"
              class="dialog-input"
              @keyup.enter="handleConfirm"
              @keyup.esc="handleCancel"
            />
            <p v-if="errorMessage" class="error-message">{{ errorMessage }}</p>
          </div>
          <div class="dialog-footer">
            <button class="dialog-button dialog-button-cancel" @click="handleCancel">
              取消
            </button>
            <button class="dialog-button dialog-button-confirm" @click="handleConfirm">
              确定
            </button>
          </div>
        </div>
      </div>
    </Transition>
  </Teleport>
</template>

<script setup lang="ts">
import { ref, watch, nextTick } from 'vue';

interface Props {
  modelValue: boolean;
  title?: string;
  placeholder?: string;
  errorMessage?: string;
}

interface Emits {
  (e: 'update:modelValue', value: boolean): void;
  (e: 'confirm', value: string): void;
  (e: 'cancel'): void;
}

const props = withDefaults(defineProps<Props>(), {
  title: '输入',
  placeholder: '请输入...',
  errorMessage: ''
});

const emit = defineEmits<Emits>();

const inputRef = ref<HTMLInputElement>();
const inputValue = ref('');

// 处理确认
const handleConfirm = () => {
  const value = inputValue.value.trim();
  if (value) {
    emit('confirm', value);
    inputValue.value = '';
  }
};

// 处理取消
const handleCancel = () => {
  emit('cancel');
  emit('update:modelValue', false);
  inputValue.value = '';
};

// 处理遮罩层点击
const handleOverlayClick = () => {
  handleCancel();
};

// 监听对话框打开，自动聚焦输入框
watch(() => props.modelValue, async (newValue) => {
  if (newValue) {
    await nextTick();
    inputRef.value?.focus();
  }
});

// 监听键盘事件
const handleKeyDown = (e: KeyboardEvent) => {
  if (props.modelValue && e.key === 'Escape') {
    e.preventDefault();
    handleCancel();
  }
};

// 添加全局键盘监听
watch(() => props.modelValue, (newValue) => {
  if (newValue) {
    document.addEventListener('keydown', handleKeyDown);
  } else {
    document.removeEventListener('keydown', handleKeyDown);
  }
}, { immediate: true });
</script>

<style scoped>
.dialog-overlay {
  position: fixed;
  top: 0;
  left: 0;
  right: 0;
  bottom: 0;
  background: rgba(0, 0, 0, 0.6);
  backdrop-filter: blur(4px);
  display: flex;
  align-items: center;
  justify-content: center;
  z-index: 9999;
  padding: 20px;
}

.dialog-container {
  background: var(--color-surface);
  border: 1px solid var(--color-border);
  border-radius: 12px;
  box-shadow: 0 8px 32px rgba(0, 0, 0, 0.4);
  width: 100%;
  max-width: 400px;
  overflow: hidden;
  animation: dialog-slide-in 0.2s ease-out;
}

@keyframes dialog-slide-in {
  from {
    opacity: 0;
    transform: translateY(-20px) scale(0.95);
  }
  to {
    opacity: 1;
    transform: translateY(0) scale(1);
  }
}

.dialog-header {
  display: flex;
  align-items: center;
  justify-content: space-between;
  padding: 16px 20px;
  border-bottom: 1px solid var(--color-border);
  background: var(--color-surface-2);
}

.dialog-title {
  margin: 0;
  font-size: 16px;
  font-weight: 600;
  color: var(--color-text);
}

.dialog-close {
  background: none;
  border: none;
  color: var(--color-text-muted);
  font-size: 28px;
  line-height: 1;
  cursor: pointer;
  padding: 0;
  width: 28px;
  height: 28px;
  display: flex;
  align-items: center;
  justify-content: center;
  border-radius: 4px;
  transition: all 0.2s ease;
}

.dialog-close:hover {
  background: var(--color-surface);
  color: var(--color-text);
}

.dialog-body {
  padding: 20px;
}

.dialog-input {
  width: 100%;
  padding: 10px 12px;
  border: 1px solid var(--color-border);
  border-radius: 6px;
  background: var(--color-bg);
  color: var(--color-text);
  font-size: 14px;
  outline: none;
  transition: border-color 0.2s ease, box-shadow 0.2s ease;
  box-sizing: border-box;
}

.dialog-input:focus {
  border-color: var(--color-accent);
  box-shadow: 0 0 0 3px rgba(129, 140, 248, 0.1);
}

.dialog-input::placeholder {
  color: var(--color-text-muted);
  opacity: 0.6;
}

.error-message {
  margin: 8px 0 0 0;
  font-size: 13px;
  color: #ef4444;
}

.dialog-footer {
  display: flex;
  gap: 12px;
  padding: 16px 20px;
  border-top: 1px solid var(--color-border);
  background: var(--color-surface-2);
  justify-content: flex-end;
}

.dialog-button {
  padding: 8px 20px;
  border-radius: 6px;
  font-size: 14px;
  font-weight: 500;
  cursor: pointer;
  transition: all 0.2s ease;
  border: none;
  outline: none;
}

.dialog-button-cancel {
  background: var(--color-surface);
  color: var(--color-text-muted);
  border: 1px solid var(--color-border);
}

.dialog-button-cancel:hover {
  background: var(--color-bg);
  color: var(--color-text);
  border-color: var(--color-text-muted);
}

.dialog-button-confirm {
  background: var(--color-accent);
  color: white;
}

.dialog-button-confirm:hover {
  background: var(--color-accent-hover);
  transform: translateY(-1px);
  box-shadow: 0 4px 12px rgba(129, 140, 248, 0.3);
}

.dialog-button-confirm:active {
  background: var(--color-accent-press);
  transform: translateY(0);
}

/* 过渡动画 */
.dialog-fade-enter-active,
.dialog-fade-leave-active {
  transition: opacity 0.2s ease;
}

.dialog-fade-enter-from,
.dialog-fade-leave-to {
  opacity: 0;
}

.dialog-fade-enter-active .dialog-container {
  animation: dialog-slide-in 0.2s ease-out;
}

.dialog-fade-leave-active .dialog-container {
  animation: dialog-slide-out 0.2s ease-in;
}

@keyframes dialog-slide-out {
  from {
    opacity: 1;
    transform: translateY(0) scale(1);
  }
  to {
    opacity: 0;
    transform: translateY(-20px) scale(0.95);
  }
}

/* 响应式设计 */
@media (max-width: 768px) {
  .dialog-container {
    max-width: 90%;
  }

  .dialog-header,
  .dialog-body,
  .dialog-footer {
    padding: 14px 16px;
  }

  .dialog-title {
    font-size: 15px;
  }

  .dialog-input {
    font-size: 16px; /* 防止 iOS Safari 自动缩放 */
  }
}
</style>

