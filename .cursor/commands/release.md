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

**必须自动执行**:

#### 3.1 插入新版本变更记录

使用 `search_replace` 在 `## [Unreleased]` 下方插入新版本:

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
- **用户视角，而非技术实现**：关注用户能感知到的功能变化
- **简洁明了**：每个功能一句话概括，必要时最多添加 2-3 个关键子点
- **避免技术细节**：不要列举组件名、函数名、代码实现细节
- **合并相关变更**：多个相关的小改动合并为一个功能点
- **不包含 Technical 部分**：技术细节可以在 commit message 中体现
- 按重要性排序

**示例对比**:

❌ **啰嗦版本**（技术实现导向）:

```
### Added
- ✨ **快捷键自定义功能**
  - 支持在设置页面自定义全局快捷键（主窗口/搜索窗口）
  - 新增 KeybindingInput 组件，支持实时录制按键
  - 支持修饰键组合（Ctrl/Alt/Shift/Meta）和多种按键类型
  - 提供快捷键冲突检测和一键重置功能
  - 新增 Rust 命令 update_global_shortcuts 实现动态快捷键注册
  - 设置保存后即时生效，无需重启应用
```

✅ **简洁版本**（用户价值导向）:

```
### Added
- ✨ **快捷键自定义**：可在设置中自定义主窗口和搜索窗口的快捷键，支持冲突检测和一键重置
```

#### 3.2 更新底部版本链接

**关键**: 必须在 git commit **之前**完成，否则会产生二次提交。

使用 `search_replace` 更新文件底部的版本链接引用：

```markdown
[Unreleased]: https://github.com/yourusername/ytools/compare/v0.2.0...HEAD
[0.2.0]: https://github.com/yourusername/ytools/releases/tag/v0.2.0
[0.1.0]: https://github.com/yourusername/ytools/releases/tag/v0.1.0
```

**操作步骤**:

1. 找到旧的 `[Unreleased]:` 行（如 `compare/v0.1.0...HEAD`）
2. 替换为新版本号（如 `compare/v0.2.0...HEAD`）
3. 在其后插入新版本的链接行（如 `[0.2.0]: https://...`）

**示例替换**:

```
旧内容:
[Unreleased]: https://github.com/yourusername/ytools/compare/v0.1.0...HEAD
[0.1.0]: https://github.com/yourusername/ytools/releases/tag/v0.1.0

新内容:
[Unreleased]: https://github.com/yourusername/ytools/compare/v0.2.0...HEAD
[0.2.0]: https://github.com/yourusername/ytools/releases/tag/v0.2.0
[0.1.0]: https://github.com/yourusername/ytools/releases/tag/v0.1.0
```

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

### 5. 测试构建

**在提交前必须测试构建**:

```bash
pnpm build
```

**要求**:
- 构建必须成功（无 TypeScript 错误）
- 如果构建失败，必须先修复错误再继续
- 警告可以忽略，但错误必须修复

---

### 6. 生成 Commit Message

**自动生成符合约定式提交规范的消息**:

**格式**:

```
<type>(<scope>): <简短描述>

<详细说明>

BREAKING CHANGE: <如果有破坏性变更>
```

**规则**:

- **第一行**：一句话概括核心功能变化（用户视角）
- **详细说明**：最多 3-5 个要点，聚焦关键变更
- **避免技术实现细节**：不列举组件名、函数名、内部实现
- **合并相关变更**：多个为了实现同一功能的改动合并为一个要点

**示例对比**:

❌ **啰嗦版本**:

```
feat(settings): 新增快捷键自定义功能

- 支持在设置页面自定义全局快捷键（主窗口/搜索窗口）
- 新增 KeybindingInput 组件，支持实时录制按键
- 支持修饰键组合（Ctrl/Alt/Shift/Meta）和多种按键类型
- 提供快捷键冲突检测和一键重置功能
- 新增 Rust 命令 update_global_shortcuts 实现动态快捷键注册
- 设置保存后即时生效，无需重启应用
- 优化设置窗口布局和主题样式细节
```

✅ **简洁版本**:

```
feat(settings): 新增快捷键自定义功能

- 可在设置中自定义主窗口和搜索窗口快捷键
- 支持冲突检测和一键重置
- 设置保存后即时生效
```

**Scope 参考**:

- `editor`: 编辑器相关
- `search`: 搜索功能
- `ui`: 界面/样式
- `tray`: 系统托盘
- `config`: 配置
- `deps`: 依赖更新

---

### 7. 执行 Git 操作

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

### 8. 完成报告

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
- [ ] 使用 `search_replace` 更新 `CHANGELOG.md` 主体内容（简洁、用户导向，无 Technical 部分）
- [ ] **使用 `search_replace` 更新 `CHANGELOG.md` 底部版本链接（在 commit 之前！）**
- [ ] 使用 `search_replace` 更新 `README.md` 已实现功能（如果是 feat）
- [ ] 使用 `search_replace` 更新 `README.md` 开发路线图（删除已完成项）
- [ ] **执行 `pnpm build` 测试构建（必须成功）**
- [ ] 如果构建失败，修复错误并提交修复后再继续
- [ ] 生成简洁的 commit message（3-5 个要点，聚焦用户价值）
- [ ] 执行 `git add .`
- [ ] 执行 `git commit -m "..."`
- [ ] 执行 `git tag v<版本号>`
- [ ] 显示完成报告（包含版本升级信息）
- [ ] 提示用户手动执行 `git push`
