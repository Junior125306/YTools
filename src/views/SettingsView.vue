<script setup lang="ts">
import { ref, onMounted, onUnmounted, computed } from 'vue'
import { getCurrentWindow } from '@tauri-apps/api/window'
import { invoke } from '@tauri-apps/api/core'
import { open } from '@tauri-apps/plugin-dialog'
import { enable as enableAutostart, disable as disableAutostart } from '@tauri-apps/plugin-autostart'
import { homeDir } from '@tauri-apps/api/path'
import {
  NForm, NFormItem,
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
const { changeTheme, themeMode } = useTheme()

// 检查是否是赛博朋克主题
const isCyberpunk = computed(() => themeMode.value === 'cyberpunk')

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
    changeTheme(localConfig.value.theme as 'light' | 'dark' | 'cyberpunk' | 'system')

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
  <div class="settings-window" :class="{ 'cyberpunk-mode': isCyberpunk }">
    <!-- 固定头部 -->
    <div class="settings-header">
      <h2 class="settings-title" :class="{ 'cyberpunk-title': isCyberpunk }">设置</h2>
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
        <NDivider title-placement="left" :class="{ 'cyberpunk-divider': isCyberpunk }">主题设置</NDivider>
        
        <!-- 四宫格主题选择器 -->
        <div class="theme-selector">
          <!-- 第一行 -->
          <div class="theme-row">
            <!-- 亮色主题 -->
            <div 
              class="theme-card" 
              :class="{ active: localConfig.theme === 'light' }"
              @click="localConfig.theme = 'light'"
            >
              <div class="theme-preview light-preview">
                <div class="preview-gradient"></div>
                <div class="preview-accent"></div>
              </div>
              <div class="theme-info">
                <span class="theme-icon">☀️</span>
                <span class="theme-name">亮色</span>
              </div>
              <div v-if="localConfig.theme === 'light'" class="check-mark">✓</div>
            </div>
            
            <!-- 暗色主题 -->
            <div 
              class="theme-card" 
              :class="{ active: localConfig.theme === 'dark' }"
              @click="localConfig.theme = 'dark'"
            >
              <div class="theme-preview dark-preview">
                <div class="preview-gradient"></div>
                <div class="preview-accent"></div>
              </div>
              <div class="theme-info">
                <span class="theme-icon">🌙</span>
                <span class="theme-name">暗色</span>
              </div>
              <div v-if="localConfig.theme === 'dark'" class="check-mark">✓</div>
            </div>
          </div>
          
          <!-- 第二行 -->
          <div class="theme-row">
            <!-- 赛博朋克主题 -->
            <div 
              class="theme-card" 
              :class="{ active: localConfig.theme === 'cyberpunk' }"
              @click="localConfig.theme = 'cyberpunk'"
            >
              <div class="theme-preview cyberpunk-preview">
                <div class="preview-gradient"></div>
                <div class="neon-line cyan"></div>
                <div class="neon-line yellow"></div>
              </div>
              <div class="theme-info">
                <span class="theme-icon">⚡</span>
                <span class="theme-name">cyberpunk</span>
              </div>
              <div v-if="localConfig.theme === 'cyberpunk'" class="check-mark">✓</div>
            </div>
            
            <!-- 跟随系统主题 -->
            <div 
              class="theme-card" 
              :class="{ active: localConfig.theme === 'system' }"
              @click="localConfig.theme = 'system'"
            >
              <div class="theme-preview system-preview">
                <div class="half-light"></div>
                <div class="system-divider"></div>
                <div class="half-dark"></div>
              </div>
              <div class="theme-info">
                <span class="theme-icon">🔄</span>
                <span class="theme-name">跟随系统</span>
              </div>
              <div v-if="localConfig.theme === 'system'" class="check-mark">✓</div>
            </div>
          </div>
        </div>

        <!-- 通用设置 -->
        <NDivider title-placement="left" :class="{ 'cyberpunk-divider': isCyberpunk }">通用设置</NDivider>
        <NFormItem label="开机启动">
          <NSwitch v-model:value="localConfig.autoStart" />
        </NFormItem>

        <!-- 编辑器设置 -->
        <NDivider title-placement="left" :class="{ 'cyberpunk-divider': isCyberpunk }">编辑器设置</NDivider>
        
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
        <NDivider title-placement="left" :class="{ 'cyberpunk-divider': isCyberpunk }">搜索设置</NDivider>
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
        <NDivider title-placement="left" :class="{ 'cyberpunk-divider': isCyberpunk }">笔记设置</NDivider>
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
      <NButton 
        @click="handleReset" 
        type="error" 
        secondary 
        size="small"
        :class="{ 'cyberpunk-button-error': isCyberpunk }"
      >
        重置设置
      </NButton>
      <NSpace :size="8">
        <NButton 
          @click="closeWindow" 
          size="small"
          :class="{ 'cyberpunk-button-secondary': isCyberpunk }"
        >
          取消
        </NButton>
        <NButton 
          @click="handleSave" 
          type="primary" 
          size="small"
          :class="{ 'cyberpunk-button-primary': isCyberpunk }"
        >
          保存
        </NButton>
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
  padding: 16px 16px;
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

/* ==================== 四宫格主题选择器样式 ==================== */

.theme-selector {
  width: 100%;
  padding: 8px 0;
  margin-bottom: 8px;
}

.theme-row {
  display: flex;
  justify-content: center;
  gap: 16px;
  margin-bottom: 12px;
}

.theme-row:last-child {
  margin-bottom: 0;
}

.theme-card {
  flex: 1;
  max-width: 125px;
  position: relative;
  border-radius: 10px;
  overflow: hidden;
  cursor: pointer;
  transition: all 0.3s cubic-bezier(0.4, 0, 0.2, 1);
  border: 2px solid rgba(92, 207, 230, 0.2);
  background: v-bind('themeVars.cardColor');
}

.theme-card:hover {
  border-color: rgba(92, 207, 230, 0.5);
  transform: translateY(-2px);
  box-shadow: 0 4px 16px rgba(92, 207, 230, 0.2);
}

.theme-card.active {
  border: 3px solid #5ccfe6;
  box-shadow: 
    0 0 0 3px rgba(92, 207, 230, 0.2),
    0 8px 24px rgba(92, 207, 230, 0.3);
}

/* 主题预览区域 */
.theme-preview {
  height: 70px;
  position: relative;
  overflow: hidden;
}

/* 勾选标记 */
.check-mark {
  position: absolute;
  top: 6px;
  right: 6px;
  width: 20px;
  height: 20px;
  border-radius: 50%;
  background: #5ccfe6;
  color: #0f1419;
  display: flex;
  align-items: center;
  justify-content: center;
  font-weight: bold;
  font-size: 12px;
  z-index: 10;
  box-shadow: 0 2px 8px rgba(92, 207, 230, 0.4);
}

/* 主题信息区域 */
.theme-info {
  display: flex;
  align-items: center;
  justify-content: center;
  gap: 4px;
  padding: 8px 6px;
  background: v-bind('themeVars.cardColor');
}

.theme-icon {
  font-size: 15px;
  line-height: 1;
}

.theme-name {
  font-size: 12px;
  font-weight: 500;
  color: v-bind('themeVars.textColor1');
}

/* ==================== 亮色主题预览 ==================== */

.light-preview .preview-gradient {
  width: 100%;
  height: 100%;
  background: linear-gradient(135deg, 
    #fafaf9 0%,
    #f5f5f4 50%,
    #e7e5e4 100%
  );
}

.light-preview .preview-accent {
  position: absolute;
  bottom: 0;
  left: 0;
  right: 0;
  height: 8px;
  background: linear-gradient(90deg, 
    #a78bfa 0%,
    #c4b5fd 100%
  );
}

/* ==================== 暗色主题预览 ==================== */

.dark-preview .preview-gradient {
  width: 100%;
  height: 100%;
  background: linear-gradient(135deg,
    #18181c 0%,
    #101014 100%
  );
}

.dark-preview .preview-accent {
  position: absolute;
  bottom: 0;
  left: 0;
  right: 0;
  height: 8px;
  background: linear-gradient(90deg,
    #8b5cf6 0%,
    #a78bfa 100%
  );
  box-shadow: 0 0 8px rgba(167, 139, 250, 0.5);
}

/* ==================== 赛博朋克主题预览 ==================== */

.cyberpunk-preview .preview-gradient {
  width: 100%;
  height: 100%;
  background: #0f1419;
}

.cyberpunk-preview .neon-line {
  position: absolute;
  left: 16px;
  right: 16px;
  height: 3px;
  border-radius: 2px;
}

.cyberpunk-preview .neon-line.cyan {
  top: 28px;
  background: #5ccfe6;
  box-shadow: 
    0 0 8px #5ccfe6,
    0 0 16px #5ccfe6,
    0 0 24px rgba(92, 207, 230, 0.5);
}

.cyberpunk-preview .neon-line.yellow {
  top: 48px;
  background: #ffcc66;
  box-shadow: 
    0 0 8px #ffcc66,
    0 0 16px #ffcc66,
    0 0 24px rgba(255, 204, 102, 0.5);
}

/* ==================== 跟随系统主题预览 ==================== */

.system-preview {
  display: flex;
  position: relative;
}

.system-preview .half-light {
  flex: 1;
  background: linear-gradient(to right,
    #f5f5f4 0%,
    #e7e5e4 100%
  );
}

.system-preview .half-dark {
  flex: 1;
  background: linear-gradient(to left,
    #101014 0%,
    #18181c 100%
  );
}

.system-preview .system-divider {
  position: absolute;
  left: 50%;
  top: 0;
  bottom: 0;
  width: 2px;
  background: linear-gradient(to bottom,
    #a78bfa 0%,
    #5ccfe6 50%,
    #ffcc66 100%
  );
  box-shadow: 0 0 8px rgba(92, 207, 230, 0.5);
  transform: translateX(-50%);
}

/* ==================== 响应式调整 ==================== */

@media (max-width: 500px) {
  .theme-row {
    gap: 12px;
  }
  
  .theme-card {
    max-width: 110px;
  }
  
  .theme-preview {
    height: 60px;
  }
  
  .theme-name {
    font-size: 11px;
  }
  
  .theme-icon {
    font-size: 14px;
  }
}

/* ==================== 赛博朋克霓虹效果 ==================== */

/* 窗口标题霓虹效果 */
.cyberpunk-title {
  color: #5ccfe6 !important;
  text-shadow: 
    0 0 6px rgba(92, 207, 230, 0.6),
    0 0 12px rgba(92, 207, 230, 0.3);
  animation: title-glow 3s ease-in-out infinite;
}

@keyframes title-glow {
  0%, 100% {
    text-shadow: 
      0 0 6px rgba(92, 207, 230, 0.6),
      0 0 12px rgba(92, 207, 230, 0.3);
  }
  50% {
    text-shadow: 
      0 0 8px rgba(92, 207, 230, 0.8),
      0 0 16px rgba(92, 207, 230, 0.5);
  }
}

/* 分隔线霓虹效果 */
:deep(.cyberpunk-divider) {
  .n-divider__line {
    background: linear-gradient(90deg,
      #5ccfe6 0%,
      #ffcc66 50%,
      #5ccfe6 100%
    );
    height: 2px !important;
    box-shadow: 
      0 0 6px rgba(92, 207, 230, 0.6),
      0 0 12px rgba(92, 207, 230, 0.4),
      0 0 18px rgba(255, 204, 102, 0.3);
    animation: neon-pulse 2s ease-in-out infinite;
  }
  
  .n-divider__title {
    color: #5ccfe6;
    text-shadow: 
      0 0 4px rgba(92, 207, 230, 0.4),
      0 0 8px rgba(92, 207, 230, 0.25);
    font-weight: 600;
  }
}

/* 主按钮霓虹效果（保存） */
:deep(.cyberpunk-button-primary) {
  background: #5ccfe6 !important;
  border-color: #5ccfe6 !important;
  box-shadow: 
    0 0 12px rgba(92, 207, 230, 0.6),
    0 0 24px rgba(92, 207, 230, 0.4),
    0 4px 8px rgba(0, 0, 0, 0.3) !important;
  transition: all 0.3s ease;
}

:deep(.cyberpunk-button-primary:hover) {
  box-shadow: 
    0 0 16px rgba(92, 207, 230, 0.8),
    0 0 32px rgba(92, 207, 230, 0.6),
    0 0 48px rgba(92, 207, 230, 0.4),
    0 4px 12px rgba(0, 0, 0, 0.4) !important;
  transform: translateY(-1px);
}

:deep(.cyberpunk-button-primary:active) {
  box-shadow: 
    0 0 8px rgba(92, 207, 230, 0.6),
    0 0 16px rgba(92, 207, 230, 0.4),
    0 2px 4px rgba(0, 0, 0, 0.3) !important;
  transform: translateY(0);
}

/* 次要按钮霓虹效果（取消） */
:deep(.cyberpunk-button-secondary) {
  border-color: rgba(92, 207, 230, 0.5) !important;
  box-shadow: 
    0 0 8px rgba(92, 207, 230, 0.3),
    0 2px 6px rgba(0, 0, 0, 0.2) !important;
}

:deep(.cyberpunk-button-secondary:hover) {
  border-color: rgba(92, 207, 230, 0.8) !important;
  box-shadow: 
    0 0 12px rgba(92, 207, 230, 0.5),
    0 0 24px rgba(92, 207, 230, 0.3),
    0 4px 8px rgba(0, 0, 0, 0.3) !important;
}

/* 错误按钮霓虹效果（重置） */
:deep(.cyberpunk-button-error) {
  border: 1px solid #ff6b73 !important;
  color: #ff6b73 !important;
  box-shadow: 
    0 0 6px rgba(255, 107, 115, 0.4),
    0 0 12px rgba(255, 107, 115, 0.2),
    0 2px 4px rgba(0, 0, 0, 0.2) !important;
}

:deep(.cyberpunk-button-error:hover) {
  border: 1px solid #ff6b73 !important;
  background: rgba(255, 107, 115, 0.08) !important;
  box-shadow: 
    0 0 8px rgba(255, 107, 115, 0.5),
    0 0 16px rgba(255, 107, 115, 0.3),
    0 0 24px rgba(255, 107, 115, 0.2),
    0 4px 8px rgba(0, 0, 0, 0.3) !important;
  transform: translateY(-1px);
}

/* 滚动条霓虹效果 */
.settings-window.cyberpunk-mode .settings-content::-webkit-scrollbar-thumb {
  background: linear-gradient(180deg, 
    #5ccfe6 0%, 
    #4fb3c9 50%, 
    #5ccfe6 100%
  );
  border-radius: 3px;
  box-shadow: 
    0 0 8px rgba(92, 207, 230, 0.8),
    0 0 16px rgba(92, 207, 230, 0.5);
}

.settings-window.cyberpunk-mode .settings-content::-webkit-scrollbar-thumb:hover {
  background: linear-gradient(180deg, 
    #6fdbf0 0%, 
    #5ccfe6 50%, 
    #6fdbf0 100%
  );
  box-shadow: 
    0 0 12px rgba(92, 207, 230, 1),
    0 0 24px rgba(92, 207, 230, 0.8),
    0 0 36px rgba(92, 207, 230, 0.5);
}

.settings-window.cyberpunk-mode .settings-content::-webkit-scrollbar-track {
  background: rgba(92, 207, 230, 0.05);
  box-shadow: inset 0 0 6px rgba(92, 207, 230, 0.2);
}

/* 霓虹脉冲动画 */
@keyframes neon-pulse {
  0%, 100% {
    opacity: 1;
    box-shadow: 
      0 0 6px rgba(92, 207, 230, 0.6),
      0 0 12px rgba(92, 207, 230, 0.4),
      0 0 18px rgba(255, 204, 102, 0.3);
  }
  50% {
    opacity: 1;
    box-shadow: 
      0 0 8px rgba(92, 207, 230, 0.8),
      0 0 16px rgba(92, 207, 230, 0.6),
      0 0 24px rgba(255, 204, 102, 0.5);
  }
}
</style>
