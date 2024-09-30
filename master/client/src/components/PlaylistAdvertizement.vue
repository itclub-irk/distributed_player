<script setup lang="ts">
import type { Advertizement, AdvertizementScheduleElement } from '@/models/Playlist'
import { computed, ref, watch } from 'vue'
import AppConfirmDialog from './AppConfirmDialog.vue'
import AppChooseFileDialog from './AppChooseFileDialog.vue'
import { i18n } from '@/main'
import AppFileField from './AppFileField.vue'

const props = defineProps<{
  advertizement?: Advertizement
  defaultAdvertizement?: Advertizement
  isDefaultMode: boolean
}>()

const useDefaultAdvertizement = ref(true)
const rawAdvertizement = ref<Advertizement>({ start_jingle: '', end_jingle: '', schedule: [] })
const confirmDialog = ref<typeof AppConfirmDialog>()
const chooseFileDialog = ref<typeof AppChooseFileDialog>()

watch(
  () => props.advertizement,
  () => {
    useDefaultAdvertizement.value = !props.advertizement

    if (!props.advertizement) return

    rawAdvertizement.value = JSON.parse(JSON.stringify(props.advertizement))
  },
  { deep: true }
)

const isDataValid = computed(() => {
  return true
})

const cleanedData = computed(() => {
  if (!isDataValid.value) return props.advertizement

  if (useDefaultAdvertizement.value) return undefined

  return rawAdvertizement.value
})

defineExpose({ isDataValid, cleanedData })
</script>
<template>
  <h3>{{ $t('labels.advertizement') }}</h3>

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

    <div class="row" v-for="(scheduleElement, _, index) of rawAdvertizement.schedule">
      <div class="col">
        <h4>{{ $t('labels.advertizement_block') }} №{{ index + 1 }}</h4>
      </div>
    </div>
  </div>

  <AppConfirmDialog ref="confirmDialog"></AppConfirmDialog>
  <AppChooseFileDialog ref="chooseFileDialog"></AppChooseFileDialog>
</template>
<style></style>
