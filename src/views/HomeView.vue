<script setup lang="ts">
import { ref, onMounted, watch, computed } from 'vue';
import { invoke } from '@tauri-apps/api/core';

// 笔记列表和当前选中的笔记
const notes = ref<string[]>([]);
const activeNote = ref<string>('notes.md');
const content = ref<string>('');
const isSaving = ref(false);
const saveStatus = ref<string>('');
const isMdMode = ref(true); // 默认展示为可编辑的 Markdown（左编右预览）
let autoSaveTimer: number | null = null;
let isInitialLoad = true; // 标记是否是初始加载

// 加载所有笔记
async function loadNotes() {
  try {
    const noteList = await invoke<string[]>('list_notes');
    notes.value = noteList.length > 0 ? noteList : ['notes.md'];
    
    // 如果当前选中的笔记不在列表中，选中第一个
    if (!notes.value.includes(activeNote.value)) {
      activeNote.value = notes.value[0];
    }
  } catch (error) {
    console.error('加载笔记列表失败:', error);
    notes.value = ['notes.md'];
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
    loadNote(filename);
  }
}

// 新建笔记
function createNote() {
  const noteName = prompt('请输入笔记名称（不需要 .md 后缀）:');
  if (noteName) {
    const filename = noteName.endsWith('.md') ? noteName : `${noteName}.md`;
    if (!notes.value.includes(filename)) {
      notes.value.push(filename);
      activeNote.value = filename;
      content.value = '';
      saveNote();
    } else {
      alert('该笔记已存在！');
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

// 快捷键保存 (Ctrl+S)
function handleKeydown(e: KeyboardEvent) {
  if ((e.ctrlKey || e.metaKey) && e.key === 's') {
    e.preventDefault();
    saveNote();
  }
}

// 初始化
onMounted(async () => {
  await loadNotes();
  await loadNote(activeNote.value);
  window.addEventListener('keydown', handleKeydown);
});

// Markdown 渲染（简易、安全处理）
function escapeHtml(raw: string): string {
  return raw
    .replace(/&/g, '&amp;')
    .replace(/</g, '&lt;')
    .replace(/>/g, '&gt;');
}

function renderMarkdown(md: string): string {
  const escaped = escapeHtml(md);
  // 代码块 ```
  let html = escaped.replace(/```([\s\S]*?)```/g, (_m, p1) => {
    return `<pre class="md-code"><code>${p1.replace(/\n$/,'')}</code></pre>`;
  });
  // 标题 # ## ###
  html = html.replace(/^###\s+(.+)$/gm, '<h3>$1</h3>');
  html = html.replace(/^##\s+(.+)$/gm, '<h2>$1</h2>');
  html = html.replace(/^#\s+(.+)$/gm, '<h1>$1</h1>');
  // 粗体/斜体/行内代码
  html = html.replace(/\*\*(.+?)\*\*/g, '<strong>$1</strong>');
  html = html.replace(/\*(.+?)\*/g, '<em>$1</em>');
  html = html.replace(/`([^`]+?)`/g, '<code class="md-inline-code">$1</code>');
  // 列表（简单处理）
  html = html.replace(/^(?:-\s+.+|\d+\.\s+.+)(?:\n(?:-\s+.+|\d+\.\s+.+))*$/gm, (block: string) => {
    const lines = block.split(/\n/);
    const isOrdered = /^\d+\./.test(lines[0]);
    const items = lines
      .map(line => line.replace(/^(-|\d+\.)\s+/, ''))
      .map(item => `<li>${item}</li>`) 
      .join('');
    return isOrdered ? `<ol>${items}</ol>` : `<ul>${items}</ul>`;
  });
  // 段落
  html = html
    .split(/\n{2,}/)
    .map(blk => /<(h\d|ul|ol|pre)/.test(blk) ? blk : `<p>${blk.replace(/\n/g, '<br/>')}</p>`)
    .join('');
  return html;
}

const renderedContent = computed(() => renderMarkdown(content.value || ''));
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
        <div class="mode-toggle no-drag">
          <button :class="['toggle-btn', { active: isMdMode }]" @click="isMdMode = true">MD</button>
          <button :class="['toggle-btn', { active: !isMdMode }]" @click="isMdMode = false">源码</button>
        </div>
        <button class="btn-save no-drag" @click="saveNote" :disabled="isSaving">
          {{ isSaving ? '保存中...' : '保存 (Ctrl+S)' }}
        </button>
      </div>
    </div>

    <!-- 文本编辑器 -->
    <div class="editor-container">
      <div v-if="isMdMode" class="md-split">
        <textarea
          v-model="content"
          class="editor no-drag"
          placeholder="开始输入你的笔记..."
          @input="saveStatus = ''"
        ></textarea>
        <div class="preview" v-html="renderedContent"></div>
      </div>
      <div v-else class="source-only">
        <textarea
          v-model="content"
          class="editor no-drag"
          placeholder="开始输入你的笔记..."
          @input="saveStatus = ''"
        ></textarea>
      </div>
    </div>
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

.btn-save {
  padding: 8px 16px;
  background-color: var(--color-accent);
  color: #fff;
  border: none;
  border-radius: var(--radius-s);
  cursor: pointer;
  font-size: 14px;
  transition: background var(--tr-fast), transform var(--tr-fast);
  white-space: nowrap;
  -webkit-app-region: no-drag;
}

.btn-save:hover:not(:disabled) {
  background-color: var(--color-accent-hover);
}

.btn-save:disabled {
  background-color: var(--color-border);
  color: var(--color-text-muted);
  cursor: not-allowed;
}

.editor-container {
  flex: 1;
  overflow: hidden;
  background-color: var(--color-surface);
}

.md-split {
  display: flex;
  height: 100%;
}

.md-split .editor {
  width: 50%;
  border-right: 1px solid var(--color-border);
}

.md-split .preview {
  width: 50%;
}

.source-only, .source-only .editor {
  height: 100%;
}

.editor {
  width: 100%;
  height: 100%;
  padding: 20px;
  border: none;
  outline: none;
  font-family: 'Consolas', 'Monaco', 'Courier New', monospace;
  font-size: 15px;
  line-height: 1.6;
  resize: none;
  background-color: var(--color-surface);
  color: var(--color-text);
  caret-color: var(--color-accent);
}

.editor::placeholder {
  color: var(--color-text-muted);
}

.preview {
  width: 100%;
  height: 100%;
  padding: 24px;
  overflow-y: auto;
  background: var(--color-surface);
  color: var(--color-text);
}

.preview h1,
.preview h2,
.preview h3 {
  margin: 12px 0 8px;
  font-weight: 700;
}
.preview h1 { font-size: 24px; }
.preview h2 { font-size: 20px; }
.preview h3 { font-size: 18px; }

.preview p { margin: 8px 0; }
.preview ul, .preview ol { margin: 8px 0 8px 18px; }
.preview code.md-inline-code {
  background: var(--color-surface-2);
  border: 1px solid var(--color-border);
  padding: 1px 6px;
  border-radius: 6px;
  font-family: 'Consolas', 'Monaco', 'Courier New', monospace;
}
.preview pre.md-code {
  background: var(--color-surface-2);
  border: 1px solid var(--color-border);
  padding: 12px;
  border-radius: 10px;
  overflow: auto;
}

.mode-toggle {
  display: inline-flex;
  border: 1px solid var(--color-border);
  border-radius: 999px;
  overflow: hidden;
}
.toggle-btn {
  padding: 6px 12px;
  background: transparent;
  border: none;
  color: var(--color-text-muted);
  cursor: pointer;
}
.toggle-btn.active {
  background: var(--color-surface-2);
  color: var(--color-text);
}

.no-drag { -webkit-app-region: no-drag; }
</style>

