import { useAppStore } from '@/stores/app'
import { getServerBaseUrl } from '@/utils/server'

/**
 * Checks is user logged in
 */
export async function checkIsLoggedIn() {
  const appStore = useAppStore()
  const url = new URL('/api/auth/is_logged_in', getServerBaseUrl())
  const response = await fetch(url, {
    method: 'GET',
    credentials: 'include'
  })

  if (response.status == 200) {
    appStore.isLoggedIn = true
  } else {
    appStore.isLoggedIn = false
  }
}

/**
 * Performs user authentication process
 * @param password master node password
 */
export async function doLogin(password: string) {
  const appStore = useAppStore()

  const url = new URL('/api/auth/login', getServerBaseUrl())
  const response = await fetch(url, {
    headers: new Headers({ 'content-type': 'application/json' }),
    body: JSON.stringify({ password }),
    method: 'POST',
    credentials: 'include'
  })

  if (response.status == 200) {
    appStore.isLoggedIn = true
  } else {
    appStore.isLoggedIn = false
  }
}

/**
 * Performs user logout process
 */
export async function doLogout() {
  const appStore = useAppStore()

  const url = new URL('/api/auth/logout', getServerBaseUrl())
  const response = await fetch(url, {
    method: 'GET',
    credentials: 'include'
  })

  if (response.status == 200) {
    appStore.isLoggedIn = false
  }
}
