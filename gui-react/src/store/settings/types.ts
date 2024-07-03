export enum Settings {
  Mining = 'mining',
  BaseNode = 'baseNode',
  Wallet = 'wallet',
  // Logs = 'logs',
  Security = 'security',
  Reset = 'reset',
}

export interface InitialSettings {
  moneroMiningAddress: string
  moneroWalletAddress: string
  numMiningThreads: number
  tariNetwork: string
  cacheDir: string
  dockerRegistry: string
  parole: string
  dockerTag: string
  monerodUrl: string
  moneroUseAuth: boolean
  moneroUsername: string
  moneroPassword: string
  rootFolder: string
}

export type ServiceSettingsState = {
  parole?: string
  dockerRegistry: string
  dockerTag: string
  // eslint-disable-next-line @typescript-eslint/no-explicit-any
} & any

export type SettingsState = {
  open: boolean
  which: Settings
  serviceSettings: ServiceSettingsState
}
