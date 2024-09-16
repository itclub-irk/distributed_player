import './assets/styles.css'
import { createApp } from 'vue'
import { createPinia } from 'pinia'
import { createI18n } from 'vue-i18n'
import enUS from './locales/en-us.json'
import ruRU from './locales/ru-ru.json'

import App from './App.vue'
import router from './router'

type MessageSchema = typeof enUS
export const i18n = createI18n<[MessageSchema], 'en-US' | 'ru-RU'>({
  // locale: window.navigator.language,
  locale: 'ru-RU',
  fallbackLocale: 'en-US',
  messages: {
    'en-US': enUS,
    'ru-RU': ruRU
  },
  globalInjection: true
})

const app = createApp(App)

app.use(i18n)
app.use(createPinia())
app.use(router)

app.mount('#app')
