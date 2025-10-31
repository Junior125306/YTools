# YTools 代码风格和约定

## 命名规范

### TypeScript/Vue
- **文件命名**：PascalCase (组件), camelCase (工具/composables)
  - 组件：`TextEditor.vue`, `SettingsDialog.vue`
  - 工具：`configStore.ts`, `dialogHelper.ts`
  - Composables：`useTheme.ts`
- **变量/函数**：camelCase
- **类型/接口**：PascalCase，接口名可选 `I` 前缀或直接命名
- **常量**：UPPER_SNAKE_CASE 或 camelCase

### Rust
- **函数命名**：snake_case
- **结构体/枚举**：PascalCase
- **常量**：UPPER_SNAKE_CASE
- **模块**：snake_case

## 代码规范

### TypeScript
- **严格模式**：启用 TypeScript strict 模式
- **类型注解**：尽可能显式类型注解，避免 any
- **Composition API**：优先使用 `<script setup lang="ts">`
- **导入顺序**：外部库 → 内部模块 → 类型导入
- **注释**：使用中文注释，关键逻辑必须注释

### Vue 组件结构
```vue
<script setup lang="ts">
// 1. 导入
// 2. 类型定义
// 3. Props/Emits
// 4. Composables/状态
// 5. 计算属性
// 6. 方法
// 7. 生命周期
</script>

<template>
  <!-- 模板内容 -->
</template>

<style scoped>
/* 组件样式 */
</style>
```

### Rust
- **错误处理**：使用 `Result<T, String>` 或自定义错误类型
- **注释**：使用中文注释
- **模块组织**：按功能划分模块
- **文档注释**：使用 `///` 为公共 API 添加文档

## 配置约定

### 配置键命名
- camelCase：`fontSize`, `fontFamily`, `lineHeight`
- 布尔值：`autoStart`
- 数组：`searchDirectories`, `notes`
- 对象：`shortcuts`

### 默认值
- 字体大小：16px
- 字体族：`Consolas, 'Courier New', monospace`
- 行高：1.6
- 主题：`system` (跟随系统)
- 快捷键：
  - 主窗口：`Alt+Space`
  - 搜索窗口：`Ctrl+Space`

## 文件组织

### 目录结构
```
src/
├── components/       # 可复用组件
├── views/          # 页面视图
├── composables/    # 组合式函数
├── utils/          # 工具函数
├── router/         # 路由配置
└── assets/         # 静态资源
```

### 导入路径
- 相对路径：`./`, `../`
- 别名导入：使用 `@/` (如果配置了)
- 类型导入：`import type { ... }`

## 注释规范
- **单行注释**：`// 中文注释`
- **多行注释**：`/* 中文注释 */`
- **文档注释**：函数/类上方使用中文说明
- **TODO 注释**：`// TODO: 描述`

## 错误处理
- **前端**：使用 try-catch，console.error 记录错误
- **后端**：返回 `Result<T, String>`，错误信息使用中文
- **用户提示**：错误信息友好、可操作