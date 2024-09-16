<script setup lang="ts">
import { onMounted, ref } from 'vue'
import { useAppStore } from '@/stores/app'
import { getPlaylistsNames } from '@/services/playlistService'

const playlistsNames = ref<string[]>([])

onMounted(async () => {
  playlistsNames.value = await getPlaylistsNames()
})
</script>
<template>
  <!-- <div class="row">
    <div class="col nodes_list">
      <button class="button primary">{{ $t('controls.general_config') }}</button>
    </div>
  </div> -->
  <div class="row">
    <div class="col">
      <!-- <h3>{{ $t('labels.node_list') }}</h3> -->

      <div class="row">
        <div class="col">
          <form>
            <p class="grouped">
              <input
                type="text"
                value=""
                name="node_filter"
                :placeholder="$t('controls.node_filter')"
              />
              <button class="button outline" type="button">{{ $t('controls.add_node') }}</button>
            </p>
          </form>
        </div>
      </div>

      <div class="nodes_list">
        <RouterLink to="/playlists/default" class="button" active-class="primary">{{
          $t('controls.default_playlist')
        }}</RouterLink>
        <RouterLink
          :to="{ name: 'edit_playlist', params: { playlist_name: playlistName } }"
          class="button"
          active-class="primary"
          v-for="playlistName of playlistsNames"
          >{{ playlistName }}</RouterLink
        >
      </div>
    </div>
  </div>
</template>
<style scoped>
.nodes_list {
  display: flex;
  flex-flow: column wrap;
  justify-content: center;
  row-gap: 5px;
  flex-basis: auto;
}

.nodes_list button {
  margin-left: 0;
}

.button:not(.primary) {
  background-color: transparent;
  border-color: var(--color-lightGrey);
}

.button + .button {
  margin-left: 0;
}
</style>
