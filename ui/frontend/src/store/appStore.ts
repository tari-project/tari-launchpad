import { create } from 'zustand';

interface AppState {
  [key: string]: any;
}

interface ContainerState {
  [key: string]: any;
}

interface AppStateStore {
  appState: AppState;
  containers: ContainerState;
  isMining: boolean;
  shaMiningEnabled: boolean;
  mergeMiningEnabled: boolean;
  isChangingMining: boolean;
  openDockerWarning: boolean;
  openSettings: boolean;
  tariAddress: string;
  setAppState: (newState: AppState) => void;
  setContainers: (newContainers: ContainerState) => void;
  setIsMining: (value: boolean) => void;
  setShaMiningEnabled: (value: boolean) => void;
  setMergeMiningEnabled: (value: boolean) => void;
  setIsChangingMining: (value: boolean) => void;
  setOpenDockerWarning: (value: boolean) => void;
  setOpenSettings: (value: boolean) => void;
  setTariAddress: (value: string) => void;
}

const useAppStateStore = create<AppStateStore>((set) => ({
  appState: {},
  containers: {},
  isMining: false,
  shaMiningEnabled: true,
  mergeMiningEnabled: true,
  isChangingMining: false,
  openDockerWarning: false,
  openSettings: false,
  tariAddress: '',
  setAppState: (newState) => set({ appState: newState }),
  setContainers: (newContainers) => set({ containers: newContainers }),
  setIsMining: (value) => set((state) => ({ ...state, isMining: value })),
  setShaMiningEnabled: (value) =>
    set((state) => ({ ...state, shaMiningEnabled: value })),
  setMergeMiningEnabled: (value) =>
    set((state) => ({ ...state, mergeMiningEnabled: value })),
  setIsChangingMining: (value) =>
    set((state) => ({ ...state, isChangingMining: value })),
  setOpenDockerWarning: (value) =>
    set((state) => ({ ...state, openDockerWarning: value })),
  setOpenSettings: (value) =>
    set((state) => ({ ...state, openSettings: value })),
  setTariAddress: (value) => set((state) => ({ ...state, tariAddress: value })),
}));

export default useAppStateStore;
