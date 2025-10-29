<template>
  <Teleport to="body">
    <Transition name="dialog-fade">
      <div v-if="modelValue" class="dialog-overlay" @click="handleOverlayClick">
        <div class="dialog-container" @click.stop>
          <div class="dialog-header">
            <h3 class="dialog-title">设置</h3>
            <button class="dialog-close" @click="handleCancel" title="关闭 (ESC)">
              ×
            </button>
          </div>
          
          <div class="dialog-body">
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
                <label class="setting-label-block">
                  字体大小
                  <div class="font-size-control">
                    <button @click="decreaseFontSize" class="size-btn">-</button>
                    <span class="size-value">{{ localConfig.fontSize }}px</span>
                    <button @click="increaseFontSize" class="size-btn">+</button>
                  </div>
                </label>
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

          <div class="dialog-footer">
            <button class="dialog-button dialog-button-reset" @click="handleReset">
              重置为默认
            </button>
            <div class="footer-right">
              <button class="dialog-button dialog-button-cancel" @click="handleCancel">
                取消
              </button>
              <button class="dialog-button dialog-button-confirm" @click="handleSave">
                保存
              </button>
            </div>
          </div>
        </div>
      </div>
    </Transition>
  </Teleport>
</template>

<script setup lang="ts">
import { ref, watch } from 'vue';
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
  DEFAULT_SHORTCUTS,
  type AppConfig
} from '../utils/configStore';
import { showError, showInfo } from '../utils/dialogHelper';

interface Props {
  modelValue: boolean;
}

interface Emits {
  (e: 'update:modelValue', value: boolean): void;
  (e: 'saved'): void;
}

const props = defineProps<Props>();
const emit = defineEmits<Emits>();

const localConfig = ref<AppConfig>({
  fontSize: 16,
  fontFamily: "Consolas, 'Courier New', monospace",
  lineHeight: 1.6,
  autoStart: false,
  searchDirectories: [],
  defaultNotesLocation: '',
  notes: [],
  theme: 'system',
  shortcuts: DEFAULT_SHORTCUTS
});

// 加载配置
const loadSettings = async () => {
  try {
    const config = await getConfig();
    localConfig.value = { ...config };
  } catch (error) {
    console.error('加载设置失败:', error);
    await showError('加载设置失败');
  }
};

// 监听对话框打开，加载配置
watch(() => props.modelValue, async (newValue) => {
  if (newValue) {
    await loadSettings();
  }
});

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

    await showInfo('设置已保存');
    emit('saved');
    emit('update:modelValue', false);
  } catch (error) {
    console.error('保存设置失败:', error);
    await showError('保存设置失败');
  }
};

// 重置为默认值
const handleReset = async () => {
  localConfig.value = {
    fontSize: 16,
    fontFamily: "Consolas, 'Courier New', monospace",
    lineHeight: 1.6,
    autoStart: false,
    searchDirectories: [],
    defaultNotesLocation: '',
    notes: [],
    theme: 'system',
    shortcuts: DEFAULT_SHORTCUTS
  };
};

// 取消
const handleCancel = () => {
  emit('update:modelValue', false);
};

// 处理遮罩层点击
const handleOverlayClick = () => {
  handleCancel();
};
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
  max-width: 600px;
  max-height: 80vh;
  overflow: hidden;
  display: flex;
  flex-direction: column;
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
  overflow-y: auto;
  flex: 1;
}

.settings-section {
  margin-bottom: 24px;
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
  max-height: 150px;
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

.dialog-footer {
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

.dialog-button-reset {
  background: transparent;
  color: #ef4444;
  border: 1px solid #ef4444;
}

.dialog-button-reset:hover {
  background: #ef4444;
  color: white;
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
}
</style>

