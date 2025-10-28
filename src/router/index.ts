import { createRouter, createWebHashHistory } from 'vue-router'
import HomeView from '../views/HomeView.vue'
import SearchView from '../views/SearchView.vue'
import SettingsView from '../views/SettingsView.vue'

const router = createRouter({
  history: createWebHashHistory(import.meta.env.BASE_URL),
  routes: [
    {
      path: '/',
      name: 'home',
      component: HomeView
    },
    {
      path: '/search',
      name: 'search',
      component: SearchView
    },
    {
      path: '/settings',
      name: 'settings',
      component: SettingsView
    }
  ]
})

export default router

