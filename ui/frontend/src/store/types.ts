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

export type MiningType = 'Sha3' | 'Merge' | 'Monero' | 'All';
