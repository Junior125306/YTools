<script setup lang="ts">
import { NConfigProvider, NMessageProvider, useThemeVars } from 'naive-ui'
import { useTheme } from './composables/useTheme'

const { naiveTheme, themeOverrides } = useTheme()
const themeVars = useThemeVars()
</script>

<template>
  <NConfigProvider :theme="naiveTheme" :theme-overrides="themeOverrides">
    <NMessageProvider>
      <div class="app-wrapper">
        <RouterView />
      </div>
    </NMessageProvider>
  </NConfigProvider>
</template>

<style scoped>
.app-wrapper {
  height: 100vh;
  width: 100vw;
  overflow: hidden;
  background-color: v-bind('themeVars.bodyColor');
}
</style>

<style>
/* 最小全局样式 - Naive UI 主题系统接管颜色和主题 */
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
}

#app {
  height: 100vh;
  overflow: hidden;
}
</style>
