# Changelog

本文档记录 YTools 的所有重要变更。

格式基于 [Keep a Changelog](https://keepachangelog.com/zh-CN/1.0.0/)，
版本号遵循 [语义化版本](https://semver.org/lang/zh-CN/)。

## [Unreleased]

### 计划中

- 设置中心（主题、字体、路径配置）
- 快捷图标访问面板
- 快捷指令系统
- JSON 格式化工具
- 加解密工具（AES、Base64）
- 远程桌面快捷方式
- 工单管理系统

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

[Unreleased]: https://github.com/yourusername/ytools/compare/v0.2.1...HEAD
[0.2.1]: https://github.com/yourusername/ytools/releases/tag/v0.2.1
[0.1.0]: https://github.com/yourusername/ytools/releases/tag/v0.1.0
