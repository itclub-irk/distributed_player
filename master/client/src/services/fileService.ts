import type { FolderEntry } from '@/models/FolderEntry'
import { useAppStore } from '@/stores/app'
import { getServerBaseUrl } from '@/utils/server'

export async function getListFiles(
  requestedPath: string,
  foldersOnly: boolean
): Promise<FolderEntry[]> {
  const url = new URL('/api/file', getServerBaseUrl())
  url.search = new URLSearchParams({
    folders_only: foldersOnly.toString(),
    requested_path: requestedPath
  }).toString()

  const response = await fetch(url, {
    method: 'GET',
    credentials: 'include'
  })

  if (response.status == 200) {
    const f = (await response.json()).files as FolderEntry[]
    return f
      .sort((a, b) => a.name.localeCompare(b.name))
      .sort((a, b) => (a.is_file === b.is_file ? 0 : a.is_file ? 1 : -1))
  }

  return []
}
