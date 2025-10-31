# Changelog

本文档记录 YTools 的所有重要变更。

格式基于 [Keep a Changelog](https://keepachangelog.com/zh-CN/1.0.0/)，
版本号遵循 [语义化版本](https://semver.org/lang/zh-CN/)。

## [Unreleased]

### 计划中

- 快捷图标访问面板
- 快捷指令系统
- JSON 格式化工具
- 远程桌面快捷方式

## [0.5.1] - 2025-10-31

### Changed

- ♻️ **代码重构**：将 Rust 代码模块化拆分，提升代码可维护性和组织结构

## [0.5.0] - 2025-10-29

### Added (新增)

- ✨ **快捷键自定义**：可在设置中自定义主窗口和搜索窗口的快捷键，支持冲突检测和一键重置

### Changed (变更)

- ♻️ 设置窗口宽度扩展到 410px，优化布局和主题样式细节

## [0.4.2] - 2025-10-29

### Added (新增)

- ✨ **赛博朋克主题（Cyberpunk Theme）**
  - 新增 Cyberpunk 主题选项，灵感来自 Halcyon VSCode 主题
  - 青色（Cyan）霓虹主色调，配合黄色（Yellow）和红色（Red）点缀
  - 深邃暗色背景（#0f1419）增强霓虹对比效果
  - 全局霓虹发光效果：按钮、输入框、滚动条、标签页等
  - 统一的柔和浅灰白色文字（#d7dce2）
  - 动态霓虹脉冲动画，营造赛博朋克氛围

### Changed (变更)

- ♻️ **主题选择器优化**
  - 设置页面采用四宫格卡片式主题选择器
  - 每个主题配有可视化预览效果（亮色、暗色、赛博朋克、跟随系统）
  - 支持点击选择，激活时显示青色霓虹边框和勾选标记
  - 优化设置窗口尺寸（310x650）以适配新的主题选择器
- ♻️ **赛博朋克模式下的视觉增强**
  - HomeView: Tab 标签霓虹发光、设置/导入按钮特殊配色
  - SearchView: 青色霓虹滚动条，渐变发光效果
  - SettingsView: 分隔线霓虹脉冲、按钮发光强化

### Technical (技术)

- 扩展 ThemeMode 类型支持 'cyberpunk' 选项
- 新增 `cyberpunkThemeOverrides` 主题覆盖配置（180+ 行样式定义）
- 添加 `isCyberpunk` 计算属性用于条件样式绑定
- 新增多个赛博朋克专用 CSS 类和动画关键帧

## [0.4.1] - 2025-10-29

### Fixed (修复)

- 🐛 修复配置对象缺失字段（notes、theme）导致的潜在错误
- 🐛 修复未使用参数命名规范问题，避免 TypeScript 编译警告
- 🐛 统一产品名称大小写（ytools → YTools），保持品牌一致性

## [0.4.0] - 2025-10-28

### Added (新增)

- ✨ **主题系统**
  - 支持亮色/暗色主题切换
  - 支持跟随系统主题自动切换
  - 新增主题配置中心（设置页面）
  - 实时主题切换，无需重启应用
- ✨ **优化的配色方案**
  - 亮色主题：柔和的米白色背景（stone-100/stone-50）
  - 暗色主题：深色护眼配色（Naive UI 默认暗色）
  - 统一的紫色主调（violet 系列）

### Changed (变更)

- ♻️ **全面主题化重构**
  - 所有视图和组件支持主题切换（HomeView、SearchView、SettingsView）
  - 新增 `useTheme` composable 统一管理主题状态
  - 主题配置持久化存储并在所有窗口间同步
  - App.vue 使用主题背景色替代硬编码颜色
- ♻️ **搜索界面优化**
  - 选中项背景无圆角，避免底色露出
  - 细滚动条样式（6px 宽度）
  - 搜索窗口保持透明背景浮动效果

### Technical (技术)

- 新增 `src/composables/useTheme.ts` 主题管理模块
- 分离亮色和暗色主题覆盖配置
- 使用 Naive UI `useThemeVars` 实现动态主题绑定
- 主题变更事件广播到所有窗口

## [0.3.0] - 2025-01-28

### Added (新增)

- ✨ **统一笔记管理系统**
  - 支持导入外部 `.md` 和 `.txt` 文件到笔记列表
  - 所有笔记统一管理，混合显示内置和外部文件
  - 删除笔记时可选择永久删除文件或仅从列表移除
  - 使用完整路径存储，支持同名文件共存
  - 添加导入按钮（📁）快速导入现有文件
- ✨ **设置中心**
  - 独立设置窗口，支持托盘菜单和主窗口按钮打开
  - 编辑器设置：字体大小、字体族、行高配置
  - 通用设置：开机启动开关
  - 搜索设置：可配置的搜索目录列表
  - 笔记设置：默认笔记保存位置配置
  - 支持重置为默认设置
- ✨ **拼音搜索功能**
  - 支持中文拼音全拼搜索（如：`changan` 匹配 `长安`）
  - 支持中文拼音首字母搜索（如：`ca` 匹配 `长安`）
  - 智能处理多音字（如：`长安` 同时生成 `changan` 和 `zhangan`）
  - 模糊搜索自动考虑拼音匹配得分
- ✨ **配置持久化存储**
  - 使用 `tauri-plugin-store` 替代自定义配置文件
  - 支持自动保存（100ms 防抖）
  - 支持从旧配置文件自动迁移
  - 配置实时同步到所有窗口

### Changed (变更)

- ♻️ **笔记路径管理重构**
  - 移除 `list_notes` Rust 命令，改为从配置读取
  - `read_note` 和 `save_note` 支持完整路径而非仅文件名
  - 新增 `create_note` 命令，在指定目录创建笔记并返回完整路径
  - 新增 `import_note` 命令，使用原生文件选择对话框
  - 新增 `delete_note_file` 命令，删除指定路径的文件
- ♻️ **系统托盘优化**
  - 托盘菜单新增"设置"选项
  - 从托盘打开设置时不再自动显示主窗口
- ♻️ **搜索功能优化**
  - 搜索目录可在设置中配置，支持多个目录
  - 未配置搜索目录时显示友好提示
  - 搜索结果支持键盘导航时滚动条自动跟随
  - 移除搜索窗口外部边框和阴影

### Performance (性能优化)

- ⚡ 预创建搜索窗口（默认隐藏），提升首次打开速度
- ⚡ 简化全局快捷键处理逻辑
- ⚡ 移除拼音转换调试日志

### Technical (技术)

- 新增依赖：`tauri-plugin-store`、`tauri-plugin-dialog`、`tauri-plugin-autostart`
- 新增依赖：`pinyin` (Rust)，用于中文拼音转换
- 新增前端工具模块：`configStore.ts`、`dialogHelper.ts`
- 新增组件：`SettingsView.vue`、`SettingsDialog.vue`
- 新增路由：`/settings`
- 新增 Rust 命令：`import_note`、`create_note`、`delete_note_file`
- 重构笔记管理逻辑，所有笔记路径存储在配置的 `notes` 字段

## [0.2.1] - 2025-10-17

### Fixed (修复)

- 🐛 修复笔记页 Tabs 加号右侧拖拽空白会导致窗口隐藏的问题（取消主窗口失焦自动隐藏，保留搜索窗口失焦隐藏；为拖拽区单独设置 `-webkit-app-region: drag`）
- 🐛 修复工作区搜索窗口键盘导航偶发“隔行”和回车重复打开的问题（移除重复键盘监听、引入打开防抖）

### Changed (变更)

- ♻️ Alt+Space 切换逻辑优化：可见未聚焦时直接置前聚焦，避免需要按两次

## [0.2.0] - 2025-10-17

### Added (新增)

- ✨ 配置系统
  - 支持持久化配置存储（~/.ytools/config.json）
  - 自动加载和保存配置
- ✨ 字体大小调整功能
  - 支持 Ctrl + 滚轮调整字体大小（12-32px）
  - 配置自动保存和恢复
- ✨ 窗口失焦自动隐藏
  - 主窗口和搜索窗口失去焦点时自动隐藏
  - 提升用户体验

### Technical (技术)

- 新增 `read_config`、`save_config`、`update_font_size` Rust 命令
- 添加 serde 序列化支持

## [0.1.0] - 2025-10-16

### Added (新增)

- ✨ 纯文本笔记编辑器
  - 多笔记标签页管理
  - 自动保存（2秒防抖）
  - 手动保存快捷键 (Ctrl+S)
  - 会话恢复功能
  - 本地存储在 ~/.ytools/ 目录
- 🔍 工作区快速搜索
  - 智能模糊搜索算法
  - 支持去分隔符匹配
  - 键盘导航支持
  - 全局快捷键唤起 (Ctrl+Space)
- ⌨️ 全局快捷键系统
  - Alt+Space: 显示/隐藏主窗口
  - Ctrl+Space: 显示/隐藏搜索窗口
  - Esc: 隐藏当前窗口
- 🎯 系统托盘集成
  - 后台常驻
  - 托盘菜单（显示/隐藏、退出）
  - 左键点击切换窗口显示
- 🎨 现代化界面设计
  - 暗色主题
  - 无边框窗口
  - 流畅过渡动画
  - 响应式布局
- 📝 自定义对话框组件
  - ESC 键关闭
  - 点击遮罩层关闭
  - Enter 键确认
  - 自动聚焦

### Changed (变更)

- ♻️ 将 Vditor Markdown 编辑器替换为轻量级纯文本编辑器
  - 移除 vditor 依赖
  - 创建 TextEditor.vue 组件
  - 优化编辑器性能和加载速度

### Technical (技术)

- 基于 Tauri 2.0 + Vue 3 + TypeScript 构建
- Rust 后端实现核心功能
- 使用 pnpm 作为包管理器
- Vite 作为构建工具

---

## 版本说明

### 版本号格式：主版本号.次版本号.修订号

- **主版本号 (MAJOR)**：做了不兼容的 API 修改
- **次版本号 (MINOR)**：做了向下兼容的功能性新增
- **修订号 (PATCH)**：做了向下兼容的问题修正

### 变更类型

- **Added (新增)**：新增的功能
- **Changed (变更)**：对现有功能的变更
- **Deprecated (弃用)**：即将移除的功能
- **Removed (移除)**：已移除的功能
- **Fixed (修复)**：任何 Bug 的修复
- **Security (安全)**：安全相关的改进

---

[Unreleased]: https://github.com/yourusername/ytools/compare/v0.5.1...HEAD
[0.5.1]: https://github.com/yourusername/ytools/releases/tag/v0.5.1
[0.5.0]: https://github.com/yourusername/ytools/releases/tag/v0.5.0
[0.4.2]: https://github.com/yourusername/ytools/releases/tag/v0.4.2
[0.4.1]: https://github.com/yourusername/ytools/releases/tag/v0.4.1
[0.4.0]: https://github.com/yourusername/ytools/releases/tag/v0.4.0
[0.3.0]: https://github.com/yourusername/ytools/releases/tag/v0.3.0
[0.2.1]: https://github.com/yourusername/ytools/releases/tag/v0.2.1
[0.1.0]: https://github.com/yourusername/ytools/releases/tag/v0.1.0
