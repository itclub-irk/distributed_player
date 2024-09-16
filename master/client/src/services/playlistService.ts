import { useAppStore } from '@/stores/app'
import { getServerBaseUrl } from '@/utils/server'
import { type Playlist } from '@/models/Playlist'

export async function getPlaylistsNames(): Promise<string[]> {
  const url = new URL('/api/playlist', getServerBaseUrl())
  const response = await fetch(url, {
    method: 'GET',
    credentials: 'include'
  })

  if (response.status == 200) {
    const payload = await response.json()
    return payload.playlists as string[]
  }
  return []
}

export async function getPlaylistByName(playlistName: string): Promise<Playlist | undefined> {
  const url = new URL(`/api/playlist/${playlistName}`, getServerBaseUrl())
  const response = await fetch(url, {
    method: 'GET',
    credentials: 'include'
  })

  if (response.status == 200) {
    return response.json()
  }
  return undefined
}

export async function updatePlaylist(playlistName: string, playlist: Playlist) {
  const url = new URL(`/api/playlist/${playlistName}`, getServerBaseUrl())
  const response = await fetch(url, {
    headers: new Headers({ 'content-type': 'application/json' }),
    body: JSON.stringify(playlist),
    method: 'POST',
    credentials: 'include'
  })
}
