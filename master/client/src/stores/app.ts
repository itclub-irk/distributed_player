import type { PlaylistPage } from '@/models/PlaylistPage'
import { defineStore } from 'pinia'
import { ref } from 'vue'

const NOTIFICATION_TIMEOUT_SECONDS = 5

export const useAppStore = defineStore('appStore', () => {
  const isLoggedIn = ref(false)
  const selectedPlaylistName = ref<string>()

  const notificationText = ref('')
  const notificationLevel = ref<'success' | 'error'>('success')
  const notificationTimeout = ref()
  const choosenFilePath = ref()

  const currentPlaylistPage = ref<PlaylistPage>('working_hours')

  function showSuccessNotification(text: string) {
    showNotification(text, 'success')
  }

  function showErrorNotification(text: string) {
    showNotification(text, 'error')
  }

  function showNotification(text: string, level: 'success' | 'error') {
    notificationLevel.value = level
    notificationText.value = text

    if (notificationTimeout.value) clearTimeout(notificationTimeout.value)

    notificationTimeout.value = setTimeout(() => {
      notificationText.value = ''
      notificationTimeout.value = undefined
    }, NOTIFICATION_TIMEOUT_SECONDS * 1000)
  }

  return {
    isLoggedIn,
    selectedPlaylistName,
    notificationText,
    notificationLevel,
    choosenFilePath,
    currentPlaylistPage,
    showSuccessNotification,
    showErrorNotification
  }
})
