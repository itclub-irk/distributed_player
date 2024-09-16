/// <reference types="vite/client" />
interface ImportMetaEnv {
  readonly VITE_APP_ENV: 'development' | 'production'
  readonly VITE_SERVER_PORT: string
}

interface ImportMeta {
  readonly env: ImportMetaEnv
}
