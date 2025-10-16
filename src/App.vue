<script setup lang="ts">
// App.vue 现在只是路由容器
</script>

<template>
  <RouterView />
</template>

<style scoped>
/* 全局样式已移至 style 标签中 */
</style>

<style>
* {
  margin: 0;
  padding: 0;
  box-sizing: border-box;
}

body {
  font-family: -apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, Oxygen,
    Ubuntu, Cantarell, 'Fira Sans', 'Droid Sans', 'Helvetica Neue', sans-serif;
  -webkit-font-smoothing: antialiased;
  -moz-osx-font-smoothing: grayscale;
  background-color: var(--color-bg);
  color: var(--color-text);
}

#app {
  height: 100vh;
  overflow: hidden;
}

/* 设计系统：主题变量 */
:root {
  /* 色彩 */
  --color-bg: #f7f8fa;
  --color-surface: #ffffff;
  --color-surface-2: #f2f3f5;
  --color-border: #e6e7eb;
  --color-text: #1f2937; /* gray-800 */
  --color-text-muted: #6b7280; /* gray-500 */
  --color-accent: #6366f1; /* indigo-500 */
  --color-accent-hover: #4f46e5; /* indigo-600 */
  --color-accent-press: #4338ca; /* indigo-700 */

  /* 圆角与阴影 */
  --radius-s: 8px;
  --radius-m: 12px;
  --radius-l: 16px;
  --shadow-sm: 0 1px 2px rgba(0, 0, 0, 0.06);
  --shadow-md: 0 8px 24px rgba(0, 0, 0, 0.08);

  /* 动效 */
  --tr-fast: 160ms ease;
}

@media (prefers-color-scheme: dark) {
  :root {
    --color-bg: #0f1115;
    --color-surface: #12151c;
    --color-surface-2: #161a22;
    --color-border: #232938;
    --color-text: #e5e7eb;
    --color-text-muted: #9ca3af;
    --color-accent: #818cf8; /* indigo-400 */
    --color-accent-hover: #6366f1; /* indigo-500 */
    --color-accent-press: #4f46e5; /* indigo-600 */

    --shadow-sm: 0 1px 2px rgba(0, 0, 0, 0.25);
    --shadow-md: 0 12px 32px rgba(0, 0, 0, 0.35);
  }
}

/* 基础控件：统一扁平、现代、圆角风格 */
button,
input,
textarea {
  font: inherit;
  color: inherit;
}

button {
  border: 1px solid var(--color-border);
  background: var(--color-surface);
  color: var(--color-text);
  border-radius: var(--radius-s);
  padding: 6px 12px;
  transition: background var(--tr-fast), color var(--tr-fast), border-color var(--tr-fast), transform var(--tr-fast), box-shadow var(--tr-fast);
}

button:hover:not(:disabled) {
  border-color: transparent;
  background: var(--color-surface-2);
}

button:active:not(:disabled) {
  transform: translateY(0.5px);
}

button:disabled {
  opacity: 0.6;
  cursor: not-allowed;
}

input[type="text"],
textarea {
  background: var(--color-surface);
  color: var(--color-text);
  border: 1px solid var(--color-border);
  border-radius: var(--radius-m);
  outline: none;
}

input[type="text"]:focus,
textarea:focus {
  border-color: var(--color-accent);
  box-shadow: 0 0 0 3px color-mix(in srgb, var(--color-accent) 20%, transparent);
}
</style>
