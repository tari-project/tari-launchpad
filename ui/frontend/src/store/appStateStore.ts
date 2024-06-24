import { create } from 'zustand';
import { emit } from '@tauri-apps/api/event';
import { AppStateStore, MiningType, SettingsTabs } from './types';

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
  setAppState: (newState) => set({ appState: newState }),

  containers: {},
  setContainers: (newContainers) =>
    set((state) => ({ ...state, containers: newContainers })),

  isMining: false,
  setIsMining: (value) => set(() => ({ isMining: value })),

  isShaMining: false,
  setIsShaMining: (value) => set(() => ({ isShaMining: value })),

  isMergeMining: false,
  setIsMergeMining: (value) => set(() => ({ isMergeMining: value })),

  isBaseNodeActive: false,
  setIsBaseNodeActive: (value) => set(() => ({ isBaseNodeActive: value })),

  shaMiningEnabled: true,
  setShaMiningEnabled: (value) => set(() => ({ shaMiningEnabled: value })),

  mergeMiningEnabled: true,
  setMergeMiningEnabled: (value) => set(() => ({ mergeMiningEnabled: value })),

  isChangingMining: false,
  setIsChangingMining: (value) => set(() => ({ isChangingMining: value })),

  openDockerWarning: false,
  setOpenDockerWarning: (value) => set(() => ({ openDockerWarning: value })),

  openSettings: false,
  setOpenSettings: (value) => set(() => ({ openSettings: value })),

  tariAddress: '',
  setTariAddress: (value) => set(() => ({ tariAddress: value })),

  moneroAddress: '',
  setMoneroAddress: (value) => set(() => ({ moneroAddress: value })),

  network: '',
  setNetwork: (value) => set(() => ({ network: value })),

  openSchedule: false,
  setOpenSchedule: (value) => set({ openSchedule: value }),

  shaTime: 0,
  setShaTime: (value) => set({ shaTime: value }),

  shaTimerOn: false,
  setShaTimerOn: (value) => set({ shaTimerOn: value }),

  mergeTime: 0,
  setMergeTime: (value) => set({ mergeTime: value }),

  mergeTimerOn: false,
  setMergeTimerOn: (value) => set({ mergeTimerOn: value }),

  settingsTab: 0,

  openSettingsFunc: (tab) => {
    set({ openSettings: true });
    switch (tab) {
      case SettingsTabs.SHA_MINING:
        set({ settingsTab: 0 });
        break;
      case SettingsTabs.MERGED_MINING:
        set({ settingsTab: 1 });
        break;
      case SettingsTabs.BASE_NODE:
        set({ settingsTab: 2 });
        break;
      case SettingsTabs.DOCKER:
        set({ settingsTab: 3 });
        break;
      case SettingsTabs.RESET:
        set({ settingsTab: 4 });
        break;
      default:
        set({ settingsTab: 0 });
    }
  },

  isSubmitting: false,
  setIsSubmitting: (value) => set({ isSubmitting: value }),

  // Mining functions
  startMining: async (miningType: MiningType) => {
    let state = get().appState;
    let stateSession = { ...state?.config?.session };
    switch (miningType) {
      case 'Sha':
        stateSession.sha3x_layer_active = true;
        // set({ isShaMining: true });
        break;
      case 'Merge':
        stateSession.merge_layer_active = true;
        // set({ isMergeMining: true });
        break;
      case 'All':
        stateSession.sha3x_layer_active = true;
        stateSession.merge_layer_active = true;
        // set({ isShaMining: true });
        // set({ isMergeMining: true });
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
      case 'Sha':
        stateSession.sha3x_layer_active = false;
        // set({ isShaMining: false });
        break;
      case 'Merge':
        stateSession.merge_layer_active = false;
        // set({ isMergeMining: false });
        break;
      case 'All':
        stateSession.sha3x_layer_active = false;
        stateSession.merge_layer_active = false;
        // set({ isShaMining: false });
        // set({ isMergeMining: false });
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
    emit('tari://actions', {
      Action: { type: 'ChangeSession', payload: stateSession },
    });
  },
  stopBaseNode: async () => {
    let state = get().appState;
    let stateSession = { ...state?.config?.session };
    stateSession.base_node_active = false;
    emit('tari://actions', {
      Action: { type: 'ChangeSession', payload: stateSession },
    });
  },

  // Address related functions
  saveTariAddress: async (tariAddress: string) => {
    let state = get().appState;
    let settings = { ...state?.config?.settings?.saved_settings };
    settings.mm_proxy.wallet_payment_address = tariAddress;
    set({ tariAddress: tariAddress });
    settings.sha3_miner.wallet_payment_address = tariAddress;
    emit('tari://actions', {
      Action: { type: 'SaveSettings', payload: settings },
    });
  },
  saveMoneroAddress: async (moneroAddress: string) => {
    let state = get().appState;
    let settings = { ...state?.config?.settings?.saved_settings };
    settings.xmrig.monero_mining_address = moneroAddress;
    set({ moneroAddress: moneroAddress });
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
}));

export default useAppStateStore;
