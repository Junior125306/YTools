<script setup lang="ts">
import { ref, onMounted, onUnmounted } from 'vue';
import { invoke } from '@tauri-apps/api/core';
import { getCurrentWindow } from '@tauri-apps/api/window';

const searchInput = ref<HTMLInputElement | null>(null);
const query = ref('');
const searchResults = ref<string[]>([]);
const selectedIndex = ref(0);
const currentWindow = getCurrentWindow();
const isOpening = ref(false); // 防止回车重复触发打开

// 搜索函数
async function performSearch(searchQuery: string) {
  try {
    // 无论 query 是否为空，都调用后端搜索
    // 后端会处理：空字符串返回所有文件夹，有内容则过滤匹配的文件夹
    searchResults.value = await invoke<string[]>('search_workspaces', { 
      query: searchQuery || '' 
    });
    selectedIndex.value = 0;
  } catch (error) {
    console.error('搜索失败:', error);
    searchResults.value = [];
  }
}

// 打开文件夹
async function openFolder(folderName: string) {
  if (isOpening.value) return; // 防抖：已在打开中则忽略
  isOpening.value = true;
  try {
    // 先尝试隐藏窗口，提供更快的反馈（无权限时忽略错误）
    await currentWindow.hide().catch(() => {});
    
    // 清空状态
    query.value = '';
    searchResults.value = [];
    
    // 然后打开文件夹（同时传 snake_case 与 camelCase 以兼容参数名解析）
    await invoke('open_folder', { folder_name: folderName, folderName: folderName });
  } catch (error) {
    console.error('打开文件夹失败:', error);
    // 即使出错也要尝试隐藏窗口（忽略错误）
    await currentWindow.hide().catch(() => {});
  }
  finally {
    // 短暂延迟，避免同一次按键冒泡导致的双触发
    setTimeout(() => {
      isOpening.value = false;
    }, 50);
  }
}

// 处理输入
function handleInput() {
  performSearch(query.value);
}

// 处理键盘事件
async function handleKeydown(e: KeyboardEvent) {
  if (e.key === 'Escape') {
    await currentWindow.hide();
    query.value = '';
    searchResults.value = [];
  } else if (e.key === 'ArrowDown') {
    e.preventDefault();
    if (searchResults.value.length > 0) {
      selectedIndex.value = (selectedIndex.value + 1) % searchResults.value.length;
    }
  } else if (e.key === 'ArrowUp') {
    e.preventDefault();
    if (searchResults.value.length > 0) {
      selectedIndex.value = (selectedIndex.value - 1 + searchResults.value.length) % searchResults.value.length;
    }
  } else if (e.key === 'Enter') {
    e.preventDefault();
    if (searchResults.value.length > 0 && searchResults.value[selectedIndex.value]) {
      openFolder(searchResults.value[selectedIndex.value]);
    }
  }
}

// 选择项目
function selectItem(index: number) {
  selectedIndex.value = index;
}

// 点击项目
function clickItem(index: number) {
  openFolder(searchResults.value[index]);
}

// 点击背景关闭
async function handleBackgroundClick(e: MouseEvent) {
  if ((e.target as HTMLElement).classList.contains('search-window')) {
    await currentWindow.hide();
    query.value = '';
    searchResults.value = [];
  }
}

// 初始化
onMounted(async () => {
  // 页面加载时初始化，显示所有文件夹
  await performSearch('');
  
  // 聚焦输入框
  searchInput.value?.focus();
  
  // 全局监听键盘事件（确保 ESC/Enter 始终有效）
  // 仅在捕获阶段监听一次，避免重复与冒泡触发两次
  document.addEventListener('keydown', handleKeydown, { capture: true } as AddEventListenerOptions);
  
  // 监听窗口显示事件
  currentWindow.listen('tauri://focus', () => {
    searchInput.value?.focus();
    query.value = '';
    performSearch('');
  });
});

// 组件卸载时清理事件监听器
onUnmounted(() => {
  document.removeEventListener('keydown', handleKeydown, { capture: true } as AddEventListenerOptions);
});
</script>

<template>
  <div class="search-window" @click="handleBackgroundClick">
    <div class="search-container">
      <input
        ref="searchInput"
        v-model="query"
        type="text"
        class="search-input"
        placeholder="搜索工作区文件夹..."
        @input="handleInput"
      />
      
      <div class="results-container">
        <div v-if="searchResults.length === 0" class="search-empty">
          未找到匹配的文件夹
        </div>
        
        <div v-else class="search-results">
          <div
            v-for="(result, index) in searchResults"
            :key="result"
            :class="['search-result-item', { selected: index === selectedIndex }]"
            @click="clickItem(index)"
            @mouseenter="selectItem(index)"
          >
            <div class="folder-icon">📁</div>
            <div class="folder-name">{{ result }}</div>
          </div>
        </div>
      </div>
      
      <div class="search-footer">
        <div class="shortcut-hint">
          <span><kbd>↑↓</kbd> 导航</span>
          <span><kbd>Enter</kbd> 打开</span>
          <span><kbd>Esc</kbd> 关闭</span>
        </div>
      </div>
    </div>
  </div>
</template>

<style scoped>
.search-window {
  width: 100vw;
  height: 100vh;
  display: flex;
  align-items: center;
  justify-content: center;
  padding: 24px;
  position: fixed;
  top: 0;
  left: 0;
  backdrop-filter: blur(10px);
}

.search-container {
  width: min(720px, 92vw);
  height: min(70vh, 720px);
  background: var(--color-surface);
  border: 1px solid var(--color-border);
  border-radius: var(--radius-l);
  box-shadow: var(--shadow-md);
  overflow: hidden;
  display: flex;
  flex-direction: column;
}

@keyframes slideIn {
  from {
    opacity: 0;
    transform: translateY(-20px) scale(0.95);
  }
  to {
    opacity: 1;
    transform: translateY(0) scale(1);
  }
}

.search-input {
  width: 100%;
  padding: 16px 20px;
  font-size: 16px;
  border: none;
  background: var(--color-surface);
  color: var(--color-text);
  outline: none;
  border-bottom: 1px solid var(--color-border);
}

.search-input::placeholder {
  color: var(--color-text-muted);
}

.results-container {
  flex: 1;  /* 占据剩余空间 */
  overflow-y: auto;
  background: var(--color-surface);
}

.results-container::-webkit-scrollbar {
  width: 8px;
}

.results-container::-webkit-scrollbar-track {
  background: transparent;
}

.results-container::-webkit-scrollbar-thumb {
  background: color-mix(in srgb, var(--color-text) 12%, transparent);
  border-radius: 4px;
}

.results-container::-webkit-scrollbar-thumb:hover {
  background: color-mix(in srgb, var(--color-text) 20%, transparent);
}

.search-results {
  display: flex;
  flex-direction: column;
}

.search-result-item {
  display: flex;
  align-items: center;
  padding: 12px 20px;
  cursor: pointer;
  transition: background var(--tr-fast), color var(--tr-fast);
  color: var(--color-text);
}

.search-result-item:hover {
  background: var(--color-surface-2);
}

.search-result-item.selected {
  background: color-mix(in srgb, var(--color-accent) 10%, var(--color-surface));
}

.folder-icon {
  font-size: 18px;
  margin-right: 12px;
}

.folder-name {
  font-size: 14px;
  flex: 1;
}

.search-empty {
  padding: 40px 24px;
  text-align: center;
  color: var(--color-text-muted);
  font-size: 14px;
}

.search-footer {
  padding: 12px 20px;
  border-top: 1px solid var(--color-border);
  display: flex;
  justify-content: space-between;
  font-size: 12px;
  color: var(--color-text-muted);
  background: var(--color-surface);
}

.shortcut-hint {
  display: flex;
  gap: 16px;
}

.shortcut-hint span {
  display: flex;
  align-items: center;
  gap: 4px;
}

kbd {
  background: var(--color-surface-2);
  border: 1px solid var(--color-border);
  padding: 2px 6px;
  border-radius: 4px;
  font-size: 11px;
  font-family: monospace;
}
</style>

<style>
/* 为搜索窗口设置透明背景 */
body:has(.search-window) {
  background: transparent !important;
  border: none !important;
  outline: none !important;
  margin: 0 !important;
  padding: 0 !important;
}

html:has(.search-window) {
  background: transparent !important;
  border: none !important;
  outline: none !important;
  margin: 0 !important;
  padding: 0 !important;
}

#app:has(.search-window) {
  background: transparent !important;
  border: none !important;
  outline: none !important;
}
</style>

