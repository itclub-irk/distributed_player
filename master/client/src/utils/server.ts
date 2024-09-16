/**
 * Returns base url for server API. If env var VITE_APP_ENV is in "development" mode
 * server port will taken from VITE_SERVER_PORT env var, otherwise server port will the same
 * as client origin
 * @returns base server url
 */
export function getServerBaseUrl(): string {
  const appEnv = import.meta.env.VITE_APP_ENV

  const origin = `${window.location.protocol}//${window.location.hostname}`
  let serverPort = window.location.port

  if (appEnv === 'development') {
    serverPort = import.meta.env.VITE_SERVER_PORT
  }
  return `${origin}:${serverPort}`
}
