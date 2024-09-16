import { createRouter, createWebHistory } from 'vue-router'

const router = createRouter({
  history: createWebHistory(import.meta.env.BASE_URL),
  routes: [
    {
      path: '/',
      redirect: 'playlists'
    },

    {
      path: '/playlists',
      name: 'playlists',
      component: () => import('../views/PlaylistsView.vue'),
      redirect: '/playlists/default',
      children: [
        {
          path: '/playlists/:playlist_name',
          name: 'edit_playlist',
          component: () => import('../views/PlaylistEditView.vue')
        }
      ]
    },
    {
      path: '/login',
      name: 'login',
      component: () => import('../views/LoginView.vue')
    }
  ]
})

export default router
