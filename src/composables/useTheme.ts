import { ref, computed, watchEffect, onMounted, onUnmounted } from 'vue'
import { darkTheme } from 'naive-ui'
import type { GlobalTheme, GlobalThemeOverrides } from 'naive-ui'
import { getTheme, setTheme } from '../utils/configStore'
import { getCurrentWindow } from '@tauri-apps/api/window'
import type { UnlistenFn } from '@tauri-apps/api/event'

export type ThemeMode = 'light' | 'dark' | 'system'

const themeMode = ref<ThemeMode>('system')
const systemPrefersDark = ref(false)
const isThemeLoaded = ref(false)

// 监听系统主题变化
if (typeof window !== 'undefined') {
  const mediaQuery = window.matchMedia('(prefers-color-scheme: dark)')
  systemPrefersDark.value = mediaQuery.matches
  
  mediaQuery.addEventListener('change', (e) => {
    systemPrefersDark.value = e.matches
  })
}

// 计算实际应用的主题
const effectiveTheme = computed<GlobalTheme | null>(() => {
  const isDark = themeMode.value === 'dark' || 
    (themeMode.value === 'system' && systemPrefersDark.value)
  return isDark ? darkTheme : null // null = light theme
})

// 亮色主题覆盖配置
const lightThemeOverrides: GlobalThemeOverrides = {
  common: {
    primaryColor: '#a78bfa',
    primaryColorHover: '#8b5cf6',
    primaryColorPressed: '#7c3aed',
    primaryColorSuppl: '#c4b5fd',
    borderRadius: '8px',
    fontFamily: "-apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, Oxygen, Ubuntu, Cantarell, sans-serif",
    // 亮色主题的柔和背景色
    bodyColor: '#f5f5f4',
    cardColor: '#fafaf9',
    modalColor: '#fafaf9',
    popoverColor: '#fafaf9',
    tableHeaderColor: '#e7e5e4',
    hoverColor: '#e7e5e4',
    inputColor: '#fafaf9',
    tableColor: '#fafaf9'
  }
}

// 暗色主题覆盖配置
const darkThemeOverrides: GlobalThemeOverrides = {
  common: {
    primaryColor: '#a78bfa',
    primaryColorHover: '#8b5cf6',
    primaryColorPressed: '#7c3aed',
    primaryColorSuppl: '#c4b5fd',
    borderRadius: '8px',
    fontFamily: "-apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, Oxygen, Ubuntu, Cantarell, sans-serif"
    // 暗色主题使用 Naive UI 默认的暗色背景
  }
}

// 根据当前主题返回对应的覆盖配置
const themeOverrides = computed<GlobalThemeOverrides>(() => {
  const isDark = themeMode.value === 'dark' || 
    (themeMode.value === 'system' && systemPrefersDark.value)
  return isDark ? darkThemeOverrides : lightThemeOverrides
})

// 立即加载主题配置（模块级别，只执行一次）
let themeInitPromise: Promise<void> | null = null

const initTheme = async () => {
  if (!themeInitPromise) {
    themeInitPromise = (async () => {
      try {
        const savedTheme = await getTheme()
        if (savedTheme) {
          themeMode.value = savedTheme as ThemeMode
        }
        isThemeLoaded.value = true
      } catch (error) {
        console.error('加载主题配置失败:', error)
        isThemeLoaded.value = true
      }
    })()
  }
  return themeInitPromise
}

// 立即开始加载主题
if (typeof window !== 'undefined') {
  initTheme()
}

export function useTheme() {
  let unlistenThemeChange: UnlistenFn | null = null

  // 监听主题变化并持久化
  watchEffect(() => {
    if (isThemeLoaded.value) {
      setTheme(themeMode.value).catch((error) => {
        console.error('保存主题配置失败:', error)
      })
    }
  })

  // 监听来自其他窗口的主题变更事件
  onMounted(async () => {
    try {
      const currentWindowInstance = getCurrentWindow()
      unlistenThemeChange = await currentWindowInstance.listen<ThemeMode>('theme-changed', (event) => {
        console.log('[useTheme] 收到主题变更事件:', event.payload)
        themeMode.value = event.payload
      })
    } catch (error) {
      console.error('监听主题变更事件失败:', error)
    }
  })

  onUnmounted(() => {
    if (unlistenThemeChange) {
      unlistenThemeChange()
    }
  })

  // 修改主题（会通知所有窗口）
  const changeTheme = async (mode: ThemeMode) => {
    themeMode.value = mode
    
    // 广播主题变更事件到所有窗口
    try {
      const { WebviewWindow } = await import('@tauri-apps/api/webviewWindow')
      const allWindows = await WebviewWindow.getAll()
      
      for (const win of allWindows) {
        try {
          await win.emit('theme-changed', mode)
        } catch (error) {
          console.error(`通知窗口 ${win.label} 失败:`, error)
        }
      }
    } catch (error) {
      console.error('广播主题变更失败:', error)
    }
  }

  return {
    themeMode,
    naiveTheme: effectiveTheme,
    themeOverrides,
    changeTheme,
    isDark: computed(() => 
      themeMode.value === 'dark' || 
      (themeMode.value === 'system' && systemPrefersDark.value)
    ),
    isThemeLoaded
  }
}

