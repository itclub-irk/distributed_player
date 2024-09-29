<script setup lang="ts">
import PlaylistWorkingHours from '../components/PlaylistWorkingHours.vue'
import { useAppStore } from '@/stores/app'
import { onBeforeRouteUpdate, useRoute, useRouter } from 'vue-router'
import { computed, onBeforeMount, onMounted, ref, watch } from 'vue'
import { i18n } from '@/main'
import { getPlaylistByName, updatePlaylist } from '@/services/playlistService'
import type { Playlist } from '@/models/Playlist'
import PlaylistMusic from '@/components/PlaylistMusic.vue'
import PlaylistAdvertizement from '@/components/PlaylistAdvertizement.vue'

const appStore = useAppStore()
const route = useRoute()
const router = useRouter()

const playlistName = ref<string>()
const isDefaultMode = ref(true)

const playlist = ref<Playlist>({})
const defaultPlaylist = ref<Playlist>({})

const useDefaultWorkingHours = ref(true)

const workingHours = ref<typeof PlaylistWorkingHours>()
const music = ref<typeof PlaylistMusic>()

function redirectToDefaultPlaylist() {
  router.push({ name: 'edit_playlist', params: { playlist_name: 'default' } })
}

async function initView() {
  const params = route.params
  if (!params || !params['playlist_name'] || Array.isArray(params['playlist_name'])) {
    redirectToDefaultPlaylist()
    return
  }

  let n = params['playlist_name']

  playlistName.value = n
  isDefaultMode.value = false

  if (n === 'default') {
    isDefaultMode.value = true
    playlistName.value = i18n.global.t('controls.default_playlist')

    const d = await getPlaylistByName(n)
    if (d) playlist.value = d
  } else {
    const [d, p] = await Promise.all([getPlaylistByName('default'), getPlaylistByName(n)])
    if (d) {
      defaultPlaylist.value = d
    }
    if (p) {
      playlist.value = p

      if (p.working_hours) {
        useDefaultWorkingHours.value = false
      }
    } else {
      redirectToDefaultPlaylist()
    }
  }
}

onMounted(() => {
  initView()
})

watch(route, () => {
  initView()
})

const isDataValid = computed(() => {
  if (!workingHours.value || !music.value) return false

  return workingHours.value.isDataValid && music.value.isDataValid
})

async function submit() {
  if (!isDataValid.value) {
    appStore.showErrorNotification(i18n.global.t('messages.validation_error'))
    return
  }

  const n = isDefaultMode.value ? 'default' : playlistName.value

  if (!n || !playlist.value || !workingHours.value || !music.value) return

  const updatedData: Playlist = {
    ...playlist.value
  }

  updatedData.working_hours = workingHours.value.cleanedData
  updatedData.music = music.value.cleanedData

  await updatePlaylist(n, updatedData)
  appStore.showSuccessNotification(i18n.global.t('messages.data_saved_success'))
}
</script>
<template>
  <div>
    <form class="config_form" @submit.prevent="submit">
      <div class="row">
        <div class="col-8">
          <h2>{{ playlistName }}</h2>
        </div>
      </div>
      <div class="row">
        <div class="col-8">
          <PlaylistWorkingHours
            ref="workingHours"
            :workingHours="playlist.working_hours"
            :defaultWorkingHours="defaultPlaylist.working_hours"
            :isDefaultMode="isDefaultMode"
          ></PlaylistWorkingHours>
        </div>
      </div>

      <div class="row">
        <div class="col-8">
          <PlaylistMusic
            ref="music"
            :music="playlist.music"
            :defaultMusic="defaultPlaylist.music"
            :isDefaultMode="isDefaultMode"
          ></PlaylistMusic>
        </div>
      </div>

      <div class="row">
        <div class="col-8">
          <PlaylistAdvertizement
            ref="advertizement"
            :advertizement="playlist.advertizement"
            :defaultAdvertizement="defaultPlaylist.advertizement"
            :isDefaultMode="isDefaultMode"
          ></PlaylistAdvertizement>
        </div>
      </div>

      <div class="row">
        <div class="col-8">
          <button type="submit" class="button primary">{{ $t('controls.save') }}</button>
        </div>
      </div>
    </form>
  </div>
</template>
<style scoped></style>
