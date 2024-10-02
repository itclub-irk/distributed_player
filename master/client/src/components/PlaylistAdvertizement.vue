<script setup lang="ts">
import type { Advertizement, AdvertizementSchedule } from '@/models/Playlist'
import { computed, ref, watch } from 'vue'
import AppConfirmDialog from './AppConfirmDialog.vue'
import AppChooseFileDialog from './AppChooseFileDialog.vue'
import { i18n } from '@/main'
import AppFileField from './AppFileField.vue'

type RawAdvertizement = { start_jingle: string; end_jingle: string }
type RawSchedule = { time: string; folders: string[] }

const props = defineProps<{
  advertizement?: Advertizement
  defaultAdvertizement?: Advertizement
  isDefaultMode: boolean
}>()

const useDefaultAdvertizement = ref(true)
const rawAdvertizement = ref<RawAdvertizement>({
  start_jingle: '',
  end_jingle: ''
})
const rawSchedule = ref<RawSchedule[]>([])

const confirmDialog = ref<typeof AppConfirmDialog>()
const chooseFileDialog = ref<typeof AppChooseFileDialog>()

watch(
  () => props.advertizement,
  () => {
    useDefaultAdvertizement.value = !props.advertizement

    if (!props.advertizement) return

    rawAdvertizement.value = {
      start_jingle: props.advertizement.start_jingle,
      end_jingle: props.advertizement.end_jingle
    }

    const schedule: RawSchedule[] = []
    if (props.advertizement.schedule) {
      for (const [time, folders] of Object.entries(props.advertizement.schedule)) {
        schedule.push({ time, folders })
      }
    }

    rawSchedule.value = schedule.sort((a, b) => a.time.localeCompare(b.time))
  },
  { deep: true }
)

const isDataValid = computed(() => {
  return true
})

const cleanedData = computed(() => {
  if (!isDataValid.value) return props.advertizement

  if (useDefaultAdvertizement.value) return undefined

  const res: Advertizement = { ...rawAdvertizement.value, schedule: {} }
  for (let scheduleElement of rawSchedule.value) {
    res.schedule![scheduleElement.time] = scheduleElement.folders
  }
  return res
})

defineExpose({ isDataValid, cleanedData })

function addScheduleElement() {
  rawSchedule.value.push({ time: '00:01', folders: [] })
}

async function deleteScheduleElement(elementIndex: number) {
  if (confirmDialog.value && (await confirmDialog.value.show()))
    rawSchedule.value.splice(elementIndex, 1)
}

async function addDir(scheduleElement: RawSchedule) {
  if (!chooseFileDialog.value) return

  const dirs = scheduleElement.folders
  const choosenDir = await chooseFileDialog.value.show(true, false)
  if (choosenDir) {
    dirs.push(choosenDir)
  }
}

async function deleteDir(scheduleElement: RawSchedule, dirIndex: number) {
  if (confirmDialog.value && (await confirmDialog.value.show()))
    scheduleElement.folders.splice(dirIndex, 1)
}
</script>
<template>
  <div class="row" v-if="!isDefaultMode">
    <div class="col">
      <input name="checkbox" type="checkbox" v-model="useDefaultAdvertizement" />
      {{ $t('labels.use_default_schedule') }}
    </div>
  </div>
  <div v-show="isDefaultMode || !useDefaultAdvertizement">
    <div class="row">
      <div class="col">
        <label>{{ $t('controls.start_jingle') }}</label>

        <AppFileField
          :model="rawAdvertizement.start_jingle"
          @change="(newValue) => (rawAdvertizement.start_jingle = newValue)"
          @delete="rawAdvertizement.start_jingle = ''"
          :chooseFile="true"
        ></AppFileField>
      </div>
    </div>
    <div class="row">
      <div class="col">
        <label>{{ $t('controls.end_jingle') }}</label>
        <AppFileField
          :model="rawAdvertizement.end_jingle"
          @change="(newValue) => (rawAdvertizement.end_jingle = newValue)"
          @delete="rawAdvertizement.end_jingle = ''"
          :chooseFile="true"
        ></AppFileField>
      </div>
    </div>

    <p>
      {{ $t('labels.advertizement_schedule_description') }}
    </p>

    <div class="row" v-for="(scheduleElement, index) of rawSchedule">
      <div class="col">
        <h4>{{ $t('labels.advertizement_block') }} №{{ index + 1 }}</h4>
        <div class="row">
          <div class="col">
            <label>{{ $t('controls.time') }}</label>
            <input type="time" min="00:01" max="00:59" v-model="scheduleElement.time" />
          </div>
        </div>
        <div class="row">
          <div class="col">
            <AppFileField
              v-for="(dir, dirIndex) of scheduleElement.folders"
              :model="dir"
              @change="(newValue) => (scheduleElement.folders[dirIndex] = newValue)"
              @delete="deleteDir(scheduleElement, dirIndex)"
              :chooseFolder="true"
            ></AppFileField>

            <div class="row">
              <div class="col">
                <button
                  type="button"
                  class="button primary outline"
                  @click="addDir(scheduleElement)"
                >
                  {{ $t('controls.add_directory') }}
                </button>
              </div>
              <div class="col is-right">
                <button type="button" class="button error" @click="deleteScheduleElement(index)">
                  {{ $t('controls.delete_advertizement_block') }}
                </button>
              </div>
            </div>
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

  <AppConfirmDialog ref="confirmDialog"></AppConfirmDialog>
  <AppChooseFileDialog ref="chooseFileDialog"></AppChooseFileDialog>
</template>
<style></style>
