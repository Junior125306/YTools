import { ref, computed, watchEffect, onMounted, onUnmounted } from 'vue'
import { darkTheme } from 'naive-ui'
import type { GlobalTheme, GlobalThemeOverrides } from 'naive-ui'
import { getTheme, setTheme } from '../utils/configStore'
import { getCurrentWindow } from '@tauri-apps/api/window'
import type { UnlistenFn } from '@tauri-apps/api/event'

export type ThemeMode = 'light' | 'dark' | 'cyberpunk' | 'system'

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
  if (themeMode.value === 'cyberpunk') {
    return darkTheme // cyberpunk 基于 darkTheme
  }
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

// 赛博朋克主题覆盖配置 (基于 Halcyon VSCode 主题 + 霓虹增强)
const cyberpunkThemeOverrides: GlobalThemeOverrides = {
  common: {
    // 主色调 - 青色霓虹（统一主色调）
    primaryColor: '#5ccfe6',
    primaryColorHover: '#6fdbf0',
    primaryColorPressed: '#4fb3c9',
    primaryColorSuppl: '#7ee3f5',
    
    // 信息色 - 青色（霓虹青）
    infoColor: '#5ccfe6',
    infoColorHover: '#6fdbf0',
    infoColorPressed: '#3ab8d9',
    
    // 成功色 - 绿色（霓虹绿）
    successColor: '#bae67e',
    successColorHover: '#c8f090',
    successColorPressed: '#a8d46b',
    
    // 警告色 - 金色（霓虹黄）
    warningColor: '#ffcc66',
    warningColorHover: '#ffd580',
    warningColorPressed: '#ffae57',
    
    // 错误色 - 红色（霓虹红）
    errorColor: '#ef6b73',
    errorColorHover: '#f58289',
    errorColorPressed: '#e35961',
    
    // 背景色系 - 更深的暗色增强对比
    bodyColor: '#0f1419',        // 更深的背景
    cardColor: '#171c28',        // 深蓝紫
    modalColor: '#171c28',
    popoverColor: '#1d2433',
    tableColor: '#1d2433',
    tableHeaderColor: '#242b3d',
    inputColor: '#1d2433',       // 更深的输入框背景
    codeColor: '#171c28',
    
    // 边框和分隔符 - 使用青色霓虹
    borderColor: 'rgba(92, 207, 230, 0.2)',      // 青色半透明边框
    dividerColor: 'rgba(92, 207, 230, 0.15)',    // 青色半透明分隔线
    
    // 文本色系 - 统一使用柔和的浅灰白色
    textColorBase: '#d7dce2',    // 柔和的浅灰白
    textColor1: '#d7dce2',       // 主要文字 - 浅灰白
    textColor2: '#c3cad8',       // 次要文字 - 稍暗的灰白
    textColor3: '#a2aabc',       // 三级文字 - 更暗的灰色
    textColorDisabled: '#6679a4',
    placeholderColor: '#8695b7',
    
    // 悬停和高亮 - 霓虹青色
    hoverColor: 'rgba(92, 207, 230, 0.12)',     // 青色半透明悬停
    pressedColor: 'rgba(92, 207, 230, 0.2)',
    opacityDisabled: '0.4',
    
    // 圆角
    borderRadius: '8px',
    borderRadiusSmall: '6px',
    
    // 阴影 - 强化霓虹发光效果（青色+紫色混合）
    boxShadow1: '0 2px 12px rgba(92, 207, 230, 0.15), 0 1px 4px rgba(195, 166, 255, 0.1)',
    boxShadow2: '0 4px 20px rgba(92, 207, 230, 0.2), 0 2px 8px rgba(195, 166, 255, 0.15)',
    boxShadow3: '0 8px 40px rgba(92, 207, 230, 0.25), 0 4px 16px rgba(195, 166, 255, 0.2)',
    
    // 滚动条 - 青色霓虹
    scrollbarColor: 'rgba(92, 207, 230, 0.5)',
    scrollbarColorHover: 'rgba(92, 207, 230, 0.8)',
    
    // 字体
    fontFamily: "-apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, Oxygen, Ubuntu, Cantarell, sans-serif"
  },
  Button: {
    // 主按钮 - 青色霓虹（统一主色调）
    colorPrimary: '#5ccfe6',
    colorHoverPrimary: '#6fdbf0',
    colorPressedPrimary: '#4fb3c9',
    colorFocusPrimary: '#5ccfe6',
    borderPrimary: '1px solid #5ccfe6',
    borderHoverPrimary: '1px solid #6fdbf0',
    borderPressedPrimary: '1px solid #4fb3c9',
    borderFocusPrimary: '1px solid #5ccfe6',
    textColorPrimary: '#0f1419',
    textColorHoverPrimary: '#0f1419',
    textColorPressedPrimary: '#0f1419',
    textColorFocusPrimary: '#0f1419',
    // 青色霓虹发光效果
    boxShadowFocus: '0 0 0 3px rgba(92, 207, 230, 0.3), 0 0 20px rgba(92, 207, 230, 0.6)',
    
    // 次要按钮 - 透明背景 + 青色边框
    color: 'rgba(92, 207, 230, 0.08)',
    colorHover: 'rgba(92, 207, 230, 0.15)',
    colorPressed: 'rgba(92, 207, 230, 0.22)',
    textColor: '#d7dce2',
    textColorHover: '#e6edf3',
    textColorPressed: '#c3cad8',
    border: '1px solid rgba(92, 207, 230, 0.4)',
    borderHover: '1px solid rgba(92, 207, 230, 0.6)',
    borderPressed: '1px solid rgba(92, 207, 230, 0.8)',
    
    // 危险按钮（删除/重置） - 提高红色对比度
    colorError: '#ff6b73',
    colorHoverError: '#ff8a91',
    colorPressedError: '#f55a63',
    textColorError: '#e6edf3',
    textColorHoverError: '#ffffff',
    borderError: '1px solid rgba(255, 107, 115, 0.6)',
    borderHoverError: '1px solid rgba(255, 107, 115, 1)'
  },
  Input: {
    color: '#1d2433',
    colorFocus: '#1d2433',
    border: '1px solid rgba(92, 207, 230, 0.3)',
    borderHover: '1px solid rgba(92, 207, 230, 0.5)',
    borderFocus: '1px solid #5ccfe6',
    // 聚焦时青色霓虹发光
    boxShadowFocus: '0 0 0 3px rgba(92, 207, 230, 0.2), 0 0 16px rgba(92, 207, 230, 0.4)',
    caretColor: '#ffcc66',        // 光标使用黄色霓虹
    textColor: '#e6edf3',
    placeholderColor: '#8695b7'
  },
  Select: {
    peers: {
      InternalSelection: {
        color: '#1d2433',
        colorActive: '#1d2433',
        border: '1px solid rgba(92, 207, 230, 0.3)',
        borderHover: '1px solid rgba(92, 207, 230, 0.5)',
        borderActive: '1px solid #5ccfe6',
        borderFocus: '1px solid #5ccfe6',
        // 青色霓虹发光
        boxShadowActive: '0 0 0 3px rgba(92, 207, 230, 0.2), 0 0 16px rgba(92, 207, 230, 0.3)',
        boxShadowFocus: '0 0 0 3px rgba(92, 207, 230, 0.2), 0 0 16px rgba(92, 207, 230, 0.3)',
        textColor: '#e6edf3',
        placeholderColor: '#8695b7'
      }
    }
  },
  Tabs: {
    // 标签页 - 激活时青色霓虹
    tabTextColorLine: '#a2aabc',
    tabTextColorActiveLine: '#5ccfe6',          // 激活文字使用青色
    tabTextColorHoverLine: '#d7dce2',           // 悬停文字使用浅灰白
    barColor: '#5ccfe6',                        // 激活指示条使用青色
    tabColorSegment: 'rgba(92, 207, 230, 0.08)',
    tabBorderColor: 'rgba(92, 207, 230, 0.2)',
    // 标签页阴影 - 青色霓虹发光
    boxShadow: '0 0 12px rgba(92, 207, 230, 0.3)'
  },
  Radio: {
    // 单选框 - 青色激活
    buttonColorActive: '#5ccfe6',
    buttonTextColorActive: '#0f1419',
    buttonBorderColorActive: '#5ccfe6',
    boxShadowFocus: '0 0 0 3px rgba(92, 207, 230, 0.25), 0 0 12px rgba(92, 207, 230, 0.4)',
    dotColorActive: '#ffcc66',                  // 圆点使用黄色
    
    // 未激活状态
    color: 'rgba(92, 207, 230, 0.1)',
    colorHover: 'rgba(92, 207, 230, 0.15)',
    textColor: '#e6edf3',
    border: '1px solid rgba(92, 207, 230, 0.3)',
    borderHover: '1px solid rgba(92, 207, 230, 0.5)'
  },
  Switch: {
    // 开关 - 黄色激活（与青色形成对比）
    railColorActive: '#ffcc66',
    loadingColor: '#5ccfe6',
    boxShadowFocus: '0 0 0 3px rgba(255, 204, 102, 0.25), 0 0 12px rgba(255, 204, 102, 0.5)',
    railColor: 'rgba(92, 207, 230, 0.2)'
  },
  Scrollbar: {
    // 滚动条 - 青色霓虹轨道
    color: 'rgba(92, 207, 230, 0.5)',
    colorHover: 'rgba(92, 207, 230, 0.8)'
  },
  List: {
    // 列表项 - 悬停时青色霓虹背景
    color: 'transparent',
    colorHover: 'rgba(92, 207, 230, 0.08)',
    colorModal: '#171c28',
    colorHoverModal: 'rgba(92, 207, 230, 0.12)',
    borderColor: 'rgba(92, 207, 230, 0.2)',
    borderColorModal: 'rgba(92, 207, 230, 0.2)'
  },
  Divider: {
    // 分隔线 - 柔和的暗色调
    color: 'rgba(92, 207, 230, 0.2)',
    textColor: '#d7dce2'
  }
}

// 根据当前主题返回对应的覆盖配置
const themeOverrides = computed<GlobalThemeOverrides>(() => {
  if (themeMode.value === 'cyberpunk') {
    return cyberpunkThemeOverrides
  }
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
      themeMode.value === 'cyberpunk' || 
      (themeMode.value === 'system' && systemPrefersDark.value)
    ),
    isThemeLoaded
  }
}

