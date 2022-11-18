//todo move this file

import { WalletBalance } from '../wallet/walletService'
import { ContainerStats } from '../containers/types'

export interface LaunchpadState {
  config: LaunchpadConfig
  containers: ContainerState
  wallet: WalletState
}

export interface FeState {
  config: LaunchpadConfig
  containers: ContainerTaskState[]
  wallet: WalletState
}

export enum TaskStatus {
  Inactive = 'Inactive',
  Pending = 'Pending',
  Progress = 'Progress',
  Active = 'Active',
  StatsRecord = 'StatsRecord',
}
export type TaskProgress = {
  pct: number
  stage: string
}

export type TaskId = string

export enum Containers {
  Wallet = 'Wallet',
  Sha3Miner = 'Sha3Miner',
  Tor = 'Tor',
  LocalNet = 'LocalNet',
  SharedVolume = 'SharedVolume',
  SharedGrafanaVolume = 'SharedGrafanaVolume',
  Grafana = 'Grafana',
  BaseNode = 'Base Node',
  Promtail = 'Promtail',
  Loki = 'Loki',
}

interface WalletState {
  active: boolean
  balance?: WalletBalance
  transactions: Array<WalletTransaction>
}

interface WalletTransaction {
  event: string
  tx_id: string
  source_pk: Array<number>
  dest_pk: Array<number>
  status: string
  direction: string
  amount: number
  message: string
  is_coinbase: boolean
}

interface LaunchpadConfig {
  session: Partial<LaunchpadSession>
  settings?: Partial<Omit<LaunchPadSettings, 'session'>>
}

interface LaunchPadSettings {
  session: Partial<LaunchpadSession>
  data_directory: string
  tari_network: TariNetwork
  tor_control_password: string
  base_node?: Record<string, string>
  wallet?: { password: string }
  sha3_miner?: { num_mining_threads: number }
  mm_proxy?: MmProxyConfig
  xmrig?: { monero_mining_address: string }
  registry?: string
  tag?: string
  with_monitoring: boolean
  with_tor: boolean
}

export enum TariNetwork {
  Dibbler = 'Dibbler',
  Esmeralda = 'Esmeralda',
  Igor = 'Igor',
  Mainnet = 'Mainnet',
}

interface MmProxyConfig {
  monerod_url: string
  monero_username: string
  monero_password: string
  monero_use_auth: boolean
}

export interface StatsData {
  timestamp: string
  cpu_usage: number
  mem_limit: number
  mem_usage: number
}
export type Task = TaskStatus | { [TaskStatus.Progress]: TaskProgress }
// | { [TaskStatus.StatsRecord]: Array<StatsData> }

export interface Delta {
  UpdateStatus: Task
  StatsRecord: Array<StatsData>
  LogRecord?: string
}
export interface TaskDelta {
  id?: Containers
  delta?: Delta
}

export interface TaskState {
  status: Task
  permanent: boolean
  stats: Array<StatsData>
  tail?: Array<string>
  exitCode?: number
  error?: string
}

export type ContainerState = {
  [Key in Containers as string]?: TaskState
}
export interface ContainerTaskState {
  id: Containers
  task_state?: TaskState
  timestamp: number
}

export interface LaunchpadDelta {
  UpdateConfig?: LaunchpadConfig
  TaskDelta?: TaskDelta
  WalletDelta?: WalletState
}

export interface LaunchpadSession {
  all_active: boolean
  base_layer_active: boolean
  merge_layer_active: boolean
  monitoring_layer_active: boolean
  tor_active: boolean
  base_node_active: boolean
  wallet_active: boolean
  miner_active: boolean
  grafana_active: boolean
  loki_active: boolean
  promtail_active: boolean
  //TODO: to ask about this for ChangeSession
  // shared_grafana_volume_active: boolean
  // local_net_active: boolean
  // shared_volume_active: boolean
}

export interface LaunchPad {
  State: LaunchpadState
  Delta: LaunchpadDelta
  taskDeltas: TaskDelta[]
  launchpadState: FeState
}
