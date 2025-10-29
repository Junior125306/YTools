<script setup lang="ts">
import { ref, onMounted, onUnmounted, nextTick, computed } from 'vue'
import { invoke } from '@tauri-apps/api/core'
import { getCurrentWindow } from '@tauri-apps/api/window'
import { 
  NCard, NInput, NList, NListItem, NEmpty, NSpace, NTag, NIcon, NText, useThemeVars
} from 'naive-ui'
import { SearchOutline, FolderOutline } from '@vicons/ionicons5'
import { getSearchDirectories } from '../utils/configStore'
import { useTheme } from '../composables/useTheme'

const themeVars = useThemeVars()
// 初始化主题并监听主题变更
const { themeMode } = useTheme()

// 检查是否是赛博朋克主题
const isCyberpunk = computed(() => themeMode.value === 'cyberpunk')

// ==== 主题颜色变量 ====
const primaryColor = computed(() => '#5ccfe6')
const primaryColorLight = computed(() => '#6fdbf0')
const primaryColorDark = computed(() => '#4fb3c9')
const primaryColorAlpha = (alpha: number) => `rgba(92, 207, 230, ${alpha})`

const searchInput = ref<any>(null)
const query = ref('')
const searchResults = ref<string[]>([])
const selectedIndex = ref(0)
const currentWindow = getCurrentWindow()
const isOpening = ref(false)
const hasSearchDirectories = ref(true)
const resultRefs = ref<HTMLElement[]>([])

// 搜索函数
async function performSearch(searchQuery: string) {
  try {
    const directories = await getSearchDirectories()
    
    if (directories.length === 0) {
      hasSearchDirectories.value = false
      searchResults.value = []
      resultRefs.value = []
      return
    }
    
    hasSearchDirectories.value = true
    
    searchResults.value = await invoke<string[]>('search_workspaces', { 
      query: searchQuery || '',
      directories
    })
    selectedIndex.value = 0
    resultRefs.value = []
  } catch (error) {
    console.error('搜索失败:', error)
    searchResults.value = []
    resultRefs.value = []
  }
}

// 打开文件夹
async function openFolder(folderName: string) {
  if (isOpening.value) return
  isOpening.value = true
  try {
    await currentWindow.hide().catch(() => {})
    query.value = ''
    searchResults.value = []
    await invoke('open_folder', { folder_name: folderName, folderName: folderName })
  } catch (error) {
    console.error('打开文件夹失败:', error)
    await currentWindow.hide().catch(() => {})
  }
  finally {
    setTimeout(() => {
      isOpening.value = false
    }, 50)
  }
}

// 处理输入
function handleInput() {
  performSearch(query.value)
}

// 处理键盘事件
async function handleKeydown(e: KeyboardEvent) {
  if (e.key === 'Escape') {
    e.preventDefault()
    await currentWindow.hide()
    query.value = ''
    searchResults.value = []
  } else if (e.key === 'ArrowDown') {
    e.preventDefault()
    if (searchResults.value.length > 0) {
      selectedIndex.value = (selectedIndex.value + 1) % searchResults.value.length
      await scrollToSelected()
    }
  } else if (e.key === 'ArrowUp') {
    e.preventDefault()
    if (searchResults.value.length > 0) {
      selectedIndex.value = (selectedIndex.value - 1 + searchResults.value.length) % searchResults.value.length
      await scrollToSelected()
    }
  } else if (e.key === 'Enter') {
    e.preventDefault()
    if (searchResults.value.length > 0 && searchResults.value[selectedIndex.value]) {
      await openFolder(searchResults.value[selectedIndex.value])
    }
  }
}

// 滚动到选中的项目
async function scrollToSelected() {
  await nextTick()
  const selectedElement = resultRefs.value[selectedIndex.value]
  if (selectedElement) {
    selectedElement.scrollIntoView({
      behavior: 'smooth',
      block: 'nearest'
    })
  }
}

// 选择项目
function selectItem(index: number) {
  selectedIndex.value = index
}

// 点击项目
async function clickItem(index: number) {
  if (index >= 0 && index < searchResults.value.length) {
    await openFolder(searchResults.value[index])
  }
}

// 设置结果项的 ref
function setResultRef(el: any, index: number) {
  if (el) {
    resultRefs.value[index] = el.$el || el
  }
}

// 点击背景关闭
async function handleBackgroundClick(e: MouseEvent) {
  if ((e.target as HTMLElement).classList.contains('search-window')) {
    await currentWindow.hide()
    query.value = ''
    searchResults.value = []
  }
}

// 初始化
onMounted(async () => {
  await performSearch('')
  
  // 自动聚焦到搜索输入框
  await nextTick()
  if (searchInput.value) {
    searchInput.value.focus()
  }
  
  document.addEventListener('keydown', handleKeydown, { capture: true } as AddEventListenerOptions)
  
  currentWindow.listen('tauri://focus', async () => {
    query.value = ''
    await performSearch('')
    // 窗口获得焦点时也自动聚焦输入框
    await nextTick()
    if (searchInput.value) {
      searchInput.value.focus()
    }
  })
})

onUnmounted(() => {
  document.removeEventListener('keydown', handleKeydown, { capture: true } as AddEventListenerOptions)
})
</script>

<template>
  <div class="search-window" :class="{ 'cyberpunk-mode': isCyberpunk }" @click="handleBackgroundClick">
    <NCard 
      class="search-container" 
      :bordered="false"
      :segmented="{ content: true, footer: true }"
      content-style="padding: 0; display: flex; flex-direction: column; height: 100%;"
    >
      <template #header>
        <NInput
          ref="searchInput"
          v-model:value="query"
          placeholder="搜索工作区文件夹..."
          size="large"
          clearable
          @input="handleInput"
        >
          <template #prefix>
            <NIcon size="20"><SearchOutline /></NIcon>
          </template>
        </NInput>
      </template>
      
      <div class="results-container">
        <NEmpty v-if="!hasSearchDirectories" description="未配置搜索目录">
          <template #icon>
            <NIcon size="48"><FolderOutline /></NIcon>
          </template>
          <template #extra>
            <NText depth="3" style="font-size: 13px">
              请在设置中添加搜索目录后使用此功能
            </NText>
          </template>
        </NEmpty>
        
        <NEmpty v-else-if="searchResults.length === 0" description="未找到匹配的文件夹" />
        
        <NList v-else hoverable clickable style="height: 100%; overflow-y: auto;">
          <NListItem
            v-for="(result, index) in searchResults"
            :key="result"
            :ref="(el: any) => setResultRef(el, index)"
            :class="{ 'selected-item': index === selectedIndex }"
            @click="clickItem(index)"
            @mouseenter="selectItem(index)"
          >
            <template #prefix>
              <NIcon size="24" color="#818cf8"><FolderOutline /></NIcon>
            </template>
            <NText>{{ result }}</NText>
          </NListItem>
        </NList>
      </div>
      
      <template #footer>
        <NSpace justify="center" size="large">
          <NTag type="info" size="small" :bordered="false">
            <template #icon>
              <span style="font-family: monospace">↑↓</span>
            </template>
            导航
          </NTag>
          <NTag type="success" size="small" :bordered="false">
            <template #icon>
              <span style="font-family: monospace">Enter</span>
            </template>
            打开
          </NTag>
          <NTag type="warning" size="small" :bordered="false">
            <template #icon>
              <span style="font-family: monospace">Esc</span>
            </template>
            关闭
          </NTag>
        </NSpace>
      </template>
    </NCard>
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
  backdrop-filter: blur(10px);
  background: transparent;
}

.search-container {
  width: min(600px, 85vw);
  height: min(500px, 70vh);
  display: flex;
  flex-direction: column;
  box-shadow: 0 12px 32px rgba(0, 0, 0, 0.3);
  border-radius: 12px;
  overflow: hidden;
}

.search-container :deep(.n-card__content) {
  flex: 1;
  display: flex;
  flex-direction: column;
  overflow: hidden;
}

.results-container {
  flex: 1;
  overflow: hidden;
  display: flex;
  flex-direction: column;
}

/* 选中项样式 - 去除圆角，填满整个区域 */
.selected-item {
  background-color: v-bind('themeVars.primaryColorSuppl + "30"') !important;
  border-radius: 0 !important;
}

/* 覆盖 NList 和 NListItem 的所有圆角 */
.search-container :deep(.n-list) {
  border-radius: 0 !important;
}

.search-container :deep(.n-list-item) {
  border-radius: 0 !important;
}

.search-container :deep(.n-list-item__main) {
  border-radius: 0 !important;
}

.search-container :deep(.n-list-item:first-child),
.search-container :deep(.n-list-item:last-child) {
  border-radius: 0 !important;
}

/* 悬停样式也去除圆角 */
.search-container :deep(.n-list-item:hover) {
  border-radius: 0 !important;
}

/* 细滚动条样式 */
.search-container :deep(.n-list)::-webkit-scrollbar {
  width: 6px;
}

.search-container :deep(.n-list)::-webkit-scrollbar-track {
  background: transparent;
}

.search-container :deep(.n-list)::-webkit-scrollbar-thumb {
  background: v-bind('themeVars.scrollbarColor');
  border-radius: 3px;
}

.search-container :deep(.n-list)::-webkit-scrollbar-thumb:hover {
  background: v-bind('themeVars.scrollbarColorHover');
}

/* 结果容器滚动条 */
.results-container::-webkit-scrollbar {
  width: 6px;
}

.results-container::-webkit-scrollbar-track {
  background: transparent;
}

.results-container::-webkit-scrollbar-thumb {
  background: v-bind('themeVars.scrollbarColor');
  border-radius: 3px;
}

.results-container::-webkit-scrollbar-thumb:hover {
  background: v-bind('themeVars.scrollbarColorHover');
}

/* 赛博朋克模式滚动条霓虹效果 */
.search-window.cyberpunk-mode .search-container :deep(.n-list)::-webkit-scrollbar-thumb {
  background: linear-gradient(180deg, 
    v-bind('primaryColor') 0%, 
    v-bind('primaryColorDark') 50%, 
    v-bind('primaryColor') 100%
  );
  border-radius: 3px;
  box-shadow: 
    0 0 8px v-bind('primaryColorAlpha(0.8)'),
    0 0 16px v-bind('primaryColorAlpha(0.5)');
}

.search-window.cyberpunk-mode .search-container :deep(.n-list)::-webkit-scrollbar-thumb:hover {
  background: linear-gradient(180deg, 
    v-bind('primaryColorLight') 0%, 
    v-bind('primaryColor') 50%, 
    v-bind('primaryColorLight') 100%
  );
  box-shadow: 
    0 0 12px v-bind('primaryColorAlpha(1)'),
    0 0 24px v-bind('primaryColorAlpha(0.8)'),
    0 0 36px v-bind('primaryColorAlpha(0.5)');
}

.search-window.cyberpunk-mode .search-container :deep(.n-list)::-webkit-scrollbar-track {
  background: v-bind('primaryColorAlpha(0.05)');
  box-shadow: inset 0 0 6px v-bind('primaryColorAlpha(0.2)');
}

.search-window.cyberpunk-mode .results-container::-webkit-scrollbar-thumb {
  background: linear-gradient(180deg, 
    v-bind('primaryColor') 0%, 
    v-bind('primaryColorDark') 50%, 
    v-bind('primaryColor') 100%
  );
  border-radius: 3px;
  box-shadow: 
    0 0 8px v-bind('primaryColorAlpha(0.8)'),
    0 0 16px v-bind('primaryColorAlpha(0.5)');
}

.search-window.cyberpunk-mode .results-container::-webkit-scrollbar-thumb:hover {
  background: linear-gradient(180deg, 
    v-bind('primaryColorLight') 0%, 
    v-bind('primaryColor') 50%, 
    v-bind('primaryColorLight') 100%
  );
  box-shadow: 
    0 0 12px v-bind('primaryColorAlpha(1)'),
    0 0 24px v-bind('primaryColorAlpha(0.8)'),
    0 0 36px v-bind('primaryColorAlpha(0.5)');
}

.search-window.cyberpunk-mode .results-container::-webkit-scrollbar-track {
  background: v-bind('primaryColorAlpha(0.05)');
  box-shadow: inset 0 0 6px v-bind('primaryColorAlpha(0.2)');
}
</style>

<style>
/* 搜索窗口专用：强制透明背景 */
body:has(.search-window) {
  background: transparent !important;
}

html:has(.search-window) {
  background: transparent !important;
}

#app:has(.search-window) {
  background: transparent !important;
}

/* 搜索窗口的 app-wrapper 也要透明 */
.app-wrapper:has(.search-window) {
  background: transparent !important;
}
</style>

