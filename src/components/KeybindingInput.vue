<template>
  <div class="keybinding-input-wrapper">
    <NInput
      ref="inputRef"
      :value="displayValue"
      readonly
      :placeholder="placeholder"
      @focus="handleFocus"
      @blur="handleBlur"
      @keydown="handleKeyDown"
      @click="handleClick"
      :class="{ 'is-cyberpunk': isCyberpunk }"
      style="cursor: pointer; font-family: 'Consolas', 'Courier New', monospace;"
    />
  </div>
</template>

<script setup lang="ts">
import { ref, computed, watch } from 'vue'
import { NInput, useMessage } from 'naive-ui'
import { useTheme } from '../composables/useTheme'

interface Props {
  modelValue: string
  existingShortcuts?: string[]
  placeholder?: string
}

interface Emits {
  (e: 'update:modelValue', value: string): void
  (e: 'conflict', shortcut: string): void
}

const props = withDefaults(defineProps<Props>(), {
  existingShortcuts: () => [],
  placeholder: '请按下要设置的快捷键'
})

const emit = defineEmits<Emits>()
const message = useMessage()

const { themeMode } = useTheme()
const isCyberpunk = computed(() => themeMode.value === 'cyberpunk')

// ==== 主题颜色变量 ====
const primaryColor = computed(() => '#5ccfe6')
const primaryColorAlpha = (alpha: number) => `rgba(92, 207, 230, ${alpha})`

const inputRef = ref()
const isFocused = ref(false)
const capturingKeys = ref('')

// 监听 modelValue 变化，确保组件正确更新（解决配置重新加载时显示旧值的问题）
watch(() => props.modelValue, () => {
  // 当 modelValue 从外部改变时，清空正在捕获的键和焦点状态
  if (!isFocused.value) {
    capturingKeys.value = ''
  }
})

// 格式化快捷键显示（将 Alt+Space 转为 Alt + Space）
const formattedShortcut = computed(() => {
  if (!props.modelValue) return ''
  return props.modelValue.split('+').join(' + ')
})

// 显示值（捕获时显示正在按的键，聚焦但未按键时为空以显示placeholder，否则显示当前快捷键）
const displayValue = computed(() => {
  if (isFocused.value) {
    // 聚焦状态：如果有正在捕获的键则显示，否则为空（显示placeholder）
    return capturingKeys.value || ''
  }
  // 未聚焦状态：显示当前快捷键
  return formattedShortcut.value
})

// 将按键代码标准化（Space -> Space, KeyA -> A）
const normalizeKey = (key: string, code: string): string => {
  // 特殊键处理
  if (key === ' ') return 'Space'
  if (key === 'Escape') return 'Escape'
  if (key === 'Enter') return 'Enter'
  if (key === 'Tab') return 'Tab'
  if (key === 'Backspace') return 'Backspace'
  if (key === 'Delete') return 'Delete'
  if (key === 'Insert') return 'Insert'
  if (key === 'Home') return 'Home'
  if (key === 'End') return 'End'
  if (key === 'PageUp') return 'PageUp'
  if (key === 'PageDown') return 'PageDown'
  if (key === 'ArrowUp') return 'ArrowUp'
  if (key === 'ArrowDown') return 'ArrowDown'
  if (key === 'ArrowLeft') return 'ArrowLeft'
  if (key === 'ArrowRight') return 'ArrowRight'
  
  // F1-F12
  if (/^F\d+$/.test(key)) return key
  
  // 数字键
  if (/^\d$/.test(key)) return key
  
  // 字母键：从 code 中提取（code 如 "KeyA"）
  if (code.startsWith('Key')) {
    return code.substring(3) // "KeyA" -> "A"
  }
  
  // 其他情况返回大写的 key
  return key.toUpperCase()
}

// 处理按键事件
const handleKeyDown = (e: KeyboardEvent) => {
  e.preventDefault()
  e.stopPropagation()
  
  // 捕获修饰键
  const modifiers: string[] = []
  if (e.ctrlKey) modifiers.push('Ctrl')
  if (e.altKey) modifiers.push('Alt')
  if (e.shiftKey) modifiers.push('Shift')
  if (e.metaKey) modifiers.push('Meta')
  
  // 过滤单独的修饰键
  const key = e.key
  if (['Control', 'Alt', 'Shift', 'Meta'].includes(key)) {
    // 只显示修饰键，不保存
    capturingKeys.value = modifiers.join(' + ')
    return
  }
  
  // 标准化主键
  const mainKey = normalizeKey(key, e.code)
  
  // 构建快捷键字符串
  const shortcutParts = [...modifiers, mainKey]
  const shortcut = shortcutParts.join('+')
  const displayShortcut = shortcutParts.join(' + ')
  
  // 实时显示正在按下的组合键
  capturingKeys.value = displayShortcut
  
  // 检查冲突（排除自己）
  if (props.existingShortcuts.includes(shortcut) && shortcut !== props.modelValue) {
    message.warning(`快捷键 ${displayShortcut} 已被占用`)
    emit('conflict', shortcut)
    // 延迟清空以显示冲突的快捷键
    setTimeout(() => {
      capturingKeys.value = ''
    }, 1000)
    return
  }
  
  // 至少需要一个修饰键（避免单键快捷键冲突）
  if (modifiers.length === 0) {
    message.warning('请使用修饰键组合（如 Ctrl、Alt 等）')
    setTimeout(() => {
      capturingKeys.value = ''
    }, 1000)
    return
  }
  
  // 延迟一点再更新并失焦，让用户看到捕获的键
  setTimeout(() => {
    emit('update:modelValue', shortcut)
    if (inputRef.value) {
      inputRef.value.blur()
    }
    capturingKeys.value = ''
  }, 200)
}

const handleFocus = () => {
  isFocused.value = true
  capturingKeys.value = ''
}

const handleBlur = () => {
  isFocused.value = false
  capturingKeys.value = ''
}

const handleClick = () => {
  // NInput 的 focus 方法
  if (inputRef.value) {
    inputRef.value.focus()
  }
}
</script>

<style scoped>
.keybinding-input-wrapper {
  display: inline-block;
  width: 200px;
}

/* 等宽字体显示 */
.keybinding-input-wrapper :deep(.n-input__input-el) {
  font-weight: 500;
  letter-spacing: 0.5px;
}

/* 赛博朋克主题下的 NInput 特殊样式 */
.keybinding-input-wrapper :deep(.n-input.is-cyberpunk .n-input__input-el) {
  color: v-bind('primaryColor');
  font-weight: 500;
}

.keybinding-input-wrapper :deep(.n-input.is-cyberpunk.n-input--focus) {
  box-shadow: 0 0 0 3px v-bind('primaryColorAlpha(0.2)'), 0 0 16px v-bind('primaryColorAlpha(0.4)');
}
</style>

