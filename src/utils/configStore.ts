import { Store } from '@tauri-apps/plugin-store';

// 配置接口
export interface AppConfig {
  fontSize: number;
  fontFamily: string;
  lineHeight: number;
  autoStart: boolean;
  searchDirectories: string[];
  defaultNotesLocation: string;
  notes: string[]; // 所有笔记的完整路径列表
  theme: 'light' | 'dark' | 'system'; // 主题模式
}

// 默认配置
const DEFAULT_CONFIG: AppConfig = {
  fontSize: 16,
  fontFamily: "Consolas, 'Courier New', monospace",
  lineHeight: 1.6,
  autoStart: false,
  searchDirectories: [],
  defaultNotesLocation: '', // 将在初始化时设置为 .ytools 目录
  notes: [],
  theme: 'system' // 默认跟随系统主题
};

// 创建 store 实例（使用 Store.load 加载）
let store: Store | null = null;

// 获取或创建 store 实例
async function getStore(): Promise<Store> {
  if (!store) {
    store = await Store.load('config.json', {
      defaults: DEFAULT_CONFIG as unknown as Record<string, unknown>,
      autoSave: true // 自动保存，100ms 防抖
    });
  }
  return store;
}

// 初始化 store（确保连接建立）
export async function initConfig(): Promise<void> {
  try {
    const storeInstance = await getStore();
    // 检查是否需要从旧配置迁移
    await migrateOldConfig();
    
    // 如果 defaultNotesLocation 为空，则设置为 .ytools 目录
    const defaultLocation = await storeInstance.get('defaultNotesLocation');
    if (!defaultLocation) {
      const { appConfigDir } = await import('@tauri-apps/api/path');
      const { join } = await import('@tauri-apps/api/path');
      const homeDir = await appConfigDir();
      await storeInstance.set('defaultNotesLocation', await join(homeDir, '.ytools'));
      await storeInstance.save();
    }
  } catch (error) {
    console.error('Failed to initialize config store:', error);
  }
}

// 迁移旧配置文件（兼容性处理）
async function migrateOldConfig(): Promise<void> {
  try {
    const storeInstance = await getStore();
    
    // 检查是否已迁移
    const migrated = await storeInstance.get<boolean>('_migrated');
    if (migrated) return;

    // 尝试读取旧的配置文件
    const { readTextFile, exists, BaseDirectory } = await import('@tauri-apps/plugin-fs');
    
    const oldConfigExists = await exists('.ytools/config.json', { 
      baseDir: BaseDirectory.Home 
    });

    if (oldConfigExists) {
      const oldConfigContent = await readTextFile('.ytools/config.json', {
        baseDir: BaseDirectory.Home
      });
      const oldConfig = JSON.parse(oldConfigContent);
      
      // 迁移字体大小
      if (oldConfig.font_size) {
        await storeInstance.set('fontSize', oldConfig.font_size);
      }
      
      console.log('✓ Old config migrated successfully');
    }

    // 标记为已迁移
    await storeInstance.set('_migrated', true);
    await storeInstance.save();
  } catch (error) {
    console.warn('Old config migration skipped:', error);
  }
}

// 获取字体大小
export async function getFontSize(): Promise<number> {
  const storeInstance = await getStore();
  const size = await storeInstance.get<number>('fontSize');
  return size ?? DEFAULT_CONFIG.fontSize;
}

// 设置字体大小
export async function setFontSize(size: number): Promise<void> {
  const storeInstance = await getStore();
  await storeInstance.set('fontSize', size);
  // autoSave 模式下会自动保存
}

// 获取完整配置（并行读取优化）
export async function getConfig(): Promise<AppConfig> {
  // 并行读取所有配置项，显著提升加载速度
  const [
    fontSize,
    fontFamily,
    lineHeight,
    autoStart,
    searchDirectories,
    defaultNotesLocation,
    notes,
    theme
  ] = await Promise.all([
    getFontSize(),
    getFontFamily(),
    getLineHeight(),
    getAutoStart(),
    getSearchDirectories(),
    getDefaultNotesLocation(),
    getNotes(),
    getTheme()
  ]);
  
  return {
    fontSize,
    fontFamily,
    lineHeight,
    autoStart,
    searchDirectories,
    defaultNotesLocation,
    notes,
    theme: theme as 'light' | 'dark' | 'system'
  };
}

// 重置配置到默认值
export async function resetConfig(): Promise<void> {
  const storeInstance = await getStore();
  await storeInstance.clear(); // 清空所有配置
  // 重新设置默认值
  await storeInstance.set('fontSize', DEFAULT_CONFIG.fontSize);
  await storeInstance.set('fontFamily', DEFAULT_CONFIG.fontFamily);
  await storeInstance.set('lineHeight', DEFAULT_CONFIG.lineHeight);
  await storeInstance.set('autoStart', DEFAULT_CONFIG.autoStart);
  await storeInstance.set('searchDirectories', DEFAULT_CONFIG.searchDirectories);
  await storeInstance.set('defaultNotesLocation', DEFAULT_CONFIG.defaultNotesLocation);
  await storeInstance.set('notes', DEFAULT_CONFIG.notes);
  await storeInstance.set('theme', DEFAULT_CONFIG.theme);
  await storeInstance.set('_migrated', true); // 保持迁移标记
  await storeInstance.save();
}

// 监听配置变化
export async function onFontSizeChange(callback: (size: number) => void) {
  const storeInstance = await getStore();
  return await storeInstance.onKeyChange('fontSize', (value: number | undefined) => {
    if (value !== undefined) {
      callback(value);
    }
  });
}

// 获取字体族
export async function getFontFamily(): Promise<string> {
  const storeInstance = await getStore();
  const family = await storeInstance.get<string>('fontFamily');
  return family ?? DEFAULT_CONFIG.fontFamily;
}

// 设置字体族
export async function setFontFamily(family: string): Promise<void> {
  const storeInstance = await getStore();
  await storeInstance.set('fontFamily', family);
}

// 获取行高
export async function getLineHeight(): Promise<number> {
  const storeInstance = await getStore();
  const height = await storeInstance.get<number>('lineHeight');
  return height ?? DEFAULT_CONFIG.lineHeight;
}

// 设置行高
export async function setLineHeight(height: number): Promise<void> {
  const storeInstance = await getStore();
  await storeInstance.set('lineHeight', height);
}

// 获取开机启动状态
export async function getAutoStart(): Promise<boolean> {
  const storeInstance = await getStore();
  const autoStart = await storeInstance.get<boolean>('autoStart');
  return autoStart ?? DEFAULT_CONFIG.autoStart;
}

// 设置开机启动状态
export async function setAutoStart(enabled: boolean): Promise<void> {
  const storeInstance = await getStore();
  await storeInstance.set('autoStart', enabled);
}

// 获取搜索目录
export async function getSearchDirectories(): Promise<string[]> {
  const storeInstance = await getStore();
  const dirs = await storeInstance.get<string[]>('searchDirectories');
  return dirs ?? DEFAULT_CONFIG.searchDirectories;
}

// 设置搜索目录
export async function setSearchDirectories(directories: string[]): Promise<void> {
  const storeInstance = await getStore();
  await storeInstance.set('searchDirectories', directories);
}

// 添加搜索目录
export async function addSearchDirectory(directory: string): Promise<void> {
  const dirs = await getSearchDirectories();
  if (!dirs.includes(directory)) {
    dirs.push(directory);
    await setSearchDirectories(dirs);
  }
}

// 删除搜索目录
export async function removeSearchDirectory(directory: string): Promise<void> {
  const dirs = await getSearchDirectories();
  const filtered = dirs.filter(d => d !== directory);
  await setSearchDirectories(filtered);
}

// 获取默认笔记位置
export async function getDefaultNotesLocation(): Promise<string> {
  const storeInstance = await getStore();
  const location = await storeInstance.get<string>('defaultNotesLocation');
  return location ?? DEFAULT_CONFIG.defaultNotesLocation;
}

// 设置默认笔记位置
export async function setDefaultNotesLocation(location: string): Promise<void> {
  const storeInstance = await getStore();
  await storeInstance.set('defaultNotesLocation', location);
}

// 获取笔记列表
export async function getNotes(): Promise<string[]> {
  const storeInstance = await getStore();
  const notes = await storeInstance.get<string[]>('notes');
  return notes ?? DEFAULT_CONFIG.notes;
}

// 设置笔记列表
export async function setNotes(notes: string[]): Promise<void> {
  const storeInstance = await getStore();
  await storeInstance.set('notes', notes);
}

// 添加笔记
export async function addNote(path: string): Promise<void> {
  const notes = await getNotes();
  if (!notes.includes(path)) {
    notes.push(path);
    await setNotes(notes);
  }
}

// 删除笔记
export async function removeNote(path: string): Promise<void> {
  const notes = await getNotes();
  const filtered = notes.filter(n => n !== path);
  await setNotes(filtered);
}

// 获取主题
export async function getTheme(): Promise<string> {
  const storeInstance = await getStore();
  const theme = await storeInstance.get<string>('theme');
  return theme ?? DEFAULT_CONFIG.theme;
}

// 设置主题
export async function setTheme(theme: string): Promise<void> {
  const storeInstance = await getStore();
  await storeInstance.set('theme', theme);
}

