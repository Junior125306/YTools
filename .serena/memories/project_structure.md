# YTools 项目结构

## 根目录结构
```
YTools/
├── src/                          # 前端源码
│   ├── components/               # Vue 组件
│   │   ├── TextEditor.vue        # 文本编辑器组件
│   │   ├── InputDialog.vue       # 输入对话框组件
│   │   ├── KeybindingInput.vue   # 快捷键输入组件
│   │   └── SettingsDialog.vue    # 设置对话框组件
│   ├── views/                    # 页面视图
│   │   ├── HomeView.vue          # 主页（笔记）
│   │   ├── SearchView.vue        # 搜索页
│   │   └── SettingsView.vue      # 设置页
│   ├── composables/             # 组合式函数
│   │   └── useTheme.ts           # 主题管理
│   ├── utils/                    # 工具函数
│   │   ├── configStore.ts        # 配置存储管理
│   │   └── dialogHelper.ts       # 对话框辅助函数
│   ├── router/                   # 路由配置
│   │   └── index.ts              # 路由定义
│   ├── assets/                   # 静态资源
│   │   └── vue.svg
│   ├── App.vue                   # 根组件 + 全局样式
│   ├── main.ts                   # 应用入口
│   └── vite-env.d.ts             # Vite 类型定义
├── src-tauri/                    # Tauri/Rust 后端
│   ├── src/
│   │   ├── lib.rs                # 核心逻辑（命令、托盘、快捷键）
│   │   └── main.rs               # 入口文件
│   ├── capabilities/             # 权限配置
│   │   └── default.json
│   ├── icons/                    # 应用图标
│   ├── Cargo.toml                # Rust 依赖
│   ├── Cargo.lock                # Rust 锁定文件
│   └── tauri.conf.json           # Tauri 配置
├── public/                       # 静态资源
│   ├── tauri.svg
│   └── vite.svg
├── dist/                         # 构建输出（前端）
├── docs/                         # 文档目录
├── package.json                  # Node 依赖
├── pnpm-lock.yaml                # pnpm 锁定文件
├── tsconfig.json                 # TypeScript 配置
├── tsconfig.node.json            # Node TypeScript 配置
├── vite.config.ts                # Vite 配置
├── README.md                     # 项目文档
└── CHANGELOG.md                  # 变更日志
```

## 核心模块说明

### 前端模块

#### components/
- **TextEditor.vue**：文本编辑器，支持自动保存、字体调整
- **InputDialog.vue**：输入对话框，用于创建新笔记
- **KeybindingInput.vue**：快捷键输入组件，支持冲突检测
- **SettingsDialog.vue**：设置对话框

#### views/
- **HomeView.vue**：主页，笔记管理界面
- **SearchView.vue**：搜索界面，工作区快速搜索
- **SettingsView.vue**：设置界面，配置管理

#### composables/
- **useTheme.ts**：主题管理组合式函数
  - 支持亮色/暗色/赛博朋克/跟随系统
  - 主题持久化
  - 跨窗口主题同步

#### utils/
- **configStore.ts**：配置存储管理
  - 使用 tauri-plugin-store 持久化
  - 配置项：字体、主题、快捷键、搜索目录等
  - 配置迁移逻辑
- **dialogHelper.ts**：对话框辅助函数

### 后端模块

#### src/lib.rs
- **命令函数**：Tauri 命令定义
  - `greet`：测试命令
  - `read_note` / `save_note`：笔记读写
  - `list_notes` / `list_all_notes`：笔记列表
  - `delete_note`：删除笔记
  - `import_note`：导入笔记
  - `search_workspaces`：工作区搜索
  - `open_folder`：打开文件夹
  - `open_directory`：打开目录
- **系统集成**：
  - 系统托盘设置
  - 全局快捷键注册
  - 窗口管理
- **搜索算法**：
  - 拼音转换（全拼/首字母）
  - 模糊匹配（LCS + 编辑距离）
  - 去分隔符匹配

#### src/main.rs
- Tauri 应用入口
- 窗口初始化
- 插件注册

## 配置文件

### 前端配置
- **vite.config.ts**：Vite 构建配置，端口 1420
- **tsconfig.json**：TypeScript 严格模式配置
- **package.json**：依赖管理和脚本定义

### 后端配置
- **Cargo.toml**：Rust 依赖定义
- **tauri.conf.json**：Tauri 应用配置
  - 窗口配置（无边框、透明）
  - 构建配置
  - 图标配置

### 运行时配置
- **config.json**：用户配置（存储在应用数据目录）
- **笔记文件**：存储在 `~/.ytools/` 或用户指定位置

## 数据流

### 配置数据流
1. 用户修改设置 → `configStore.ts`
2. 保存到 `tauri-plugin-store` → `config.json`
3. 同步到所有窗口（事件广播）

### 笔记数据流
1. 前端调用 Tauri 命令 → `lib.rs`
2. Rust 读写文件系统 → `~/.ytools/`
3. 返回结果到前端 → Vue 组件更新

### 搜索数据流
1. 用户输入查询 → `SearchView.vue`
2. 调用 `search_workspaces` 命令 → `lib.rs`
3. Rust 执行搜索算法 → 返回结果
4. 前端显示结果列表