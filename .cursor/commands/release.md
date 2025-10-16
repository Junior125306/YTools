# Release 发布流程

## ⚠️ 重要说明

**AI 必须自动执行所有步骤，而不是仅提供建议！**

用户期望 AI 完成：
1. ✅ 自动修改 CHANGELOG.md
2. ✅ 自动修改 README.md  
3. ✅ 自动执行 `git add` 和 `git commit`
4. ⚠️ 仅提示 `git push`（让用户手动执行）

---

## 📋 执行步骤

### 1. 分析代码变更（自动执行）
- 执行 `git status` 和 `git diff --stat` 查看变更
- 识别变更类型：
  - 🎉 **新功能** (feat)
  - 🐛 **Bug 修复** (fix)
  - 📝 **文档更新** (docs)
  - 💄 **样式调整** (style)
  - ♻️ **代码重构** (refactor)
  - ⚡️ **性能优化** (perf)
  - ✅ **测试相关** (test)
  - 🔧 **配置变更** (chore)

### 2. 确定版本号变更（自动检查，变更需确认）
根据 [语义化版本](https://semver.org/lang/zh-CN/) 规则：

- **主版本号 (MAJOR)**: 不兼容的 API 修改
- **次版本号 (MINOR)**: 向下兼容的功能性新增
- **修订号 (PATCH)**: 向下兼容的问题修正

**自动检查以下文件中的版本号**：
- `package.json` (version)
- `src-tauri/Cargo.toml` (version)
- `src-tauri/tauri.conf.json` (version)

**判断标准**：
- 新增主要功能模块 → 建议增加 MINOR 版本（询问用户）
- Bug 修复、小优化 → 建议增加 PATCH 版本（询问用户）
- 破坏性变更 → 必须增加 MAJOR 版本（需用户确认）

如果需要升级版本号，**自动使用 `search_replace` 更新这三个文件**，确保版本号一致。

### 3. 自动更新 CHANGELOG.md

**必须自动执行**: 使用 `search_replace` 工具修改 `CHANGELOG.md`。

在 `## [Unreleased]` 下方添加新版本记录：

```markdown
## [版本号] - YYYY-MM-DD

### Added (新增)
- 列出新增的功能

### Changed (变更)
- 列出功能变更

### Fixed (修复)
- 列出 Bug 修复

### Removed (移除)
- 列出移除的功能
```

**注意**：
- 使用今天的日期 (YYYY-MM-DD)
- 使用清晰、用户友好的语言
- 按重要性排序

### 4. 自动更新 README.md

**必须自动执行**: 使用 `search_replace` 工具修改 `README.md`。

#### 4.1 更新"已实现功能"部分
- 如果新增了功能，自动将其添加到 `### 已实现功能` 章节
- 按功能模块分类
- 使用清晰的描述和 emoji 图标

#### 4.2 更新"开发路线图"部分
- 将已完成的功能从 `### 🗺 开发路线图` 中**自动删除**
- 或标记为 `[x]` 已完成

#### 4.3 更新"项目结构"（如果有新增文件/目录）
- 自动更新 `### 📁 项目结构` 部分
- 添加新增的重要文件说明

#### 4.4 更新技术栈（如果引入新依赖）
- 在 `### 🧩 技术栈` 自动添加新的技术/库

### 5. 生成 Commit Message

根据变更内容生成符合 [约定式提交](https://www.conventionalcommits.org/zh-hans/) 规范的提交信息。

**格式**：
```
<type>[optional scope]: <description>

[optional body]

[optional footer(s)]
```

**Type 类型**：
- `feat`: 新功能
- `fix`: Bug 修复
- `docs`: 文档更新
- `style`: 代码格式
- `refactor`: 重构
- `perf`: 性能优化
- `test`: 测试相关
- `chore`: 构建/工具变动

**Scope 范围**（可选）：
- `editor`: 编辑器
- `search`: 搜索
- `ui`: 界面
- `config`: 配置
- `deps`: 依赖

### 6. 自动执行 Git 命令

**必须自动执行以下命令**：

```bash
# 1. 添加所有变更（必须执行）
git add .

# 2. 提交（必须执行，使用生成的 commit message）
git commit -m "<完整的 commit message>"

# 3. 如果更新了版本号，创建标签（必须执行）
git tag v<版本号>
```

**然后提示用户手动执行**（不要自动执行 push）：
```bash
# 推送到远程（用户手动执行）
git push origin main
git push origin v<版本号>  # 如果创建了标签
```

### 7. 显示完成报告

显示一个简洁的完成报告：

```
✅ Release 流程已完成！

📝 已更新文件:
- CHANGELOG.md
- README.md
- package.json (如果版本号变更)
- src-tauri/Cargo.toml (如果版本号变更)
- src-tauri/tauri.conf.json (如果版本号变更)

💾 Git 提交:
- ✅ 已执行 git add .
- ✅ 已执行 git commit
- ✅ 已创建标签 v<版本号> (如果适用)

🚀 下一步（请手动执行）:
git push origin main
git push origin v<版本号>
```

---

## 🔄 完整工作流程

```
用户输入: /release
    ↓
AI 自动执行 git status, git diff
    ↓
AI 分析变更类型
    ↓
AI 检查版本号（如需变更，询问用户）
    ↓
AI 自动修改 CHANGELOG.md
    ↓
AI 自动修改 README.md
    ↓
AI 生成 commit message
    ↓
AI 自动执行 git add .
    ↓
AI 自动执行 git commit
    ↓
AI 自动创建 git tag（如果适用）
    ↓
AI 显示完成报告并提示用户 push
```

---

## 📝 关键原则

1. **自动化优先**: 除了 `git push`，其他步骤都必须自动执行
2. **文档同步**: 文档更新与代码变更必须同步
3. **版本一致**: 三个配置文件的版本号必须一致
4. **清晰沟通**: 明确告知用户哪些已完成，哪些需要手动操作

---

## 🎯 触发关键词

当用户说以下任何一句时，自动执行此流程：
- "/release"
- "准备发布"
- "提交代码"
- "release"
- "准备提交"
