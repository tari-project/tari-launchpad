export interface ConfigState {
  session: {
    all_active: boolean;
    base_layer_active: boolean;
    base_node_active: boolean;
    grafana_active: boolean;
    loki_active: boolean;
    merge_layer_active: boolean;
    mm_proxy_active: boolean;
    sha3x_layer_active: boolean;
    xmrig_active: boolean;
  };
  settings: {
    data_directory: string;
    saved_settings: {
      mm_proxy: {
        wallet_payment_address: string;
        monerod_url: string;
      };
      sha3_miner: {
        num_mining_threads: number;
        wallet_payment_address: string;
      };
      xmrig: {
        num_mining_threads: number;
        monero_mining_address: string;
      };
      registry: string;
      tag: string;
      tari_network: string;
    };
    tor_control_password: string;
    with_monitoring: boolean;
    with_tor: boolean;
  };
}

export interface ContainerState {
  [key: string]: any;
}

export interface ErrorState {
  data: any[];
  length: number;
}

export interface NodeState {
  chain_height: number;
  identity: any;
  peer_count: number;
  sync_status: string;
}

export interface AppState {
  config: ConfigState;
  containers: ContainerState;
  errors: ErrorState;
  node: NodeState;
}

export type MiningType = 'Sha' | 'Merge' | 'Monero' | 'All';

export const BaseNodeStatus = {
  WAITING: 'Waiting',
  SHUTTINGDOWN: 'ShuttingDown',
  ACTIVE: 'Active',
  STARTING: 'Starting...',
  INACTIVE: 'Inactive',
  PENDING: 'Pending',
  CHECKING: 'Checking for old containers...',
  SYNCING: 'Syncing blockchain...',
};

export const ShaMiningStatus = {
  WAITING: 'Waiting',
  SHUTTINGDOWN: 'ShuttingDown',
  ACTIVE: 'Active',
  STARTING: 'Starting...',
  INACTIVE: 'Inactive',
  PENDING: 'Pending',
};

export const MergeMiningStatus = {
  WAITING: 'Waiting',
  SHUTTINGDOWN: 'ShuttingDown',
  ACTIVE: 'Active',
  STARTING: 'Starting...',
  INACTIVE: 'Inactive',
  PENDING: 'Pending',
};

export const SettingsTabs = {
  SHA_MINING: 'Sha Mining',
  MERGED_MINING: 'Merged Mining',
  BASE_NODE: 'BaseNode',
  DOCKER: 'Docker',
  GENERAL: 'General',
  RESET: 'Reset',
};

export interface AppStateStore {
  // AppState
  appState: AppState;
  setAppState: (newState: AppState) => void;

  // ContainerState
  containers: ContainerState;
  setContainers: (newContainers: ContainerState) => void;

  // Mining states
  isMining: boolean;
  setIsMining: (value: boolean) => void;

  isShaMining: boolean;
  setIsShaMining: (value: boolean) => void;

  isMergeMining: boolean;
  setIsMergeMining: (value: boolean) => void;

  isBaseNodeActive: boolean;
  setIsBaseNodeActive: (value: boolean) => void;

  shaMiningEnabled: boolean;
  setShaMiningEnabled: (value: boolean) => void;

  mergeMiningEnabled: boolean;
  setMergeMiningEnabled: (value: boolean) => void;

  isChangingMining: boolean;
  setIsChangingMining: (value: boolean) => void;

  // UI states
  openDockerWarning: boolean;
  setOpenDockerWarning: (value: boolean) => void;

  openSettings: boolean;
  setOpenSettings: (value: boolean) => void;

  openSchedule: boolean;
  setOpenSchedule: (value: boolean) => void;

  // Addresses
  tariAddress: string;
  setTariAddress: (value: string) => void;

  moneroAddress: string;
  setMoneroAddress: (value: string) => void;

  // Network
  network: string;
  setNetwork: (value: string) => void;

  // Timer states
  shaTime: number;
  setShaTime: (value: number) => void;

  shaTimerOn: boolean;
  setShaTimerOn: (value: boolean) => void;

  mergeTime: number;
  setMergeTime: (value: number) => void;

  mergeTimerOn: boolean;
  setMergeTimerOn: (value: boolean) => void;

  // Form submission state
  isSubmitting: boolean;
  setIsSubmitting: (value: boolean) => void;

  // Settings tab
  settingsTab: number;

  // Functions
  openSettingsFunc: (settingsTab: string) => void;
  startMining: (type: MiningType) => void;
  stopMining: (type: MiningType) => void;
  startBaseNode: () => void;
  stopBaseNode: () => void;
  saveTariAddress: (tariAddress: string) => void;
  saveMoneroAddress: (moneroAddress: string) => void;
  saveSettings: (formData: any) => void;
}
