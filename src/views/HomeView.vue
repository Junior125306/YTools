<script setup lang="ts">
import { ref, onMounted, onUnmounted, watch } from 'vue';
import { invoke } from '@tauri-apps/api/core';
import { getCurrentWindow } from '@tauri-apps/api/window';
import TextEditor from '../components/TextEditor.vue';
import InputDialog from '../components/InputDialog.vue';

// 笔记列表和当前选中的笔记
const notes = ref<string[]>([]);
const activeNote = ref<string>('notes.md');
const content = ref<string>('');
const isSaving = ref(false);
const saveStatus = ref<string>('');
// 已移除 Markdown 模式，使用纯文本编辑
let autoSaveTimer: number | null = null;
let isInitialLoad = true; // 标记是否是初始加载
const showCreateDialog = ref(false); // 控制新建笔记对话框显示

// 从本地存储获取最后编辑的笔记
const getLastActiveNote = () => {
  const lastNote = localStorage.getItem('ytools-last-note');
  return lastNote || 'notes.md';
};

// 保存当前编辑的笔记到本地存储
const saveLastActiveNote = (filename: string) => {
  localStorage.setItem('ytools-last-note', filename);
};

// 加载所有笔记
async function loadNotes() {
  try {
    const noteList = await invoke<string[]>('list_notes');
    notes.value = noteList.length > 0 ? noteList : ['notes.md'];
    
    // 获取最后编辑的笔记
    const lastNote = getLastActiveNote();
    // 如果最后编辑的笔记在列表中，选中它；否则选中第一个
    if (notes.value.includes(lastNote)) {
      activeNote.value = lastNote;
    } else {
      activeNote.value = notes.value[0];
      saveLastActiveNote(activeNote.value);
    }
  } catch (error) {
    console.error('加载笔记列表失败:', error);
    notes.value = ['notes.md'];
    activeNote.value = 'notes.md';
  }
}

// 加载笔记内容
async function loadNote(filename: string) {
  try {
    isInitialLoad = true; // 标记为加载状态
    const noteContent = await invoke<string>('read_note', { filename });
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
  } finally {
    isSaving.value = false;
  }
}

// 切换笔记
function switchNote(filename: string) {
  if (activeNote.value !== filename) {
    activeNote.value = filename;
    saveLastActiveNote(filename); // 保存当前选择的笔记
    loadNote(filename);
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
function handleCreateConfirm(noteName: string) {
  const filename = noteName.endsWith('.md') ? noteName : `${noteName}.md`;
  if (!notes.value.includes(filename)) {
    notes.value.push(filename);
    activeNote.value = filename;
    saveLastActiveNote(filename); // 保存新创建的笔记
    content.value = '';
    saveNote();
    showCreateDialog.value = false;
  } else {
    alert('该笔记已存在！');
  }
}

// 处理创建笔记取消
function handleCreateCancel() {
  showCreateDialog.value = false;
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

// 初始化
onMounted(async () => {
  await loadNotes();
  await loadNote(activeNote.value);
  window.addEventListener('keydown', handleKeydown);
  
  // HomeView 初始化完成
});

// 清理事件监听
onUnmounted(() => {
  window.removeEventListener('keydown', handleKeydown);
  if (autoSaveTimer) {
    clearTimeout(autoSaveTimer);
  }
});

</script>

<template>
  <div class="app-container">
    <!-- 顶部工具栏 -->
    <div class="toolbar">
      <div class="tabs">
        <button
          v-for="note in notes"
          :key="note"
          :class="['tab', 'no-drag', { active: activeNote === note }]"
          @click="switchNote(note)"
        >
          {{ note.replace('.md', '') }}
        </button>
        <button class="tab new-tab no-drag" @click="createNote" title="新建笔记">
          +
        </button>
      </div>
      <div class="actions">
        <span class="save-status">{{ saveStatus }}</span>
      </div>
    </div>

    <!-- 文本编辑器 -->
    <div class="editor-container">
      <TextEditor
        v-model="content"
        :height="'100%'"
        @change="handleContentChange"
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

