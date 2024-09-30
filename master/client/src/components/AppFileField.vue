<script setup lang="ts">
import { onMounted, ref, watch } from 'vue'
import AppChooseFileDialog from './AppChooseFileDialog.vue'

const props = defineProps<{
  model: string
  chooseFolder?: boolean
  chooseFile?: boolean
}>()
const chooseFileDialog = ref<typeof AppChooseFileDialog>()
const rawValue = ref<string>()

const emit = defineEmits<{
  (e: 'change', newValue: string): void
  (e: 'delete'): void
}>()

onMounted(() => {
  rawValue.value = props.model
})

watch(
  () => props.model,
  (newModel) => {
    rawValue.value = newModel
  }
)

async function openChooseFileDialog() {
  if (!chooseFileDialog.value) return

  const chooseFolder = props.chooseFolder || false
  const chooseFile = props.chooseFile || false
  const choosenPath = await chooseFileDialog.value.show(chooseFolder, chooseFile)

  if (!choosenPath) return
  emit('change', choosenPath)
}

function deleteValue() {
  emit('delete')
}
</script>
<template>
  <p class="grouped">
    <input
      :readonly="true"
      class="clickable"
      type="text"
      v-model="rawValue"
      @click="openChooseFileDialog"
    />
    <button type="button" class="button error" @click="deleteValue">
      {{ $t('controls.delete') }}
    </button>
  </p>

  <AppChooseFileDialog ref="chooseFileDialog"></AppChooseFileDialog>
</template>
<style scoped></style>
