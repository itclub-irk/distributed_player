<script setup lang="ts">
import { ref } from 'vue'
import { i18n } from '@/main'
import { getListFiles } from '@/services/fileService'
import { type CollapsableFolderEntry } from '@/models/CollapsableFolderEntry'
import AppFolderEntryTreeNode from './AppFolderEntryTreeNode.vue'
import { useAppStore } from '@/stores/app'

const dialog = ref<HTMLDialogElement>()
const chooseButton = ref<HTMLButtonElement>()
const closeButton = ref<HTMLButtonElement>()

const chooseFolderMode = ref(false)
const chooseFileMode = ref(true)

const files = ref<CollapsableFolderEntry[]>([])
const appStore = useAppStore()
let resolver: (value: string | PromiseLike<string | undefined> | undefined) => void

defineExpose({ show })

function initHTMLButtonElement(srcElement: HTMLButtonElement) {
  const cloned = srcElement.cloneNode(true)
  srcElement.parentNode?.replaceChild(cloned, srcElement)
}

function choose() {
  dialog.value?.close()
  const choosenFilePath = appStore.choosenFilePath
  appStore.choosenFilePath = undefined
  resolver(choosenFilePath)
}

function close() {
  dialog.value?.close()
  appStore.choosenFilePath = undefined
  resolver(undefined)
}

async function show(
  chooseFolder: boolean = false,
  chooseFile: boolean = true
): Promise<string | undefined> {
  chooseFolderMode.value = chooseFolder
  chooseFileMode.value = chooseFile

  files.value = await getListFiles('', chooseFolder)

  dialog.value?.showModal()

  return new Promise((resolve) => {
    resolver = resolve
  })
}
</script>
<template>
  <dialog class="choose_file_dialog" ref="dialog">
    <div class="row">
      <div class="col">
        <h4 v-if="!chooseFolderMode">{{ $t('labels.choose_file') }}</h4>
        <h4 v-if="chooseFolderMode">{{ $t('labels.choose_folder') }}</h4>
      </div>
    </div>
    <div class="row">
      <div class="col choose_file_dialog__tree">
        <AppFolderEntryTreeNode
          v-for="entry of files"
          :key="entry.path"
          :entry="entry"
          :chooseFolderMode="chooseFolderMode"
          :chooseFileMode="chooseFileMode"
        ></AppFolderEntryTreeNode>
      </div>
    </div>

    <div class="row">
      <div class="col">
        <button
          @click="choose"
          class="button primary"
          type="button"
          :disabled="!appStore.choosenFilePath"
        >
          {{ $t('controls.choose') }}
        </button>
      </div>
      <div class="col is-right">
        <button @click="close" class="button outline" type="button">
          {{ $t('controls.cancel') }}
        </button>
      </div>
    </div>
  </dialog>
</template>
<style scoped>
.choose_file_dialog {
  min-width: 30vw;
  border: none;
  border-radius: 4px;
}

.choose_file_dialog:focus {
  outline: none;
}

.choose_file_dialog__tree {
  height: 30vh;
  overflow: auto;
}
</style>
