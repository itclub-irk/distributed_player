<script setup lang="ts">
import { onMounted } from 'vue'
import { checkIsLoggedIn, doLogout } from './services/authService'
import { useAppStore } from './stores/app'
import { useRoute, useRouter } from 'vue-router'
import AppNotification from './components/AppNotification.vue'

const appStore = useAppStore()
const router = useRouter()

function redirectToLoginPageIfNecessary() {
  if (!appStore.isLoggedIn) {
    router.push({ name: 'login' })
  }
}

onMounted(async () => {
  await checkIsLoggedIn()
  redirectToLoginPageIfNecessary()
})

async function logout() {
  await doLogout()
  redirectToLoginPageIfNecessary()
}
</script>

<template>
  <header v-cloak>
    <nav class="nav" v-if="appStore.isLoggedIn">
      <div class="nav-left">
        <a class="brand" href="#"> <img alt="logo" src="/logo.png" />Distributed player </a>
        <div class="tabs">
          <RouterLink :to="{ name: 'playlists' }" active-class="active">{{
            $t('menus.schedule')
          }}</RouterLink>
          <RouterLink to="/files">{{ $t('menus.files') }}</RouterLink>
          <RouterLink to="/logs">{{ $t('menus.logs') }}</RouterLink>
        </div>
      </div>
      <div class="nav-right">
        <a class="button outline" @click="logout">{{ $t('menus.logout') }}</a>
      </div>
    </nav>
  </header>

  <router-view v-slot="{ Component }">
    <transition mode="out-in">
      <component :is="Component" />
    </transition>
  </router-view>

  <AppNotification></AppNotification>
</template>

<style scoped>
header {
  padding-bottom: 15px;
}
</style>
