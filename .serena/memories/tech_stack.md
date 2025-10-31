# YTools 技术栈

## 前端技术栈
- **框架**：Vue 3 (Composition API)
- **开发语言**：TypeScript 5.6.2
- **构建工具**：Vite 6.0.3
- **路由**：Vue Router 4.6.3
- **UI 组件库**：Naive UI 2.43.1
- **图标库**：@vicons/ionicons5 0.13.0

## 后端技术栈
- **框架**：Tauri 2.0
- **语言**：Rust (Edition 2021)
- **核心依赖**：
  - tauri-plugin-opener: 2
  - tauri-plugin-global-shortcut: 2
  - tauri-plugin-fs: 2
  - tauri-plugin-store: 2.0.0
  - tauri-plugin-dialog: 2.0.0
  - tauri-plugin-autostart: 2.0.0
- **工具库**：
  - serde / serde_json: 序列化
  - dirs: 5 (目录路径获取)
  - pinyin: 0.10 (拼音转换)

## 开发工具
- **包管理器**：pnpm >= 8.0
- **Node.js**：>= 18.0
- **Rust**：>= 1.70
- **TypeScript 编译器**：vue-tsc 2.1.10

## 存储方案
- **配置存储**：tauri-plugin-store (config.json)
- **笔记存储**：本地文件系统 (~/.ytools/)
- **数据格式**：JSON

## 开发服务器
- **端口**：1420 (固定端口)
- **HMR 端口**：1421
- **协议**：HTTP/WebSocket