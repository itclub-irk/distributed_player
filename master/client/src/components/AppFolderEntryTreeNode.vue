<script setup lang="ts">
import type { CollapsableFolderEntry } from '@/models/CollapsableFolderEntry'
import { computed, ref } from 'vue'
import FolderIcon from '../assets/folder.png'
import OpenedFolderIcon from '../assets/opened_folder.png'
import FileIcon from '../assets/file.png'
import { useAppStore } from '@/stores/app'
import { getListFiles } from '@/services/fileService'

const iconSize = ref(25)
const props = defineProps<{
  entry: CollapsableFolderEntry
  chooseFolderMode?: boolean
  chooseFileMode?: boolean
}>()

const appStore = useAppStore()

const iconPath = computed(() => {
  let iconFile = FileIcon
  if (!props.entry.is_file) {
    iconFile = FolderIcon
    if (props.entry.is_opened) iconFile = OpenedFolderIcon
  }

  return iconFile
})

async function handleIconClick() {
  if (props.entry.is_file) {
    selectEntry()
  } else {
    if (props.entry.is_opened) {
      props.entry.is_opened = false
    } else {
      await loadNested()
      props.entry.is_opened = true
    }
  }
}

function selectEntry() {
  if (props.chooseFileMode && !props.entry.is_file) return

  if (appStore.choosenFilePath !== props.entry.path) {
    appStore.choosenFilePath = props.entry.path
  } else {
    appStore.choosenFilePath = undefined
  }
}

async function loadNested() {
  props.entry.nested = await getListFiles(props.entry.path, props.chooseFolderMode)
}
</script>
<template>
  <div class="folder_entry clickable" @click="selectEntry">
    <img
      alt="folder entry icon"
      class="folder_entry__icon"
      @click.stop="handleIconClick"
      :src="iconPath"
      :height="iconSize"
      :width="iconSize"
    />
    <span :class="{ choosen: appStore.choosenFilePath === entry.path }" class="folder_entry__name">
      {{ entry.name }}
    </span>
  </div>
  <div class="folder_entry__nested_list" v-if="entry.is_opened">
    <span v-if="entry.nested?.length === 0" class="text-light">Папка пуста</span>
    <AppFolderEntryTreeNode
      v-for="nestedEntry of entry.nested"
      :key="nestedEntry.path"
      :entry="nestedEntry"
      :chooseFolderMode="chooseFileMode"
      :chooseFileMode="chooseFileMode"
    ></AppFolderEntryTreeNode>
  </div>
</template>
<style>
.folder_entry {
  width: 100%;
  height: 30px;
}

.folder_entry__icon {
  vertical-align: bottom;
  margin-right: 5px;
}

.folder_entry__name {
  width: 100%;
}

.folder_entry__nested_list {
  margin-left: 20px;
}

.choosen {
  color: var(--color-primary) !important;
  font-weight: 800;
}
</style>
