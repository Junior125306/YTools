<script setup lang="ts">
import { ref, onMounted, onUnmounted, watch } from 'vue';
import { invoke } from '@tauri-apps/api/core';
import { getCurrentWindow } from '@tauri-apps/api/window';
import TextEditor from '../components/TextEditor.vue';
import InputDialog from '../components/InputDialog.vue';
import { initConfig, getFontSize, setFontSize, getFontFamily, getLineHeight, getNotes, addNote, removeNote, getDefaultNotesLocation } from '../utils/configStore';
import { showError, showInfo, showWarning, showConfirm } from '../utils/dialogHelper';
import { WebviewWindow } from '@tauri-apps/api/webviewWindow';

// 笔记列表和当前选中的笔记（完整路径）
const notes = ref<string[]>([]);
const activeNote = ref<string>(''); // 当前激活笔记的完整路径
const content = ref<string>('');
const isSaving = ref(false);
const saveStatus = ref<string>('');
// 已移除 Markdown 模式，使用纯文本编辑
let autoSaveTimer: number | null = null;
let isInitialLoad = true; // 标记是否是初始加载
const showCreateDialog = ref(false); // 控制新建笔记对话框显示
const fontSize = ref<number>(16); // 字体大小，默认 16px
const fontFamily = ref<string>("Consolas, 'Courier New', monospace"); // 字体族
const lineHeight = ref<number>(1.6); // 行高

// 辅助函数：从路径提取文件名显示
function getFileName(path: string): string {
  if (!path) return '';
  return path.split(/[/\\]/).pop()?.replace(/\.(md|txt)$/, '') || path;
}

// 从本地存储获取最后编辑的笔记（完整路径）
const getLastActiveNote = () => {
  const lastNote = localStorage.getItem('ytools-last-note');
  return lastNote || '';
};

// 保存当前编辑的笔记到本地存储（完整路径）
const saveLastActiveNote = (path: string) => {
  localStorage.setItem('ytools-last-note', path);
};

// 加载配置
async function loadConfig() {
  try {
    await initConfig(); // 初始化并迁移旧配置
    fontSize.value = await getFontSize();
    fontFamily.value = await getFontFamily();
    lineHeight.value = await getLineHeight();
  } catch (error) {
    console.error('加载配置失败:', error);
    fontSize.value = 16; // 使用默认值
    fontFamily.value = "Consolas, 'Courier New', monospace";
    lineHeight.value = 1.6;
    await showError('加载配置失败，已使用默认值');
  }
}

// 处理字体大小变化
async function handleFontSizeChange(newSize: number) {
  fontSize.value = newSize;
  try {
    await setFontSize(newSize);
  } catch (error) {
    console.error('保存字体大小失败:', error);
  }
}

// 加载所有笔记
async function loadNotes() {
  try {
    const noteList = await getNotes();
    notes.value = noteList;
    
    // 获取最后编辑的笔记
    const lastNote = getLastActiveNote();
    // 如果最后编辑的笔记在列表中，选中它；否则选中第一个（如果有）
    if (lastNote && notes.value.includes(lastNote)) {
      activeNote.value = lastNote;
    } else if (notes.value.length > 0) {
      activeNote.value = notes.value[0];
      saveLastActiveNote(activeNote.value);
    } else {
      activeNote.value = '';
    }
  } catch (error) {
    console.error('加载笔记列表失败:', error);
    notes.value = [];
    activeNote.value = '';
    await showError('加载笔记列表失败');
  }
}

// 加载笔记内容
async function loadNote(notePath: string) {
  if (!notePath) {
    content.value = '';
    return;
  }
  
  try {
    isInitialLoad = true; // 标记为加载状态
    const noteContent = await invoke<string>('read_note', { filename: notePath });
    content.value = noteContent;
    saveStatus.value = '';
    // 延迟解除初始加载状态，避免 watch 触发
    setTimeout(() => {
      isInitialLoad = false;
    }, 100);
  } catch (error) {
    console.error('加载笔记失败:', error);
    content.value = '';
    isInitialLoad = false;
  }
}

// 保存笔记
async function saveNote() {
  if (isSaving.value) return;
  
  isSaving.value = true;
  saveStatus.value = '保存中...';
  
  try {
    await invoke('save_note', {
      filename: activeNote.value,
      content: content.value,
    });
    saveStatus.value = '✓ 已保存';
    setTimeout(() => {
      saveStatus.value = '';
    }, 2000);
  } catch (error) {
    console.error('保存失败:', error);
    saveStatus.value = '✗ 保存失败';
    await showError('保存笔记失败，请检查文件权限或磁盘空间');
  } finally {
    isSaving.value = false;
  }
}

// 切换笔记
function switchNote(notePath: string) {
  if (activeNote.value !== notePath) {
    activeNote.value = notePath;
    saveLastActiveNote(notePath); // 保存当前选择的笔记
    loadNote(notePath);
  }
}

// 处理内容变化
function handleContentChange() {
  saveStatus.value = '未保存';
}

// 新建笔记
function createNote() {
  showCreateDialog.value = true;
}

// 处理创建笔记确认
async function handleCreateConfirm(noteName: string) {
  showCreateDialog.value = false;
  
  try {
    const defaultLocation = await getDefaultNotesLocation();
    const fullName = noteName.endsWith('.md') ? noteName : `${noteName}.md`;
    
    // 调用后端创建文件
    const newPath = await invoke<string>('create_note', {
      name: fullName,
      baseDir: defaultLocation
    });
    
    // 检查是否已存在
    const existing = await getNotes();
    if (existing.includes(newPath)) {
      await showWarning(`笔记 "${noteName}" 已存在！`);
      switchNote(newPath);
      return;
    }
    
    // 添加到配置
    await addNote(newPath);
    await loadNotes();
    switchNote(newPath);
    
    await showInfo(`笔记 "${noteName}" 创建成功！`);
  } catch (error) {
    console.error('创建笔记失败:', error);
    await showError('创建笔记失败');
  }
}

// 处理创建笔记取消
function handleCreateCancel() {
  showCreateDialog.value = false;
}

// 导入笔记
async function importNote() {
  try {
    const filePath = await invoke<string>('import_note');
    
    if (!filePath) return; // 用户取消
    
    // 检查是否已存在
    const existing = await getNotes();
    if (existing.includes(filePath)) {
      await showWarning('该文件已在笔记列表中');
      switchNote(filePath);
      return;
    }
    
    // 添加到配置
    await addNote(filePath);
    await loadNotes();
    switchNote(filePath);
    
    await showInfo('笔记已导入');
  } catch (error) {
    console.error('导入笔记失败:', error);
    await showError('导入笔记失败');
  }
}

// 删除笔记
async function deleteNote(notePath: string) {
  const fileName = getFileName(notePath);
  
  const deleteFile = await showConfirm(
    `是否删除文件 "${fileName}"？\n\n确定：永久删除文件\n取消：仅从列表移除`,
    '删除笔记'
  );
  
  if (deleteFile) {
    try {
      await invoke('delete_note_file', { path: notePath });
      await removeNote(notePath);
      await showInfo('文件已删除');
    } catch (error) {
      console.error('删除文件失败:', error);
      await showError('删除文件失败');
      return;
    }
  } else {
    await removeNote(notePath);
  }
  
  await loadNotes();
  
  // 切换到第一个笔记或空白
  if (activeNote.value === notePath) {
    if (notes.value.length > 0) {
      switchNote(notes.value[0]);
    } else {
      activeNote.value = '';
      content.value = '';
    }
  }
}

// 自动保存 (内容变化后 2 秒自动保存)
watch(content, () => {
  // 跳过初始加载时的触发
  if (isInitialLoad) {
    return;
  }
  
  if (autoSaveTimer) {
    clearTimeout(autoSaveTimer);
  }
  
  saveStatus.value = '未保存';
  
  autoSaveTimer = setTimeout(() => {
    saveNote();
  }, 2000) as unknown as number;
});

// 监听笔记切换
watch(activeNote, (newNote) => {
  // 切换前清除自动保存定时器
  if (autoSaveTimer) {
    clearTimeout(autoSaveTimer);
    autoSaveTimer = null;
  }
  loadNote(newNote);
});

// 快捷键处理
function handleKeydown(e: KeyboardEvent) {
  // 如果对话框打开，不处理 ESC 键（让对话框自己处理）
  if (showCreateDialog.value) {
    return;
  }
  
  // ESC 键隐藏窗口
  if (e.key === 'Escape') {
    e.preventDefault();
    hideWindow();
    return;
  }
  
  // Ctrl+S 保存
  if ((e.ctrlKey || e.metaKey) && e.key === 's') {
    e.preventDefault();
    saveNote();
  }
}

// 隐藏窗口
async function hideWindow() {
  try {
    const appWindow = getCurrentWindow();
    await appWindow.hide();
  } catch (error) {
    console.error('隐藏窗口失败:', error);
  }
}

// 打开设置窗口
async function openSettings() {
  try {
    console.log('尝试打开设置窗口...');
    
    // 检查窗口是否已存在
    const existingWindows = await WebviewWindow.getAll();
    const existingSettings = existingWindows.find(w => w.label === 'settings');
    
    if (existingSettings) {
      // 如果窗口已存在，直接显示和聚焦
      console.log('设置窗口已存在，显示窗口');
      await existingSettings.show();
      await existingSettings.setFocus();
      return;
    }
    
    // 如果窗口不存在，创建新窗口
    const newWindow = new WebviewWindow('settings', {
      url: '/#/settings',
      title: '设置',
      width: 600,
      height: 580,
      minWidth: 500,
      minHeight: 500,
      resizable: true,
      center: true,
      skipTaskbar: true,
      alwaysOnTop: false,
      decorations: false,
      transparent: true,
      visible: true
    });

    console.log('设置窗口已创建');

    // 等待窗口创建完成
    newWindow.once('tauri://created', () => {
      console.log('设置窗口创建成功');
    });

    // 监听窗口关闭事件，重新加载配置
    newWindow.once('tauri://destroyed', async () => {
      console.log('设置窗口已关闭，重新加载配置');
      await loadConfig();
    });

    // 监听错误
    newWindow.once('tauri://error', (error) => {
      console.error('设置窗口创建失败:', error);
      showError('打开设置窗口失败');
    });
  } catch (error) {
    console.error('打开设置窗口出错:', error);
    await showError('打开设置窗口失败: ' + error);
  }
}

// 存储取消监听器的函数
let unlistenSettings: (() => void) | null = null;
let unlistenSettingsSaved: (() => void) | null = null;

// 初始化
onMounted(async () => {
  await loadConfig(); // 先加载配置
  await loadNotes();
  
  // 加载当前激活的笔记
  if (activeNote.value) {
    await loadNote(activeNote.value);
  }
  
  window.addEventListener('keydown', handleKeydown);
  
  // 监听来自托盘菜单的设置事件
  const appWindow = getCurrentWindow();
  unlistenSettings = await appWindow.listen('open-settings', () => {
    openSettings();
  });
  
  // 监听设置保存事件，重新加载配置
  unlistenSettingsSaved = await appWindow.listen('settings-saved', async () => {
    console.log('收到设置保存事件，重新加载配置');
    await loadConfig();
  });
  
  // HomeView 初始化完成
});

// 清理事件监听
onUnmounted(() => {
  window.removeEventListener('keydown', handleKeydown);
  if (autoSaveTimer) {
    clearTimeout(autoSaveTimer);
  }
  // 取消托盘菜单的设置监听
  if (unlistenSettings) {
    unlistenSettings();
  }
  // 取消设置保存事件监听
  if (unlistenSettingsSaved) {
    unlistenSettingsSaved();
  }
});

</script>

<template>
  <div class="app-container">
    <!-- 顶部工具栏 -->
    <div class="toolbar">
      <div class="tabs">
        <button
          v-for="notePath in notes"
          :key="notePath"
          :class="['tab', 'no-drag', { active: activeNote === notePath }]"
          @click="switchNote(notePath)"
          @contextmenu.prevent="deleteNote(notePath)"
          :title="`${getFileName(notePath)}\n右键删除`"
        >
          {{ getFileName(notePath) }}
        </button>
        <button class="tab new-tab no-drag" @click="createNote" title="新建笔记">
          +
        </button>
        <button class="tab new-tab no-drag" @click="importNote" title="导入笔记">
          📁
        </button>
        <!-- 拖拽空白区域：用于在 + 按钮右侧拖动窗口 -->
        <div class="drag-spacer"></div>
      </div>
      <div class="actions">
        <span class="save-status">{{ saveStatus }}</span>
        <button class="settings-btn no-drag" @click="openSettings" title="设置">
          ⚙️
        </button>
      </div>
    </div>

    <!-- 文本编辑器 -->
    <div class="editor-container">
      <TextEditor
        v-model="content"
        :height="'100%'"
        :font-size="fontSize"
        :font-family="fontFamily"
        :line-height="lineHeight"
        @change="handleContentChange"
        @update:fontSize="handleFontSizeChange"
      />
    </div>

    <!-- 新建笔记对话框 -->
    <InputDialog
      v-model="showCreateDialog"
      title="新建笔记"
      placeholder="请输入笔记名称（不需要 .md 后缀）"
      @confirm="handleCreateConfirm"
      @cancel="handleCreateCancel"
    />
  </div>
</template>

<style scoped>
.app-container {
  display: flex;
  flex-direction: column;
  height: 100vh;
  overflow: hidden;
  position: relative;
  /* 强制暗色主题：覆盖变量 */
  --color-bg: #0f1115;
  --color-surface: #12151c;
  --color-surface-2: #161a22;
  --color-border: #232938;
  --color-text: #e5e7eb;
  --color-text-muted: #9ca3af;
  --color-accent: #818cf8;
  --color-accent-hover: #6366f1;
  --color-accent-press: #4f46e5;
  background: var(--color-bg);
}

.toolbar {
  display: flex;
  justify-content: space-between;
  align-items: center;
  padding: 10px 16px;
  background-color: var(--color-surface);
  border-bottom: 1px solid var(--color-border);
  gap: 12px;
  -webkit-app-region: drag; /* 无边框窗口下可拖拽区域 */
}

.tabs {
  display: flex;
  gap: 4px;
  flex: 1;
  overflow-x: auto;
  overflow-y: hidden;
}

.tabs::-webkit-scrollbar {
  height: 4px;
}

.tabs::-webkit-scrollbar-thumb {
  background: #ccc;
  border-radius: 2px;
}


.tab {
  padding: 8px 16px;
  border: 1px solid var(--color-border);
  background-color: var(--color-surface-2);
  border-radius: var(--radius-s) var(--radius-s) 0 0;
  cursor: pointer;
  font-size: 14px;
  white-space: nowrap;
  transition: background var(--tr-fast), color var(--tr-fast), border-color var(--tr-fast);
  color: var(--color-text-muted);
  -webkit-app-region: no-drag;
}

.tab:hover {
  background-color: var(--color-surface);
  color: var(--color-text);
}

.tab.active {
  background-color: var(--color-surface);
  color: var(--color-text);
  font-weight: 600;
  border-bottom-color: transparent;
}

.tab.new-tab {
  font-size: 18px;
  padding: 6px 12px;
  color: var(--color-accent);
  font-weight: 700;
}

.tab.new-tab:hover {
  background-color: var(--color-accent);
  color: #fff;
  border-color: var(--color-accent);
}

/* + 按钮右侧可拖拽空白区域 */
.drag-spacer {
  flex: 1;
  -webkit-app-region: drag;
}

.actions {
  display: flex;
  align-items: center;
  gap: 12px;
}

.save-status {
  font-size: 13px;
  color: var(--color-text-muted);
  min-width: 60px;
  text-align: right;
}

.settings-btn {
  background: transparent;
  border: none;
  padding: 4px 8px;
  cursor: pointer;
  font-size: 18px;
  border-radius: 6px;
  transition: all 0.2s ease;
  display: flex;
  align-items: center;
  justify-content: center;
}

.settings-btn:hover {
  background: var(--color-surface);
  transform: scale(1.1);
}

/* 保存按钮样式已移除 */

.editor-container {
  flex: 1;
  display: flex;
  flex-direction: column;
  overflow: hidden;
  background: var(--color-surface);
  border-radius: 12px;
  margin: 12px;
  box-shadow: 0 4px 12px rgba(0, 0, 0, 0.15);
  border: 1px solid var(--color-border);
  position: relative;
}

/* 模式切换按钮样式已移除 */

.no-drag { -webkit-app-region: no-drag; }

/* 编辑器容器动画效果 */
.editor-container {
  transition: all 0.3s ease;
}

.editor-container:hover {
  box-shadow: 0 6px 16px rgba(0, 0, 0, 0.2);
  transform: translateY(-1px);
}

/* 响应式设计优化 */
@media (max-width: 1024px) {
  .editor-container {
    margin: 8px;
    border-radius: 8px;
  }
  
  .toolbar {
    padding: 8px 12px;
    gap: 8px;
  }
  
  .tabs {
    gap: 2px;
  }
  
  .tab {
    padding: 6px 12px;
    font-size: 13px;
  }
  
  .actions {
    gap: 8px;
  }
}

@media (max-width: 768px) {
  .app-container {
    --color-bg: #0f1115;
    --color-surface: #12151c;
    --color-surface-2: #161a22;
    --color-border: #232938;
    --color-text: #e5e7eb;
    --color-text-muted: #9ca3af;
    --color-accent: #818cf8;
    --color-accent-hover: #6366f1;
    --color-accent-press: #4f46e5;
  }
  
  .toolbar {
    flex-direction: column;
    gap: 8px;
    padding: 8px;
  }
  
  .tabs {
    order: 1;
    width: 100%;
    justify-content: flex-start;
  }
  
  .actions {
    order: 2;
    width: 100%;
    justify-content: space-between;
  }
  
  .editor-container {
    margin: 4px;
    border-radius: 8px;
  }
  
  /* 模式切换按钮样式已移除 */
}
</style>

