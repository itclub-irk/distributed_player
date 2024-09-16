<script setup lang="ts">
import { ref } from 'vue'
import { doLogin } from '@/services/authService'
import { useRouter } from 'vue-router'
import { useAppStore } from '@/stores/app'
import { i18n } from '@/main'

const password = ref('')
const appStore = useAppStore()
const router = useRouter()
const isError = ref(false)

async function submit() {
  await doLogin(password.value)
  if (appStore.isLoggedIn) {
    router.push({ path: '/' })
    appStore.showSuccessNotification(i18n.global.t('messages.success_login'))
  } else {
    appStore.showErrorNotification(i18n.global.t('messages.wrong_password'))
    isError.value = true
  }
}
</script>
<template>
  <div class="login_form">
    <div class="card">
      <header>
        <h4>Distributed player</h4>
      </header>
      <form @submit.prevent="submit">
        <p>
          <label for="password_input">{{ $t('controls.password') }}</label>
          <input
            id="password_input"
            type="password"
            :placeholder="$t('controls.enter_master_node_password')"
            v-model="password"
            :class="{ error: isError }"
          />
        </p>
      </form>
      <footer class="is-right">
        <button class="button primary" type="button" @click="submit">
          {{ $t('controls.login') }}
        </button>
      </footer>
    </div>
  </div>
</template>
<style scoped>
.login_form {
  height: 90vh;
  display: flex;
  align-items: center;
  justify-content: center;
}

.card {
  width: 400px;
}

form {
  margin-bottom: 10px;
}
</style>
