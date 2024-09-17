<script setup lang="ts">
import type { Music, MusicScheduleElement } from '@/models/Playlist'
import PlaylistMusicScheduleInput from './PlaylistMusicScheduleInput.vue'
import { ref, watch } from 'vue'

const props = defineProps<{
  music?: Music
  defaultMusic?: Music
  isDefaultMode: boolean
}>()

const useDefaultMusic = ref(true)
const rawMusic = ref<Music>({ shuffle: false, schedule: [] })

watch(
  () => props.music,
  () => {
    useDefaultMusic.value = !props.music

    if (!props.music) return

    rawMusic.value = JSON.parse(JSON.stringify(props.music))
    console.log(rawMusic.value)
  },
  { deep: true }
)

function addScheduleElement() {
  const initialDate = '1970-01-01'
  rawMusic.value.schedule.push([
    { $__toml_private_datetime: initialDate },
    { $__toml_private_datetime: initialDate },
    []
  ])
}

function deleteScheduleElement(elementIndex: number) {
  rawMusic.value.schedule.splice(elementIndex, 1)
}

function addDir(scheduleElement: MusicScheduleElement) {
  scheduleElement[2].push('')
}

function deleteDir(scheduleElement: MusicScheduleElement, dirIndex: number) {
  scheduleElement[2].splice(dirIndex, 1)
}
</script>
<template>
  <h3>{{ $t('labels.music') }}</h3>

  <div class="row" v-if="!isDefaultMode">
    <div class="col">
      <input name="checkbox" type="checkbox" v-model="useDefaultMusic" />
      {{ $t('labels.use_default_schedule') }}
    </div>
  </div>

  <div class="config_form__schedule" v-show="isDefaultMode || !useDefaultMusic">
    <p>
      {{ $t('labels.shuffle_mode_description') }}
    </p>

    <div class="row">
      <div class="col">
        <p class="grouped">
          <button
            type="button"
            class="button"
            :class="{ primary: !rawMusic.shuffle, outline: rawMusic.shuffle }"
            @click="rawMusic.shuffle = false"
          >
            {{ $t('controls.play_in_order') }}
          </button>
          <button
            type="button"
            class="button"
            :class="{ primary: rawMusic.shuffle, outline: !rawMusic.shuffle }"
            @click="rawMusic.shuffle = true"
          >
            {{ $t('controls.shuffle') }}
          </button>
        </p>
      </div>
    </div>
    <p>
      {{ $t('labels.music_schedule_description') }}
    </p>
    <div class="row" v-if="rawMusic.schedule.length == 0">
      <div class="col text-light">{{ $t('labels.no_intervals_added') }}</div>
    </div>

    <div class="row" v-for="(scheduleElement, index) of rawMusic.schedule">
      <div class="col">
        <h4>{{ $t('labels.interval') }} №{{ index + 1 }}</h4>
        <div class="row">
          <div class="col">{{ $t('labels.start_date') }}</div>
          <div class="col">{{ $t('labels.end_date') }}</div>
        </div>
        <div class="row">
          <div class="col">
            <input type="date" v-model="scheduleElement[0].$__toml_private_datetime" />
          </div>
          <div class="col">
            <input type="date" v-model="scheduleElement[1].$__toml_private_datetime" />
          </div>
        </div>
        <p>{{ $t('labels.directories') }}</p>
        <div class="row">
          <div class="col">
            <p v-if="!scheduleElement[2].length" class="text-light">
              {{ $t('labels.no_directories_added') }}
            </p>
            <p class="grouped" v-for="(dir, dirIndex) of scheduleElement[2]">
              <input type="text" v-model="scheduleElement[2][dirIndex]" />
              <button
                type="button"
                class="button error"
                @click="deleteDir(scheduleElement, dirIndex)"
              >
                {{ $t('controls.delete') }}
              </button>
            </p>
          </div>
        </div>
        <div class="row">
          <div class="col">
            <button type="button" class="button primary outline" @click="addDir(scheduleElement)">
              {{ $t('controls.add_directory') }}
            </button>
          </div>
          <div class="col is-right">
            <button type="button" class="button error" @click="deleteScheduleElement(index)">
              {{ $t('controls.delete_interval') }}
            </button>
          </div>
        </div>
      </div>
    </div>
    <div class="row">
      <div class="col">
        <button type="button" class="button primary outline" @click="addScheduleElement">
          {{ $t('controls.add') }}
        </button>
      </div>
    </div>
  </div>
</template>
<style scoped></style>
