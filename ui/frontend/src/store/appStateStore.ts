import { create } from 'zustand';
import { emit } from '@tauri-apps/api/event';
import { AppState, ContainerState, MiningType } from './types';

interface AppStateStore {
  appState: AppState;
  containers: ContainerState;
  isMining: boolean;
  isShaMining: boolean;
  isMergeMining: boolean;
  isBaseNodeActive: boolean;
  shaMiningEnabled: boolean;
  mergeMiningEnabled: boolean;
  isChangingMining: boolean;
  openDockerWarning: boolean;
  openSettings: boolean;
  tariAddress: string;
  moneroAddress: string;
  setAppState: (newState: AppState) => void;
  setContainers: (newContainers: ContainerState) => void;
  setIsMining: (value: boolean) => void;
  setIsShaMining: (value: boolean) => void;
  setIsMergeMining: (value: boolean) => void;
  setIsBaseNodeActive: (value: boolean) => void;
  setShaMiningEnabled: (value: boolean) => void;
  setMergeMiningEnabled: (value: boolean) => void;
  setIsChangingMining: (value: boolean) => void;
  setOpenDockerWarning: (value: boolean) => void;
  setOpenSettings: (value: boolean) => void;
  setTariAddress: (value: string) => void;
  setMoneroAddress: (value: string) => void;
  startMining: (type: MiningType) => void;
  stopMining: (type: MiningType) => void;
  startBaseNode: () => void;
  stopBaseNode: () => void;
  //settings
  isSubmitting: boolean;
  setIsSubmitting: (value: boolean) => void;
  saveTariAddress: (tariAddress: string) => void;
  saveMoneroAddress: (moneroAddress: string) => void;
  saveSettings: (formData: any) => void;
  runOnStartup: boolean;
  mineOnStartup: boolean;
  setRunOnStartup: (value: boolean) => void;
  setMineOnStartup: (value: boolean) => void;
}

const useAppStateStore = create<AppStateStore>((set, get) => ({
  appState: {
    config: {
      session: {
        all_active: false,
        base_layer_active: false,
        base_node_active: false,
        grafana_active: false,
        loki_active: false,
        merge_layer_active: false,
        mm_proxy_active: false,
        sha3x_layer_active: false,
        xmrig_active: false,
      },
      settings: {
        data_directory: '',
        saved_settings: {
          mm_proxy: {
            wallet_payment_address: '',
            monerod_url: '',
          },
          sha3_miner: {
            num_mining_threads: 0,
            wallet_payment_address: '',
          },
          xmrig: {
            num_mining_threads: 0,
            monero_mining_address: '',
          },
          registry: '',
          tag: '',
          tari_network: '',
        },
        tor_control_password: '',
        with_monitoring: false,
        with_tor: false,
      },
    },
    containers: {},
    errors: {
      data: [],
      length: 0,
    },
    node: {
      chain_height: 0,
      identity: {},
      peer_count: 0,
      sync_status: '',
    },
  },
  containers: {},
  isMining: false,
  isShaMining: false,
  isMergeMining: false,
  isBaseNodeActive: false,
  shaMiningEnabled: true,
  mergeMiningEnabled: true,
  isChangingMining: false,
  openDockerWarning: false,
  openSettings: false,
  tariAddress: '',
  moneroAddress: '',
  setAppState: (newState) => set({ appState: newState }),
  setContainers: (newContainers) =>
    set((state) => ({ ...state, containers: newContainers })),
  setIsMining: (value) => set(() => ({ isMining: value })),
  setIsShaMining: (value) => set(() => ({ isShaMining: value })),
  setIsMergeMining: (value) => set(() => ({ isMergeMining: value })),
  setIsBaseNodeActive: (value) => set(() => ({ isBaseNodeActive: value })),
  setShaMiningEnabled: (value) => set(() => ({ shaMiningEnabled: value })),
  setMergeMiningEnabled: (value) => set(() => ({ mergeMiningEnabled: value })),
  setIsChangingMining: (value) => set(() => ({ isChangingMining: value })),
  setOpenDockerWarning: (value) => set(() => ({ openDockerWarning: value })),
  setOpenSettings: (value) => set(() => ({ openSettings: value })),
  setTariAddress: (value) => set(() => ({ tariAddress: value })),
  setMoneroAddress: (value) => set(() => ({ moneroAddress: value })),
  startMining: async (miningType: MiningType) => {
    let state = get().appState;
    let stateSession = { ...state?.config?.session };
    switch (miningType) {
      case 'Sha3':
        stateSession.sha3x_layer_active = true;
        set({ isShaMining: true });
        break;
      case 'Merge':
        stateSession.merge_layer_active = true;
        set({ isMergeMining: true });
        break;
      case 'All':
        stateSession.sha3x_layer_active = true;
        stateSession.merge_layer_active = true;
        set({ isShaMining: true });
        set({ isMergeMining: true });
        break;
    }
    emit('tari://actions', {
      Action: { type: 'ChangeSession', payload: stateSession },
    });
  },
  stopMining: async (miningType: MiningType) => {
    let state = get().appState;
    let stateSession = { ...state?.config?.session };
    switch (miningType) {
      case 'Sha3':
        stateSession.sha3x_layer_active = false;
        set({ isShaMining: false });
        break;
      case 'Merge':
        stateSession.merge_layer_active = false;
        set({ isMergeMining: false });
        break;
      case 'All':
        stateSession.sha3x_layer_active = false;
        stateSession.merge_layer_active = false;
        set({ isShaMining: false });
        set({ isMergeMining: false });
        break;
    }
    emit('tari://actions', {
      Action: { type: 'ChangeSession', payload: stateSession },
    });
  },
  startBaseNode: async () => {
    let state = get().appState;
    let stateSession = { ...state?.config?.session };
    stateSession.base_node_active = true;
    // set({ isBaseNodeActive: true });
    emit('tari://actions', {
      Action: { type: 'ChangeSession', payload: stateSession },
    });
  },
  stopBaseNode: async () => {
    let state = get().appState;
    let stateSession = { ...state?.config?.session };
    stateSession.base_node_active = false;
    // set({ isBaseNodeActive: false });
    emit('tari://actions', {
      Action: { type: 'ChangeSession', payload: stateSession },
    });
  },
  isSubmitting: false,
  setIsSubmitting: (value) => set({ isSubmitting: value }),
  saveTariAddress: async (tariAddress: string) => {
    let state = get().appState;
    let settings = { ...state?.config?.settings?.saved_settings };
    settings.mm_proxy.wallet_payment_address = tariAddress;
    settings.sha3_miner.wallet_payment_address = tariAddress;
    emit('tari://actions', {
      Action: { type: 'SaveSettings', payload: settings },
    });
  },
  saveMoneroAddress: async (moneroAddress: string) => {
    let state = get().appState;
    let settings = { ...state?.config?.settings?.saved_settings };
    settings.xmrig.monero_mining_address = moneroAddress;
    emit('tari://actions', {
      Action: { type: 'SaveSettings', payload: settings },
    });
  },
  saveSettings: async (formData: any) => {
    let state = get().appState;
    let settings = { ...state?.config?.settings?.saved_settings };
    settings.mm_proxy.wallet_payment_address =
      formData.walletSettings.tariAddress;
    emit('tari://actions', {
      Action: { type: 'SaveSettings', payload: settings },
    });
  },
  runOnStartup: false,
  mineOnStartup: false,
  setRunOnStartup: (value) => set({ runOnStartup: value }),
  setMineOnStartup: (value) => set({ mineOnStartup: value }),
}));

export default useAppStateStore;
