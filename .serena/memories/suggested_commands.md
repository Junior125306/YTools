# YTools 常用命令

## 项目初始化

### 安装依赖
```powershell
# 安装前端依赖
pnpm install
```

### 开发调试
```powershell
# 启动开发服务器（带热重载）
pnpm tauri dev
```

### 生产构建
```powershell
# 构建生产版本
pnpm tauri build
```

## 开发命令

### 前端开发
```powershell
# 启动 Vite 开发服务器（仅前端）
pnpm dev

# 类型检查（无输出）
pnpm vue-tsc --noEmit

# 构建前端
pnpm build

# 预览构建结果
pnpm preview
```

### Rust 开发
```powershell
# 进入 Rust 目录
cd src-tauri

# 检查 Rust 代码
cargo check

# 运行 Rust 测试
cargo test

# 清理构建
cargo clean
```

## 系统命令（Windows PowerShell）

### 文件操作
```powershell
# 列出目录
Get-ChildItem 或 ls

# 切换目录
cd <路径>

# 查找文件
Get-ChildItem -Recurse -Filter "*.ts"

# 查找文本
Select-String -Path "*.ts" -Pattern "pattern"
```

### Git 操作
```powershell
# 查看状态
git status

# 查看差异
git diff

# 提交更改
git commit -m "提交信息"

# 推送更改
git push
```

## 构建产物位置

### 开发构建
- `src-tauri/target/debug/` - 调试版本
- `src-tauri/target/debug/bundle/` - 安装包

### 生产构建
- `src-tauri/target/release/` - 发布版本
- `src-tauri/target/release/bundle/` - 安装包
  - `msi/` - Windows 安装程序
  - `nsis/` - NSIS 安装程序

## 配置相关

### 配置文件位置
- **Tauri 配置**：`src-tauri/tauri.conf.json`
- **前端配置**：`vite.config.ts`
- **TypeScript 配置**：`tsconfig.json`
- **用户配置**：`~/.ytools/config.json` (运行时生成)

### 笔记存储位置
- **默认位置**：`~/.ytools/` (用户主目录下的隐藏文件夹)
- **Windows**：`C:\Users\<用户名>\.ytools\`
- **可配置**：在设置页面修改默认笔记位置

## 调试技巧

### 前端调试
- 使用浏览器开发者工具（F12）
- Vue DevTools 扩展
- 控制台日志：`console.log`, `console.error`

### Rust 调试
- 使用 `dbg!()` 宏
- `println!()` 输出调试信息
- 使用 VS Code Rust Analyzer 插件

## 测试命令
- **前端**：暂未配置测试框架
- **后端**：`cargo test` (Rust 单元测试)

## 代码检查
- **TypeScript**：`pnpm vue-tsc --noEmit`
- **Rust**：`cargo clippy` (代码质量检查)