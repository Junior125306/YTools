# YTools - 高效工作辅助工具集

<div align="center">

一款基于 **Tauri 2 + Vue 3 + TypeScript** 构建的轻量级桌面工具集  
专为提升日常工作效率而设计，聚焦「简洁 · 高效 · 优雅」

本项目完全由Cursor生成，并由Cursor进行持续优化和维护。

[![Tauri](https://img.shields.io/badge/Tauri-2.0-blue.svg)](https://tauri.app/)
[![Vue.js](https://img.shields.io/badge/Vue-3.x-green.svg)](https://vuejs.org/)
[![TypeScript](https://img.shields.io/badge/TypeScript-5.x-blue.svg)](https://www.typescriptlang.org/)
[![Rust](https://img.shields.io/badge/Rust-1.70+-orange.svg)](https://www.rust-lang.org/)

</div>

---

## 📋 目录

- [功能特性](#-功能特性)
  - [已实现功能](#已实现功能)
  - [开发路线图](#-开发路线图)
- [快捷键](#-快捷键)
- [技术栈](#-技术栈)
- [快速开始](#-快速开始)
- [配置说明](#-配置说明)
- [项目结构](#-项目结构)
- [许可证](#-许可证)

---

## ✨ 功能特性

### 已实现功能

#### 📝 笔记管理

- **纯文本编辑器**：轻量、快速的文本笔记
- **多笔记支持**：标签页式管理，快速切换
- **统一笔记管理**：支持导入外部 `.md` 和 `.txt` 文件，混合显示内置和外部笔记 ✨新
- **导入功能**：点击 📁 按钮快速导入现有笔记文件 ✨新
- **智能删除**：删除笔记时可选择永久删除文件或仅从列表移除 ✨新
- **同名支持**：使用完整路径存储，支持同名文件共存 ✨新
- **自动保存**：输入停止 2 秒后自动保存，防止数据丢失
- **手动保存**：支持 `Ctrl+S` / `Cmd+S` 快捷键
- **会话恢复**：自动记忆上次编辑的笔记
- **本地存储**：默认保存在 `~/.ytools/` 目录（可在设置中修改）
- **字体调整**：`Ctrl + 滚轮` 快速调节字体大小（12-32px），配置自动保存

#### 🔍 工作区快速搜索

- **智能搜索**：多种匹配方式，快速定位目标
  - 直接匹配：输入完整或部分目录名称
  - 去分隔符匹配：如 `saleclient` 匹配 `sale-client`
  - **拼音全拼搜索**：如 `xiangmu` 匹配 `项目管理` ✨新
  - **拼音首字母搜索**：如 `xmgl` 匹配 `项目管理` ✨新
  - 模糊匹配：当无直接匹配时，使用智能算法排序（最长公共子串 + LCS + 编辑距离）
  - 自动返回最相关的前 5 个结果
- **键盘导航**：
  - `↑` / `↓` 选择项目，滚动条自动跟随 ✨新
  - `Enter` 打开文件夹
  - `Esc` 关闭搜索窗口
- **快速启动**：`Ctrl+Space` 全局唤起
- **可配置目录**：在设置中管理搜索目录列表 ✨新

#### ⌨️ 全局快捷键

- **Alt+Space**：显示/隐藏主窗口（笔记）
- **Ctrl+Space**：显示/隐藏搜索窗口
- **Esc**：隐藏当前窗口

#### 🎯 系统集成

- **系统托盘**：后台常驻，快速访问
  - 左键点击：显示/隐藏主窗口
  - 右键菜单：显示/隐藏、退出
- **无边框窗口**：现代化界面设计
- **始终置顶**：搜索窗口自动置顶（可选）
- **失焦自动隐藏**：搜索窗口失焦自动隐藏；主窗口默认不再失焦隐藏，避免拖拽/切换时误隐藏

#### ⚙️ 配置管理

- **设置中心**：独立的设置窗口，支持以下配置 ✨新
  - **通用设置**：开机启动
  - **编辑器设置**：字体大小、字体族、行高
  - **搜索设置**：可配置的搜索目录列表
  - **笔记设置**：默认笔记保存位置
- **持久化存储**：使用 `tauri-plugin-store` 自动保存配置
- **实时生效**：配置更改后自动应用到所有窗口

#### 🎨 界面设计

- **暗色主题**：护眼的深色配色
- **现代化 UI**：扁平化设计，圆角风格
- **流畅动画**：优雅的过渡效果
- **响应式布局**：适配不同屏幕尺寸

---

### 🗺 开发路线图

#### 🔧 设置中心

- [ ] **主题设置**
  - 浅色/深色主题切换
  - 自定义配色方案
  - 跟随系统主题
- [x] ~~**字体配置**~~ (已完成)
  - [x] ~~字体大小调节~~
  - [x] ~~字体系列选择~~
  - [x] ~~行高设置~~
- [x] ~~**路径管理**~~ (已完成)
  - [x] ~~自定义工作区目录（搜索目录）~~
  - [x] ~~多工作区支持~~
  - [x] ~~笔记存储位置配置~~
- [x] ~~**开机启动**~~ (已完成)

#### 🚀 快捷访问（计划中）

- [ ] **快捷图标面板**
  - 常用应用快速启动
  - 自定义图标和命令
  - 拖拽排序
- [ ] **快捷指令**
  - 预设常用操作
  - 自定义脚本执行
  - 参数化命令模板

#### 🛠 开发者工具（计划中）

- [ ] **JSON 工具**
  - JSON 格式化
  - JSON 验证
  - JSON 转换（压缩/美化）
- [ ] **加解密工具**
  - AES 加密/解密
  - Base64 编码/解码
  - MD5/SHA 哈希计算
  - URL 编码/解码

#### 🖥 远程管理（计划中）

- [ ] **远程桌面快捷方式**
  - 保存常用远程桌面连接
  - 一键启动 RDP/VNC
  - 连接历史记录
- [ ] **工单管理**
  - 快速登记工单
  - 查看工单状态
  - 工单提醒功能

#### 📌 笔记增强

- [x] ~~删除笔记功能~~ (已完成)
- [x] ~~导入外部笔记~~ (已完成)
- [ ] 重命名笔记
- [ ] 笔记内搜索（Ctrl+F）
- [ ] 全局笔记内容搜索
- [ ] 笔记导出（.txt / .md）
- [ ] 笔记标签分类
- [ ] 回收站功能

---

## ⌨️ 快捷键

### 全局快捷键

| 快捷键 | 功能 | 说明 |
|--------|------|------|
| `Alt + Space` | 显示/隐藏主窗口 | 快速唤起笔记窗口 |
| `Ctrl + Space` | 显示/隐藏搜索窗口 | 快速搜索工作区 |

### 笔记窗口

| 快捷键 | 功能 |
|--------|------|
| `Ctrl + S` | 保存笔记 |
| `Ctrl + 滚轮` | 调整字体大小 |
| `Esc` | 隐藏窗口 |

### 搜索窗口

| 快捷键 | 功能 |
|--------|------|
| `↑` / `↓` | 上下导航 |
| `Enter` | 打开选中文件夹 |
| `Esc` | 关闭搜索窗口 |

---

## 🧩 技术栈

- **前端框架**：Vue 3 (Composition API)
- **开发语言**：TypeScript
- **构建工具**：Vite
- **桌面框架**：Tauri 2
- **后端语言**：Rust
- **状态管理**：Vue Reactivity
- **路由管理**：Vue Router
- **持久化存储**：tauri-plugin-store
- **系统对话框**：tauri-plugin-dialog
- **开机启动**：tauri-plugin-autostart
- **拼音转换**：pinyin (Rust)

---

## 🚀 快速开始

### 环境要求

- **Node.js**：>= 18.0
- **pnpm**：>= 8.0
- **Rust**：>= 1.70
- **系统要求**：
  - Windows: Visual Studio 2022 C++ 构建工具
  - macOS: Xcode Command Line Tools
  - Linux: 必要的开发库（参考 Tauri 文档）

### 安装依赖

```bash
# 安装前端依赖
pnpm install
```

### 开发调试

```bash
# 启动开发服务器（带热重载）
pnpm tauri dev
```

### 生产构建

```bash
# 构建生产版本
pnpm tauri build
```

构建产物位于 `src-tauri/target/release/bundle/`

---

## ⚙️ 配置说明

### 工作区目录配置

默认工作区目录：`D:\workspaces`

如需修改，请编辑 `src-tauri/src/lib.rs`：

```rust
// 搜索工作区文件夹
#[tauri::command]
fn search_workspaces(query: String) -> Result<Vec<String>, String> {
    let workspace_dir = PathBuf::from("D:\\workspaces"); // 修改此路径
    // ...
}

// 打开文件夹
#[tauri::command]
fn open_folder(folder_name: String) -> Result<(), String> {
    let folder_path = PathBuf::from("D:\\workspaces").join(&folder_name); // 修改此路径
    // ...
}
```

### 笔记存储位置

笔记文件存储在用户主目录的 `.ytools` 文件夹：

- **Windows**: `C:\Users\<用户名>\.ytools\`
- **macOS**: `/Users/<用户名>/.ytools/`
- **Linux**: `/home/<用户名>/.ytools/`

### 全局快捷键自定义

在 `src-tauri/src/lib.rs` 中修改：

```rust
// 设置全局快捷键
let alt_space = Shortcut::new(Some(Modifiers::ALT), Code::Space);
let ctrl_space = Shortcut::new(Some(Modifiers::CONTROL), Code::Space);
```

---

## 📁 项目结构

```
YTools/
├── src/                          # 前端源码
│   ├── components/               # Vue 组件
│   │   ├── TextEditor.vue        # 文本编辑器组件
│   │   └── InputDialog.vue       # 输入对话框组件
│   ├── views/                    # 页面视图
│   │   ├── HomeView.vue          # 主页（笔记）
│   │   └── SearchView.vue        # 搜索页
│   ├── router/                   # 路由配置
│   │   └── index.ts
│   ├── App.vue                   # 根组件 + 全局样式
│   └── main.ts                   # 应用入口
├── src-tauri/                    # Tauri/Rust 后端
│   ├── src/
│   │   ├── lib.rs                # 核心逻辑（命令、托盘、快捷键）
│   │   └── main.rs               # 入口文件
│   ├── capabilities/             # 权限配置
│   │   └── default.json
│   ├── icons/                    # 应用图标
│   ├── Cargo.toml                # Rust 依赖
│   └── tauri.conf.json           # Tauri 配置
├── public/                       # 静态资源
├── package.json                  # Node 依赖
├── tsconfig.json                 # TypeScript 配置
├── vite.config.ts                # Vite 配置
└── README.md                     # 项目文档
```

---

## 🔍 核心功能实现

### 模糊搜索算法

搜索策略采用多级匹配机制（Rust 实现）：

1. **标准匹配**：
   - 小写化后包含匹配
   - 去分隔符归一化匹配（`sale-client` ≈ `saleclient`）

2. **智能回退**（当标准匹配无结果时）：
   - 计算最长公共子串（连续匹配长度）
   - 计算最长公共子序列（总匹配字符数）
   - 计算 Levenshtein 编辑距离
   - 综合排序，返回前 5 个最相关结果

### 自动保存机制

- 使用防抖（debounce）技术
- 内容变化后 2 秒触发保存
- 避免频繁 I/O 操作
- 切换笔记时自动保存当前内容

---

## 🎨 设计理念

### UI/UX 原则

- **简洁至上**：去除冗余元素，聚焦核心功能
- **一致性**：统一的设计语言和交互模式
- **高效性**：快捷键优先，减少鼠标操作
- **现代感**：扁平化设计，流畅动画

### 主题系统

全局 CSS 变量管理（`src/App.vue`）：

```css
:root {
  --color-bg: #0f1115;           /* 背景色 */
  --color-surface: #12151c;      /* 表面色 */
  --color-border: #232938;       /* 边框色 */
  --color-text: #e5e7eb;         /* 文本色 */
  --color-accent: #818cf8;       /* 强调色 */
  --radius-s: 8px;               /* 小圆角 */
  --radius-m: 12px;              /* 中圆角 */
  --shadow-md: 0 8px 24px ...;   /* 阴影 */
}
```

---

## 🤝 贡献指南

欢迎提交 Issue 和 Pull Request！

### 开发规范

- 提交前运行 `pnpm build` 确保构建通过
- 遵循项目已有的代码风格
- 提交信息使用中文，简洁明了

---

## 📄 许可证

本项目基于 [MIT](LICENSE) 许可证开源

---

## 🙏 致谢

- [Tauri](https://tauri.app/) - 轻量级桌面应用框架
- [Vue.js](https://vuejs.org/) - 渐进式 JavaScript 框架
- [Vite](https://vitejs.dev/) - 下一代前端构建工具
- [Rust](https://www.rust-lang.org/) - 高性能系统编程语言

---

<div align="center">

**用 ❤️ 打造的效率工具**

如果觉得有用，请给个 ⭐️ Star 支持一下！

</div>
