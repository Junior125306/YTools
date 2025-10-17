# Release 发布流程

## 概览
全自动化代码发布流程：分析变更 → 自动升级版本 → 更新文档 → Git 提交。

**重要**: 除 `git push` 外的所有步骤（包括版本号升级）必须自动执行，无需用户确认。

---

## 项目信息
- **项目名称**: YTools
- **版本文件**:
  - `package.json` → `version`
  - `src-tauri/Cargo.toml` → `version`
  - `src-tauri/tauri.conf.json` → `version`
- **文档文件**: `CHANGELOG.md`, `README.md`
- **当前版本**: 自动从 `package.json` 读取

---

## 执行步骤

### 1. 分析代码变更
**自动执行以下命令**:
```bash
git status
git diff --staged --name-only
git diff --staged --stat
```

**识别变更类型**（根据文件和代码变化判断）:
- `feat`: 新增功能代码
- `fix`: Bug 修复
- `docs`: 仅文档修改
- `style`: 代码格式/样式
- `refactor`: 重构
- `perf`: 性能优化
- `chore`: 配置/依赖变更

---

### 2. 自动确定版本号升级

**版本号升级规则**（自动判断，无需确认）:
- **PATCH** (0.1.0 → 0.1.1): 改动现有功能
  - `fix`: Bug 修复
  - `refactor`: 代码重构
  - `perf`: 性能优化
  - `style`: 代码格式
  - `docs`: 文档更新
  - `chore`: 配置/依赖变更
  
- **MINOR** (0.1.0 → 0.2.0): 新增功能
  - `feat`: 新功能

- **MAJOR** (0.1.0 → 1.0.0): 破坏性变更
  - 由开发人员手动修改，AI 不自动升级主版本号

**自动执行流程**:
1. 从 `package.json` 读取当前版本号（如 `0.1.0`）
2. 根据步骤1识别的变更类型自动计算新版本号：
   - 如果是 `feat` → 升级 MINOR：`0.1.0` → `0.2.0`
   - 如果是 `fix`/`refactor`/`perf`/`docs`/`style`/`chore` → 升级 PATCH：`0.1.0` → `0.1.1`
   - MAJOR 版本由开发人员手动修改，AI 不处理
3. 使用 `search_replace` 同步更新三个文件的版本号：
   ```bash
   # package.json: "version": "0.1.0" → "version": "0.2.0"
   # src-tauri/Cargo.toml: version = "0.1.0" → version = "0.2.0"
   # src-tauri/tauri.conf.json: "version": "0.1.0" → "version": "0.2.0"
   ```
4. 显示版本升级信息（如：`📦 版本升级: 0.1.0 → 0.2.0 (MINOR - feat)`）

**无需用户确认，直接自动执行版本号更新。**

---

### 3. 更新 CHANGELOG.md

**必须自动执行**: 使用 `search_replace` 在 `## [Unreleased]` 下方插入新版本:

```markdown
## [Unreleased]

### 计划中
- ...

## [0.2.0] - 2025-10-17

### Added
- ✨ 新增的功能描述

### Changed
- ♻️ 变更的功能描述

### Fixed
- 🐛 修复的问题描述

## [0.1.0] - 2025-10-16
...
```

**规则**:
- 日期使用今天（格式: YYYY-MM-DD）
- 根据 git diff 内容自动生成变更描述
- 使用用户友好的语言，非技术术语
- 按重要性排序

---

### 4. 更新 README.md

**自动执行以下更新**:

#### 4.1 更新"已实现功能"
如果是 `feat` 类型变更，在 `### 已实现功能` 下添加新功能描述:
```markdown
#### 📝 笔记管理
- **纯文本编辑器**: ...
- **[新功能名称]**: 功能描述  ← 自动添加到对应模块
```

#### 4.2 更新"开发路线图"
将已完成的待办从 `### 🗺 开发路线图` 中删除或标记为完成:
```markdown
- [x] ~~删除笔记功能~~  ← 标记完成或直接删除
```

#### 4.3 更新"技术栈"（如有新依赖）
在 `package.json` 或 `Cargo.toml` 中新增依赖时，自动添加到技术栈列表。

---

### 5. 生成 Commit Message

**自动生成符合约定式提交规范的消息**:

**格式**:
```
<type>(<scope>): <简短描述>

<详细说明>

BREAKING CHANGE: <如果有破坏性变更>
```

**示例**:
```
feat(editor): 新增笔记删除功能

- 添加删除按钮到笔记标签页
- 实现删除确认对话框
- 支持 Delete 键快捷删除

Closes #12
```

**Scope 参考**:
- `editor`: 编辑器相关
- `search`: 搜索功能
- `ui`: 界面/样式
- `tray`: 系统托盘
- `config`: 配置
- `deps`: 依赖更新

---

### 6. 执行 Git 操作

**自动执行以下命令**:

```bash
# 1. 暂存所有变更
git add .

# 2. 提交（使用生成的 commit message）
git commit -m "<完整的 commit message>"

# 3. 创建版本标签（如果版本号有变更）
git tag v0.2.0
```

**然后显示提示**（不自动执行 push）:
```
⚠️ 请手动执行以下命令推送到远程:

git push origin main
git push origin v0.2.0
```

---

### 7. 完成报告

**自动显示**:

```
✅ Release 流程完成！

📦 版本: 0.1.0 → 0.2.0

📝 已更新文件:
  ✓ package.json
  ✓ src-tauri/Cargo.toml
  ✓ src-tauri/tauri.conf.json
  ✓ CHANGELOG.md
  ✓ README.md

💾 Git 操作:
  ✓ git add .
  ✓ git commit -m "feat(editor): 新增笔记删除功能"
  ✓ git tag v0.2.0

🚀 下一步（请手动执行）:
  git push origin main
  git push origin v0.2.0
```

---

## 关键原则

- ✅ **完全自动化**: 除 `git push` 外全部自动执行，包括版本号升级（无需用户确认）
- ✅ **智能版本号**: 根据变更类型自动计算（feat→MINOR, fix等→PATCH）
- ✅ **版本一致性**: 三个配置文件版本号必须同步更新
- ✅ **文档同步**: CHANGELOG 和 README 同步更新
- ✅ **清晰沟通**: 明确告知用户操作结果和下一步

---

## 检查清单

执行此命令时，AI 必须自动完成（无需用户确认）:

- [ ] 运行 `git status` 和 `git diff` 分析变更
- [ ] 识别变更类型（feat/fix/docs...）
- [ ] 从 `package.json` 读取当前版本号
- [ ] 根据变更类型自动计算新版本号（feat→MINOR, fix/refactor等→PATCH）
- [ ] 使用 `search_replace` 更新 `package.json` 版本号
- [ ] 使用 `search_replace` 更新 `src-tauri/Cargo.toml` 版本号
- [ ] 使用 `search_replace` 更新 `src-tauri/tauri.conf.json` 版本号
- [ ] 使用 `search_replace` 更新 `CHANGELOG.md`（插入新版本记录）
- [ ] 使用 `search_replace` 更新 `README.md` 已实现功能（如果是 feat）
- [ ] 使用 `search_replace` 更新 `README.md` 开发路线图（删除已完成项）
- [ ] 生成符合约定式提交规范的 commit message
- [ ] 执行 `git add .`
- [ ] 执行 `git commit -m "..."`
- [ ] 执行 `git tag v<版本号>`
- [ ] 显示完成报告（包含版本升级信息）
- [ ] 提示用户手动执行 `git push`
