# YTools - 多功能桌面工具集

基于 Tauri 2 + Vue 3 + TypeScript 构建的跨平台桌面应用，聚焦“高效、优雅、扁平、现代、圆角”的一致化体验。

## ✨ 功能与特性

- 临时笔记（Markdown）
  - MD/源码双模式：
    - MD 模式：左侧编辑、右侧实时预览（默认）。
    - 源码模式：全宽编辑。
  - 自动保存（输入 2s 防抖），支持 Ctrl+S 手动保存。
  - 暗色主题，扁平化、圆角 UI。
- 文件夹查询（搜索工作区）
  - 模糊检索 `%keyword%`；
  - 回退智能匹配：当无直接包含命中时，按“最长连续匹配长度 + 总匹配字符数 + 编辑距离”综合排序并返回前 5 项（适配常见误拼，如 `saleclient` ≈ `sale-client`；`safeclient` ≈ `safecheckclient`）。
  - 方向键导航、Enter 打开、Esc 关闭。
- 无边框窗口（Windows）：支持在工具栏拖拽移动窗口。
- 系统托盘与全局快捷键
  - Alt+Space 切换主窗口显示/隐藏
  - Ctrl+Space 显示/隐藏搜索窗口

## 🧩 技术栈

- 前端：Vue 3、TypeScript、Vite
- 桌面：Tauri 2（Rust）

## 📦 安装与运行

```bash
pnpm install

# 开发调试（Tauri Dev）
pnpm tauri dev

# 生产构建
pnpm tauri build
```

### Windows 先决条件（Tauri）
- 安装 Visual Studio 2022 C++ 构建工具（含 Desktop development with C++）。

## 🗂 工作区目录说明

搜索功能当前默认扫描 `D:\\workspaces`。

- 如需修改，请调整 `src-tauri/src/lib.rs` 中 `search_workspaces` / `open_folder` 所用的根路径。

## 🎛 快捷键

- 笔记页：
  - Ctrl+S 保存；
  - 模式切换：顶部 MD / 源码。
- 搜索页：
  - ↑/↓ 导航，Enter 打开，Esc 关闭。

## 🎨 设计与主题

- 全局主题变量位于 `src/App.vue`：
  - 颜色、圆角、阴影、动效统一管理；
  - 支持系统暗色模式；
  - 笔记页内强制暗色覆盖，确保一致观感。

## 🧠 模糊搜索策略（后端 Rust）

1. 标准匹配：小写化后进行包含匹配，并额外进行“去分隔符”的归一化匹配（`sale-client` ≈ `saleclient`）。
2. 回退排序（当 1 无结果）：对所有目录计算：
   - 最长公共子串长度（连续匹配优先）、
   - 最长公共子序列长度（总匹配量）、
   - Levenshtein 编辑距离；
   按上述指标（降序/升序）综合排序，取前 5。

## 📁 主要目录结构

```
YTools/
├─ src/
│  ├─ views/
│  │  ├─ HomeView.vue        # 笔记：MD/源码双模式、自动保存、暗色
│  │  └─ SearchView.vue      # 搜索：模糊与回退、键盘导航
│  ├─ App.vue                # 全局主题变量与基础样式
│  └─ router/index.ts        # 路由
├─ src-tauri/
│  ├─ src/lib.rs             # Rust 后端：笔记读写、搜索、打开文件夹、托盘与快捷键
│  └─ tauri.conf.json        # Tauri 配置（无边框、透明窗口等）
└─ vite.config.ts
```

## 🔒 许可证

MIT

## 🙌 致谢

使用 ❤️ 与 Tauri + Rust + Vue 打造。
