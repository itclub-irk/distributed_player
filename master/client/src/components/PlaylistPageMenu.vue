<script setup lang="ts">
import type { PlaylistPage } from '@/models/PlaylistPage'
import { useAppStore } from '@/stores/app'
import { computed } from 'vue'

const appStore = useAppStore()

function changePage(newPage: PlaylistPage) {
  appStore.currentPlaylistPage = newPage
}

const currentPage = computed(() => {
  return appStore.currentPlaylistPage
})
</script>
<template>
  <nav class="tabs">
    <a
      class="clickable"
      @click="changePage('working_hours')"
      :class="{ active: currentPage === 'working_hours' }"
      >{{ $t('labels.working_hours') }}</a
    >
    <a
      class="clickable"
      @click="changePage('music')"
      :class="{ active: currentPage === 'music' }"
      >{{ $t('labels.music') }}</a
    >
    <a
      class="clickable"
      @click="changePage('advertizement')"
      :class="{ active: currentPage === 'advertizement' }"
      >{{ $t('labels.advertizement') }}</a
    >
  </nav>
</template>
<style scoped></style>
