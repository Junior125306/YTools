<template>
  <div class="settings-window">
    <div class="settings-container">
      <div class="settings-header">
        <h2 class="settings-title">设置</h2>
        <button class="close-btn" @click="closeWindow" title="关闭 (ESC)">
          ×
        </button>
      </div>
      
      <div class="settings-body">
        <!-- 通用设置 -->
        <section class="settings-section">
          <h4 class="section-title">通用设置</h4>
          <div class="setting-item">
            <label class="setting-label">
              <input
                type="checkbox"
                v-model="localConfig.autoStart"
                class="setting-checkbox"
              />
              <span>开机启动</span>
            </label>
          </div>
        </section>

        <!-- 编辑器设置 -->
        <section class="settings-section">
          <h4 class="section-title">编辑器设置</h4>
          
          <div class="setting-item">
            <div class="setting-label-block">
              字体大小
            </div>
            <div class="font-size-control">
              <button @click="decreaseFontSize" class="size-btn no-drag">-</button>
              <span class="size-value">{{ localConfig.fontSize }}px</span>
              <button @click="increaseFontSize" class="size-btn no-drag">+</button>
            </div>
          </div>

          <div class="setting-item">
            <label class="setting-label-block">
              字体族
              <select v-model="localConfig.fontFamily" class="setting-select">
                <option value="Consolas, 'Courier New', monospace">Consolas (等宽)</option>
                <option value="'Microsoft YaHei', sans-serif">微软雅黑</option>
                <option value="'SimHei', sans-serif">黑体</option>
                <option value="'KaiTi', serif">楷体</option>
                <option value="monospace">系统等宽字体</option>
              </select>
            </label>
          </div>

          <div class="setting-item">
            <label class="setting-label-block">
              行高
              <select v-model.number="localConfig.lineHeight" class="setting-select">
                <option :value="1.4">1.4 (紧凑)</option>
                <option :value="1.6">1.6 (标准)</option>
                <option :value="1.8">1.8 (舒适)</option>
                <option :value="2.0">2.0 (宽松)</option>
              </select>
            </label>
          </div>
        </section>

        <!-- 搜索设置 -->
        <section class="settings-section">
          <h4 class="section-title">搜索设置</h4>
          <div class="setting-item">
            <div class="setting-label-block">
              搜索目录
              <div class="directory-list">
                <div
                  v-for="(dir, index) in localConfig.searchDirectories"
                  :key="index"
                  class="directory-item"
                >
                  <span class="directory-path">{{ dir }}</span>
                  <button @click="removeDirectory(index)" class="remove-btn">删除</button>
                </div>
                <div v-if="localConfig.searchDirectories.length === 0" class="empty-hint">
                  暂无搜索目录
                </div>
              </div>
              <button @click="addDirectory" class="add-btn">添加目录</button>
            </div>
          </div>
        </section>

        <!-- 笔记设置 -->
        <section class="settings-section">
          <h4 class="section-title">笔记设置</h4>
          <div class="setting-item">
            <label class="setting-label-block">
              默认笔记位置
              <div class="path-selector">
                <input
                  type="text"
                  v-model="localConfig.defaultNotesLocation"
                  readonly
                  class="path-input"
                  placeholder="使用默认位置 (.ytools)"
                />
                <button @click="selectNotesLocation" class="browse-btn">浏览</button>
              </div>
            </label>
          </div>
        </section>
      </div>

      <div class="settings-footer">
        <button class="settings-button settings-button-reset" @click="handleReset">
          重置为默认
        </button>
        <div class="footer-right">
          <button class="settings-button settings-button-save" @click="handleSave">
            保存
          </button>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, onMounted, onUnmounted } from 'vue';
import { getCurrentWindow } from '@tauri-apps/api/window';
import { open } from '@tauri-apps/plugin-dialog';
import { enable as enableAutostart, disable as disableAutostart } from '@tauri-apps/plugin-autostart';
import {
  getConfig,
  setFontSize,
  setFontFamily,
  setLineHeight,
  setAutoStart,
  setSearchDirectories,
  setDefaultNotesLocation,
  resetConfig,
  type AppConfig
} from '../utils/configStore';
import { showError, showInfo } from '../utils/dialogHelper';

const currentWindow = getCurrentWindow();

const localConfig = ref<AppConfig>({
  fontSize: 16,
  fontFamily: "Consolas, 'Courier New', monospace",
  lineHeight: 1.6,
  autoStart: false,
  searchDirectories: [],
  defaultNotesLocation: '',
  notes: []
});

// 加载配置
const loadSettings = async () => {
  try {
    const config = await getConfig();
    // 确保 searchDirectories 总是一个数组
    localConfig.value = { 
      ...config,
      searchDirectories: Array.isArray(config.searchDirectories) ? config.searchDirectories : []
    };
  } catch (error) {
    console.error('加载设置失败:', error);
    await showError('加载设置失败');
  }
};

// 字体大小调整
const increaseFontSize = () => {
  if (localConfig.value.fontSize < 32) {
    localConfig.value.fontSize += 2;
  }
};

const decreaseFontSize = () => {
  if (localConfig.value.fontSize > 12) {
    localConfig.value.fontSize -= 2;
  }
};

// 添加搜索目录
const addDirectory = async () => {
  try {
    const selected = await open({
      directory: true,
      multiple: false,
      title: '选择搜索目录'
    });
    
    if (selected && typeof selected === 'string') {
      // 确保 searchDirectories 是一个数组
      if (!Array.isArray(localConfig.value.searchDirectories)) {
        localConfig.value.searchDirectories = [];
      }
      if (!localConfig.value.searchDirectories.includes(selected)) {
        localConfig.value.searchDirectories.push(selected);
      }
    }
  } catch (error) {
    console.error('选择目录失败:', error);
    await showError('选择目录失败');
  }
};

// 删除搜索目录
const removeDirectory = (index: number) => {
  localConfig.value.searchDirectories.splice(index, 1);
};

// 选择笔记位置
const selectNotesLocation = async () => {
  try {
    const selected = await open({
      directory: true,
      multiple: false,
      title: '选择默认笔记位置'
    });
    
    if (selected && typeof selected === 'string') {
      localConfig.value.defaultNotesLocation = selected;
    }
  } catch (error) {
    console.error('选择目录失败:', error);
    await showError('选择目录失败');
  }
};

// 保存设置
const handleSave = async () => {
  try {
    await setFontSize(localConfig.value.fontSize);
    await setFontFamily(localConfig.value.fontFamily);
    await setLineHeight(localConfig.value.lineHeight);
    await setAutoStart(localConfig.value.autoStart);
    await setSearchDirectories(localConfig.value.searchDirectories);
    await setDefaultNotesLocation(localConfig.value.defaultNotesLocation);

    // 应用开机启动设置
    try {
      if (localConfig.value.autoStart) {
        await enableAutostart();
      } else {
        await disableAutostart();
      }
    } catch (error) {
      console.error('设置开机启动失败:', error);
    }

    // 通知主窗口重新加载配置
    try {
      const { WebviewWindow } = await import('@tauri-apps/api/webviewWindow');
      const mainWindow = await WebviewWindow.getByLabel('main');
      if (mainWindow) {
        await mainWindow.emit('settings-saved', {});
      }
    } catch (error) {
      console.error('通知主窗口失败:', error);
    }

    await showInfo('设置已保存');
    await currentWindow.hide();
  } catch (error) {
    console.error('保存设置失败:', error);
    await showError('保存设置失败');
  }
};

// 重置为默认值
const handleReset = async () => {
  try {
    await resetConfig();
    await loadSettings(); // 重新加载配置
    await showInfo('已重置为默认设置');
  } catch (error) {
    console.error('重置设置失败:', error);
    await showError('重置设置失败');
  }
};

// 关闭窗口
const closeWindow = async () => {
  await currentWindow.hide();
};

// 处理键盘事件
const handleKeydown = (e: KeyboardEvent) => {
  if (e.key === 'Escape') {
    closeWindow();
  }
};

// 初始化
onMounted(async () => {
  await loadSettings();
  
  // 只在窗口第一次显示时加载配置，而不是每次获得焦点都加载
  // 这样可以避免在选择目录时覆盖用户的修改
  
  document.addEventListener('keydown', handleKeydown);
});

onUnmounted(() => {
  document.removeEventListener('keydown', handleKeydown);
});
</script>

<style scoped>
.settings-window {
  width: 100vw;
  height: 100vh;
  display: flex;
  align-items: center;
  justify-content: center;
  padding: 0;
  position: fixed;
  top: 0;
  left: 0;
  background: var(--color-bg);
}

.settings-container {
  width: 100%;
  height: 100%;
  background: var(--color-surface);
  display: flex;
  flex-direction: column;
  overflow: hidden;
}

.settings-header {
  display: flex;
  align-items: center;
  justify-content: space-between;
  padding: 16px 20px;
  border-bottom: 1px solid var(--color-border);
  background: var(--color-surface-2);
}

.settings-title {
  margin: 0;
  font-size: 18px;
  font-weight: 600;
  color: var(--color-text);
}

.close-btn {
  background: none;
  border: none;
  color: var(--color-text-muted);
  font-size: 32px;
  line-height: 1;
  cursor: pointer;
  padding: 0;
  width: 32px;
  height: 32px;
  display: flex;
  align-items: center;
  justify-content: center;
  border-radius: 6px;
  transition: all 0.2s ease;
}

.close-btn:hover {
  background: var(--color-surface);
  color: var(--color-text);
}

.settings-body {
  flex: 1;
  padding: 20px;
  overflow-y: auto;
}

.settings-body::-webkit-scrollbar {
  width: 8px;
}

.settings-body::-webkit-scrollbar-track {
  background: transparent;
}

.settings-body::-webkit-scrollbar-thumb {
  background: color-mix(in srgb, var(--color-text) 12%, transparent);
  border-radius: 4px;
}

.settings-body::-webkit-scrollbar-thumb:hover {
  background: color-mix(in srgb, var(--color-text) 20%, transparent);
}

.settings-section {
  margin-bottom: 32px;
}

.settings-section:last-child {
  margin-bottom: 0;
}

.section-title {
  margin: 0 0 16px 0;
  font-size: 14px;
  font-weight: 600;
  color: var(--color-accent);
  text-transform: uppercase;
  letter-spacing: 0.5px;
}

.setting-item {
  margin-bottom: 16px;
}

.setting-item:last-child {
  margin-bottom: 0;
}

.setting-label {
  display: flex;
  align-items: center;
  gap: 8px;
  color: var(--color-text);
  font-size: 14px;
  cursor: pointer;
}

.setting-label-block {
  display: block;
  color: var(--color-text);
  font-size: 14px;
  font-weight: 500;
  margin-bottom: 8px;
}

.setting-checkbox {
  width: 18px;
  height: 18px;
  cursor: pointer;
}

.font-size-control {
  display: flex;
  align-items: center;
  gap: 12px;
  margin-top: 8px;
}

.size-btn {
  width: 32px;
  height: 32px;
  border: 1px solid var(--color-border);
  background: var(--color-surface);
  color: var(--color-text);
  border-radius: 6px;
  cursor: pointer;
  font-size: 18px;
  font-weight: 600;
  transition: all 0.2s ease;
}

.size-btn:hover {
  background: var(--color-accent);
  color: white;
  border-color: var(--color-accent);
}

.size-value {
  font-size: 14px;
  font-weight: 600;
  color: var(--color-text);
  min-width: 50px;
  text-align: center;
}

.setting-select {
  width: 100%;
  padding: 8px 12px;
  border: 1px solid var(--color-border);
  border-radius: 6px;
  background: var(--color-bg);
  color: var(--color-text);
  font-size: 14px;
  cursor: pointer;
  margin-top: 8px;
}

.setting-select:focus {
  outline: none;
  border-color: var(--color-accent);
}

.directory-list {
  margin-top: 8px;
  margin-bottom: 12px;
  border: 1px solid var(--color-border);
  border-radius: 6px;
  background: var(--color-bg);
  padding: 8px;
  min-height: 80px;
  max-height: 200px;
  overflow-y: auto;
}

.directory-item {
  display: flex;
  align-items: center;
  justify-content: space-between;
  padding: 8px;
  background: var(--color-surface);
  border-radius: 4px;
  margin-bottom: 6px;
}

.directory-item:last-child {
  margin-bottom: 0;
}

.directory-path {
  flex: 1;
  font-size: 13px;
  color: var(--color-text);
  word-break: break-all;
}

.remove-btn {
  padding: 4px 12px;
  border: 1px solid #ef4444;
  background: transparent;
  color: #ef4444;
  border-radius: 4px;
  font-size: 12px;
  cursor: pointer;
  transition: all 0.2s ease;
}

.remove-btn:hover {
  background: #ef4444;
  color: white;
}

.empty-hint {
  padding: 16px;
  text-align: center;
  color: var(--color-text-muted);
  font-size: 13px;
}

.add-btn {
  width: 100%;
  padding: 8px 16px;
  border: 1px dashed var(--color-border);
  background: transparent;
  color: var(--color-text);
  border-radius: 6px;
  font-size: 14px;
  cursor: pointer;
  transition: all 0.2s ease;
}

.add-btn:hover {
  border-color: var(--color-accent);
  color: var(--color-accent);
  background: rgba(129, 140, 248, 0.05);
}

.path-selector {
  display: flex;
  gap: 8px;
  margin-top: 8px;
}

.path-input {
  flex: 1;
  padding: 8px 12px;
  border: 1px solid var(--color-border);
  border-radius: 6px;
  background: var(--color-bg);
  color: var(--color-text);
  font-size: 14px;
}

.path-input::placeholder {
  color: var(--color-text-muted);
}

.browse-btn {
  padding: 8px 16px;
  border: 1px solid var(--color-border);
  background: var(--color-surface);
  color: var(--color-text);
  border-radius: 6px;
  font-size: 14px;
  cursor: pointer;
  transition: all 0.2s ease;
}

.browse-btn:hover {
  background: var(--color-accent);
  color: white;
  border-color: var(--color-accent);
}

.settings-footer {
  display: flex;
  justify-content: space-between;
  align-items: center;
  padding: 16px 20px;
  border-top: 1px solid var(--color-border);
  background: var(--color-surface-2);
}

.footer-right {
  display: flex;
  gap: 12px;
}

.settings-button {
  padding: 10px 24px;
  border-radius: 6px;
  font-size: 14px;
  font-weight: 500;
  cursor: pointer;
  transition: all 0.2s ease;
  border: none;
  outline: none;
}

.settings-button-save {
  background: var(--color-accent);
  color: white;
}

.settings-button-save:hover {
  background: var(--color-accent-hover);
  transform: translateY(-1px);
  box-shadow: 0 4px 12px rgba(129, 140, 248, 0.3);
}

.settings-button-reset {
  background: transparent;
  color: #ef4444;
  border: 1px solid #ef4444;
}

.settings-button-reset:hover {
  background: #ef4444;
  color: white;
}
</style>

<style>
/* 为设置窗口设置背景 */
body:has(.settings-window) {
  background: var(--color-bg) !important;
  margin: 0 !important;
  padding: 0 !important;
}

html:has(.settings-window) {
  background: var(--color-bg) !important;
  margin: 0 !important;
  padding: 0 !important;
}

#app:has(.settings-window) {
  background: var(--color-bg) !important;
}
</style>

