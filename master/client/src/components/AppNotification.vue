<script setup lang="ts">
import { useAppStore } from '@/stores/app'
import { ref, watch } from 'vue'

const appStore = useAppStore()

const dialog = ref<HTMLDialogElement>()

watch(
  () => appStore.notificationText,
  (newText: string) => {
    if (!dialog.value) return

    if (newText) {
      dialog.value.style.display = 'initial'
    } else {
      dialog.value.style.display = 'none'
    }
  }
)
</script>

<template>
  <div
    class="dialog"
    style="display: none"
    ref="dialog"
    :class="{
      success: appStore.notificationLevel === 'success',
      error: appStore.notificationLevel === 'error'
    }"
  >
    <p>{{ appStore.notificationText }}</p>
  </div>
</template>

<style scoped>
.dialog {
  position: fixed;
  bottom: 0;
  right: 0;
  height: 80px;
  width: 250px;
  border-radius: 10px;
  margin-right: 10px;
  margin-bottom: 10px;
  color: white;
  padding: 10px;
}

.success {
  background-color: var(--color-success);
}

.error {
  background-color: var(--color-error);
}
</style>
