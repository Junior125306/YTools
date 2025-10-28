<script setup lang="ts">
import { ref, onMounted, onUnmounted, computed } from 'vue'
import { getCurrentWindow } from '@tauri-apps/api/window'
import { invoke } from '@tauri-apps/api/core'
import { open } from '@tauri-apps/plugin-dialog'
import { enable as enableAutostart, disable as disableAutostart } from '@tauri-apps/plugin-autostart'
import { homeDir } from '@tauri-apps/api/path'
import {
  NForm, NFormItem, NRadioGroup, NRadioButton,
  NSwitch, NInputNumber, NSelect, NList, NListItem,
  NButton, NSpace, NIcon, NText, NDivider, useThemeVars, useMessage
} from 'naive-ui'
import { AddOutline, TrashOutline, CloseOutline, SettingsOutline } from '@vicons/ionicons5'
import { useTheme } from '../composables/useTheme'

const themeVars = useThemeVars()
const message = useMessage()

import {
  getConfig,
  setFontSize,
  setFontFamily,
  setLineHeight,
  setAutoStart,
  setSearchDirectories,
  setDefaultNotesLocation,
  setTheme,
  resetConfig,
  type AppConfig
} from '../utils/configStore'

const currentWindow = getCurrentWindow()
const { changeTheme } = useTheme()

const localConfig = ref<AppConfig>({
  fontSize: 16,
  fontFamily: "Consolas, 'Courier New', monospace",
  lineHeight: 1.6,
  autoStart: false,
  searchDirectories: [],
  defaultNotesLocation: '',
  notes: [],
  theme: 'system'
})

// 加载状态
const isLoading = ref(true)

// 计算显示的路径（实际路径或提示文本）
const displayPath = computed(() => {
  return localConfig.value.defaultNotesLocation || '使用默认位置 (~/.ytools)'
})

// 获取实际要复制的路径
const getActualPath = async () => {
  if (localConfig.value.defaultNotesLocation) {
    return localConfig.value.defaultNotesLocation
  }
  // 如果没有自定义路径，使用默认路径
  const home = await homeDir()
  return `${home}.ytools`
}

// 字体族选项
const fontFamilyOptions = [
  { label: 'Consolas (等宽)', value: "Consolas, 'Courier New', monospace" },
  { label: '微软雅黑', value: "'Microsoft YaHei', sans-serif" },
  { label: '黑体', value: "'SimHei', sans-serif" },
  { label: '楷体', value: "'KaiTi', serif" },
  { label: '系统等宽字体', value: "monospace" }
]

// 行高选项
const lineHeightOptions = [
  { label: '1.4 (紧凑)', value: 1.4 },
  { label: '1.6 (标准)', value: 1.6 },
  { label: '1.8 (舒适)', value: 1.8 },
  { label: '2.0 (宽松)', value: 2.0 }
]

// 加载配置
const loadSettings = async () => {
  try {
    isLoading.value = true
    const config = await getConfig()
    localConfig.value = { 
      ...config,
      searchDirectories: Array.isArray(config.searchDirectories) ? config.searchDirectories : []
    }
  } catch (error) {
    console.error('加载设置失败:', error)
    message.error('加载设置失败')
  } finally {
    isLoading.value = false
  }
}

// 添加搜索目录
const addDirectory = async () => {
  try {
    const selected = await open({
      directory: true,
      multiple: false,
      title: '选择搜索目录'
    })
    
    if (selected && typeof selected === 'string') {
      if (!Array.isArray(localConfig.value.searchDirectories)) {
        localConfig.value.searchDirectories = []
      }
      if (!localConfig.value.searchDirectories.includes(selected)) {
        localConfig.value.searchDirectories.push(selected)
      }
    }
  } catch (error) {
    console.error('选择目录失败:', error)
    message.error('选择目录失败')
  }
}

// 删除搜索目录
const removeDirectory = (index: number) => {
  localConfig.value.searchDirectories.splice(index, 1)
}

// 选择笔记位置
const selectNotesLocation = async () => {
  try {
    const selected = await open({
      directory: true,
      multiple: false,
      title: '选择默认笔记位置'
    })
    
    if (selected && typeof selected === 'string') {
      localConfig.value.defaultNotesLocation = selected
    }
  } catch (error) {
    console.error('选择目录失败:', error)
    message.error('选择目录失败')
  }
}

// 打开路径所在目录
const handleOpenPath = async () => {
  try {
    const path = await getActualPath()
    // 调用 Rust 命令打开文件夹
    await invoke('open_directory', { path })
  } catch (error) {
    console.error('打开目录失败:', error)
    message.error('打开目录失败')
  }
}

// 保存设置
const handleSave = async () => {
  try {
    await setFontSize(localConfig.value.fontSize)
    await setFontFamily(localConfig.value.fontFamily)
    await setLineHeight(localConfig.value.lineHeight)
    await setAutoStart(localConfig.value.autoStart)
    await setSearchDirectories(localConfig.value.searchDirectories)
    await setDefaultNotesLocation(localConfig.value.defaultNotesLocation)
    await setTheme(localConfig.value.theme)

    // 应用主题切换
    changeTheme(localConfig.value.theme as 'light' | 'dark' | 'system')

    // 应用开机启动设置
    try {
      if (localConfig.value.autoStart) {
        await enableAutostart()
      } else {
        await disableAutostart()
      }
    } catch (error) {
      console.error('设置开机启动失败:', error)
    }

    // 通知主窗口重新加载配置
    try {
      const { WebviewWindow } = await import('@tauri-apps/api/webviewWindow')
      const mainWindow = await WebviewWindow.getByLabel('main')
      if (mainWindow) {
        await mainWindow.emit('settings-saved', {})
      }
    } catch (error) {
      console.error('通知主窗口失败:', error)
    }

    // 显示成功提示（不关闭窗口）
    message.success('设置已保存', { duration: 2000 })
  } catch (error) {
    console.error('保存设置失败:', error)
    message.error('保存设置失败')
  }
}

// 重置为默认值
const handleReset = async () => {
  try {
    await resetConfig()
    await loadSettings()
    message.success('已重置为默认设置', { duration: 2000 })
  } catch (error) {
    console.error('重置设置失败:', error)
    message.error('重置设置失败')
  }
}

// 关闭窗口
const closeWindow = async () => {
  await currentWindow.hide()
}

// 处理键盘事件
const handleKeydown = (e: KeyboardEvent) => {
  if (e.key === 'Escape') {
    closeWindow()
  }
}

// 初始化
onMounted(async () => {
  await loadSettings()
  document.addEventListener('keydown', handleKeydown)
})

onUnmounted(() => {
  document.removeEventListener('keydown', handleKeydown)
})
</script>

<template>
  <div class="settings-window">
    <!-- 固定头部 -->
    <div class="settings-header">
      <h2 class="settings-title">设置</h2>
      <NButton text circle @click="closeWindow" title="关闭 (ESC)" class="close-button">
        <template #icon>
          <NIcon size="20"><CloseOutline /></NIcon>
        </template>
      </NButton>
    </div>

    <!-- 可滚动内容区域 -->
    <div class="settings-content">
      <!-- 加载状态 -->
      <div v-if="isLoading" class="loading-container">
        <NSpace vertical align="center" :size="16">
          <NIcon size="40" :component="SettingsOutline" />
          <NText depth="3">加载设置中...</NText>
        </NSpace>
      </div>

      <NForm v-else label-placement="left" label-width="100">
        <!-- 主题设置 -->
        <NDivider title-placement="left">主题设置</NDivider>
        <NFormItem label="主题模式">
          <NRadioGroup v-model:value="localConfig.theme">
            <NRadioButton value="light">亮色</NRadioButton>
            <NRadioButton value="dark">暗色</NRadioButton>
            <NRadioButton value="system">跟随系统</NRadioButton>
          </NRadioGroup>
        </NFormItem>

        <!-- 通用设置 -->
        <NDivider title-placement="left">通用设置</NDivider>
        <NFormItem label="开机启动">
          <NSwitch v-model:value="localConfig.autoStart" />
        </NFormItem>

        <!-- 编辑器设置 -->
        <NDivider title-placement="left">编辑器设置</NDivider>
        
        <NFormItem label="字体大小">
          <NInputNumber 
            v-model:value="localConfig.fontSize" 
            :min="12" 
            :max="32"
            :step="2"
            style="width: 150px"
          >
            <template #suffix>px</template>
          </NInputNumber>
        </NFormItem>

        <NFormItem label="字体族">
          <NSelect 
            v-model:value="localConfig.fontFamily" 
            :options="fontFamilyOptions"
            style="width: 100%"
          />
        </NFormItem>

        <NFormItem label="行高">
          <NSelect 
            v-model:value="localConfig.lineHeight" 
            :options="lineHeightOptions"
            style="width: 150px"
          />
        </NFormItem>

        <!-- 搜索设置 -->
        <NDivider title-placement="left">搜索设置</NDivider>
        <NFormItem label="搜索目录">
          <NSpace vertical style="width: 100%">
            <NList v-if="localConfig.searchDirectories.length > 0" bordered style="max-height: 200px; overflow-y: auto">
              <NListItem v-for="(dir, index) in localConfig.searchDirectories" :key="index">
                <template #suffix>
                  <NButton text @click="removeDirectory(index)" type="error">
                    <template #icon>
                      <NIcon><TrashOutline /></NIcon>
                    </template>
                  </NButton>
                </template>
                <NText>{{ dir }}</NText>
              </NListItem>
            </NList>
            <NText v-else depth="3" style="padding: 16px; text-align: center">
              暂无搜索目录
            </NText>
            <NButton @click="addDirectory" dashed block>
              <template #icon>
                <NIcon><AddOutline /></NIcon>
              </template>
              添加目录
            </NButton>
          </NSpace>
        </NFormItem>

        <!-- 笔记设置 -->
        <NDivider title-placement="left">笔记设置</NDivider>
        <NFormItem label="默认笔记位置">
          <div class="notes-location-container">
            <NText 
              class="notes-location-path"
              :title="displayPath"
              @click="handleOpenPath"
            >
              {{ displayPath }}
            </NText>
            <NButton @click="selectNotesLocation" size="small">修改</NButton>
          </div>
        </NFormItem>
      </NForm>
    </div>

    <!-- 固定底部 -->
    <div class="settings-footer">
      <NButton @click="handleReset" type="error" secondary size="small">
        重置为默认
      </NButton>
      <NSpace :size="8">
        <NButton @click="closeWindow" size="small">取消</NButton>
        <NButton @click="handleSave" type="primary" size="small">保存</NButton>
      </NSpace>
    </div>
  </div>
</template>

<style scoped>
.settings-window {
  width: 100vw;
  height: 100vh;
  display: flex;
  flex-direction: column;
  background-color: v-bind('themeVars.bodyColor');
  overflow: hidden;
}

/* 固定头部 */
.settings-header {
  display: flex;
  align-items: center;
  justify-content: space-between;
  padding: 16px 20px;
  border-bottom: 1px solid v-bind('themeVars.dividerColor');
  background-color: v-bind('themeVars.cardColor');
  flex-shrink: 0;
  -webkit-app-region: drag;
}

.settings-title {
  margin: 0;
  font-size: 16px;
  font-weight: 600;
  color: v-bind('themeVars.textColor1');
}

.close-button {
  -webkit-app-region: no-drag;
}

/* 可滚动内容区域 */
.settings-content {
  flex: 1;
  overflow-y: auto;
  padding: 16px 20px;
}

/* 自定义细滚动条 */
.settings-content::-webkit-scrollbar {
  width: 6px;
}

.settings-content::-webkit-scrollbar-track {
  background: transparent;
}

.settings-content::-webkit-scrollbar-thumb {
  background: v-bind('themeVars.scrollbarColor');
  border-radius: 3px;
}

.settings-content::-webkit-scrollbar-thumb:hover {
  background: v-bind('themeVars.scrollbarColorHover');
}

/* 固定底部 */
.settings-footer {
  display: flex;
  justify-content: space-between;
  align-items: center;
  padding: 12px 20px;
  border-top: 1px solid v-bind('themeVars.dividerColor');
  background-color: v-bind('themeVars.cardColor');
  flex-shrink: 0;
}

.loading-container {
  display: flex;
  align-items: center;
  justify-content: center;
  min-height: 400px;
  padding: 60px 20px;
}

/* 默认笔记位置样式 */
.notes-location-container {
  display: flex;
  align-items: center;
  gap: 8px;
  width: 100%;
}

.notes-location-path {
  flex: 1;
  overflow: hidden;
  text-overflow: ellipsis;
  white-space: nowrap;
  cursor: pointer;
  padding: 4px 8px;
  border-radius: 4px;
  transition: background-color 0.2s ease;
  font-size: 13px;
}

.notes-location-path:hover {
  background-color: v-bind('themeVars.hoverColor');
  color: v-bind('themeVars.primaryColor');
}

/* 全局 message 样式调整 - 调整到垂直中间 */
:deep(.n-message-container) {
  top: 50% !important;
  transform: translateY(-50%) !important;
}
</style>
