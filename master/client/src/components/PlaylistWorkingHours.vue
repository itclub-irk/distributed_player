<script setup lang="ts">
import type {
  WorkingHours,
  WorkingHoursScheduleElement,
  WorkingHoursSchedule,
  WorkingHoursException
} from '@/models/Playlist'
import { computed, defineExpose, ref, watch } from 'vue'
import PlaylistWorkingHoursScheduleInput from './PlaylistWorkingHoursScheduleInput.vue'

const DEFAULT_WORKING_HOURS_START_TIME = '08:00:00'
const DEFAULT_WORKINT_HOURS_END_TIME = '20:00:00'

type RawWorkingHoursExceptionElement = [date: string, startTime: string, endTime: string]

const props = defineProps<{
  workingHours?: WorkingHours
  defaultWorkingHours?: WorkingHours
  isDefaultMode: boolean
}>()
const useDefaultWorkingHours = ref(true)

function getWorkingHoursScheduleElement(
  startTime: string,
  endTime: string
): WorkingHoursScheduleElement {
  return [{ $__toml_private_datetime: startTime }, { $__toml_private_datetime: endTime }]
}

const rawWorkingHoursSchedule = ref<WorkingHoursSchedule>([
  getWorkingHoursScheduleElement(DEFAULT_WORKING_HOURS_START_TIME, DEFAULT_WORKINT_HOURS_END_TIME),
  getWorkingHoursScheduleElement(DEFAULT_WORKING_HOURS_START_TIME, DEFAULT_WORKINT_HOURS_END_TIME),
  getWorkingHoursScheduleElement(DEFAULT_WORKING_HOURS_START_TIME, DEFAULT_WORKINT_HOURS_END_TIME),
  getWorkingHoursScheduleElement(DEFAULT_WORKING_HOURS_START_TIME, DEFAULT_WORKINT_HOURS_END_TIME),
  getWorkingHoursScheduleElement(DEFAULT_WORKING_HOURS_START_TIME, DEFAULT_WORKINT_HOURS_END_TIME),
  getWorkingHoursScheduleElement(DEFAULT_WORKING_HOURS_START_TIME, DEFAULT_WORKINT_HOURS_END_TIME),
  getWorkingHoursScheduleElement(DEFAULT_WORKING_HOURS_START_TIME, DEFAULT_WORKINT_HOURS_END_TIME)
])

const rawWorkingHoursExceptions = ref<RawWorkingHoursExceptionElement[]>([])

watch(
  () => props.workingHours,
  () => {
    useDefaultWorkingHours.value = !props.workingHours

    if (!props.workingHours) return

    rawWorkingHoursSchedule.value = JSON.parse(JSON.stringify(props.workingHours.schedule))

    const rawExceptions: RawWorkingHoursExceptionElement[] = []
    for (const [exceptionDate, scheduleElement] of Object.entries(props.workingHours.exceptions)) {
      rawExceptions.push([
        exceptionDate,
        scheduleElement[0].$__toml_private_datetime,
        scheduleElement[1].$__toml_private_datetime
      ])
    }

    rawWorkingHoursExceptions.value = rawExceptions.sort(
      (a: RawWorkingHoursExceptionElement, b: RawWorkingHoursExceptionElement) =>
        new Date(a[0]).getTime() - new Date(b[0]).getTime()
    )
  },
  { deep: true }
)

const isDataValid = computed(() => {
  if (useDefaultWorkingHours.value) return true

  return rawWorkingHoursSchedule.value.reduce(
    (accumulator, currentValue) =>
      accumulator &&
      !!currentValue[0].$__toml_private_datetime &&
      !!currentValue[1].$__toml_private_datetime,
    true
  )
})

const cleanedData = computed(() => {
  if (!isDataValid.value) return props.workingHours

  if (useDefaultWorkingHours.value) return undefined

  const cleanedData: WorkingHours = { schedule: rawWorkingHoursSchedule.value, exceptions: {} }
  for (const [day, startTime, endTime] of rawWorkingHoursExceptions.value) {
    cleanedData.exceptions[day] = getWorkingHoursScheduleElement(startTime, endTime)
  }
  return cleanedData
})

defineExpose({ isDataValid, cleanedData })

function getDayIndex(pairIndex: number, colNumber: number): number {
  return 2 * pairIndex + colNumber
}

function deleteExceptionRow(rowIndex: number) {
  rawWorkingHoursExceptions.value.splice(rowIndex, 1)
}

function addExceptionRow() {
  rawWorkingHoursExceptions.value.push(['1970-01-01', '00:00:00', '00:00:00'])
}
</script>
<template>
  <h3>{{ $t('labels.working_hours') }}</h3>
  <div class="row" v-if="!isDefaultMode">
    <div class="col">
      <input name="checkbox" type="checkbox" v-model="useDefaultWorkingHours" />
      {{ $t('labels.use_default_schedule') }}
    </div>
  </div>

  <div class="config_form__schedule" v-show="isDefaultMode || !useDefaultWorkingHours">
    <p>{{ $t('labels.fills_working_hours') }}</p>
    <div class="row" v-for="pairIndex of Array(4).keys()">
      <div class="col" v-for="index of Array(2).keys()">
        <PlaylistWorkingHoursScheduleInput
          v-if="getDayIndex(pairIndex, index) < 7"
          :dayIndex="getDayIndex(pairIndex, index)"
          :workingHoursSchedule="rawWorkingHoursSchedule"
        ></PlaylistWorkingHoursScheduleInput>
      </div>
    </div>
  </div>

  <h3>{{ $t('labels.working_hours_exceptions') }}</h3>
  <p>{{ $t('labels.fills_working_hours_exceptions') }}</p>
  <div class="row">
    <div class="col">
      <label>{{ $t('labels.date') }}</label>
    </div>
    <div class="col">
      <label>{{ $t('labels.start_time') }}</label>
    </div>
    <div class="col">
      <label>{{ $t('labels.end_time') }}</label>
    </div>
    <div class="col"></div>
  </div>
  <div class="row" v-for="(exception, index) of rawWorkingHoursExceptions">
    <div class="col">
      <input type="date" v-model="exception[0]" />
    </div>
    <div class="col">
      <input type="time" v-model="exception[1]" />
    </div>
    <div class="col">
      <input type="time" v-model="exception[2]" />
    </div>
    <div class="col">
      <button type="button" class="button error" @click="deleteExceptionRow(index)">
        {{ $t('controls.delete') }}
      </button>
    </div>
  </div>
  <button type="button" class="button primary outline" @click="addExceptionRow">
    {{ $t('controls.add') }}
  </button>
</template>
<style scoped></style>
