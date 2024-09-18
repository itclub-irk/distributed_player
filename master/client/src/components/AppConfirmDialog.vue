<script setup lang="ts">
import { ref } from 'vue'
import { i18n } from '@/main'

const dialog = ref<HTMLDialogElement>()
const message = ref<string>()
const acceptButton = ref<HTMLButtonElement>()
const closeButton = ref<HTMLButtonElement>()

defineExpose({ show })

/**
 * Show dialog
 * @param [msg='Confirm action?'] text inside dialog window
 */
function show(msg?: string): Promise<boolean> {
  if (!msg) {
    msg = i18n.global.t('labels.are_you_shure_you_want_to_do_this')
  }

  message.value = msg
  dialog.value?.showModal()
  return new Promise((resolve) => {
    if (acceptButton.value) {
      acceptButton.value.addEventListener('click', () => {
        dialog.value?.close()
        resolve(true)
      })
    }
    if (closeButton.value) {
      closeButton.value.addEventListener('click', () => {
        dialog.value?.close()
        resolve(false)
      })
    }
  })
}
</script>
<template>
  <dialog class="confirm_dialog" ref="dialog">
    <div class="row">
      <div class="col">
        <h4>{{ $t('labels.confirm_action') }}</h4>
        <p>{{ message }}</p>
      </div>
    </div>
    <div class="row">
      <div class="col">
        <button ref="acceptButton" class="button error" type="button">
          {{ $t('controls.yes') }}
        </button>
      </div>
      <div class="col is-right">
        <button ref="closeButton" class="button outline" type="button">
          {{ $t('controls.no') }}
        </button>
      </div>
    </div>
  </dialog>
</template>
<style scoped>
.confirm_dialog:focus {
  outline: none;
}

.confirm_dialog {
  border: none;
  border-radius: 4px;
}
</style>
