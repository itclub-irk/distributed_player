<script setup lang="ts">
import { computed, ref } from 'vue'
import type { WorkingHoursSchedule } from '@/models/Playlist'
import { i18n } from '@/main'

const props = defineProps<{
  workingHoursSchedule: WorkingHoursSchedule
  dayIndex: number
}>()

const label = computed(() => {
  const i = props.dayIndex
  const t = i18n.global.t

  let d = {
    0: 'labels.monday',
    1: 'labels.tuesday',
    2: 'labels.wednesday',
    3: 'labels.thursday',
    4: 'labels.friday',
    5: 'labels.saturday',
    6: 'labels.sunday'
  }[i]

  if (!d) d = 'labels.monday'

  return t(d)
})

function isDataValid(data: string) {
  return data
}
</script>
<template>
  <span>
    <label>{{ label }}</label>
    <p class="grouped">
      <input
        type="time"
        v-model="workingHoursSchedule[dayIndex][0].$__toml_private_datetime"
        :class="{ error: !isDataValid(workingHoursSchedule[dayIndex][0].$__toml_private_datetime) }"
      />
      <input
        type="time"
        v-model="workingHoursSchedule[dayIndex][1].$__toml_private_datetime"
        :class="{ error: !isDataValid(workingHoursSchedule[dayIndex][1].$__toml_private_datetime) }"
      />
    </p>
  </span>
</template>
<style scoped></style>
