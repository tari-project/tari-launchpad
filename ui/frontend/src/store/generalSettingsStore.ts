import { create } from 'zustand';

interface GeneralSettingsStore {
  runOnStartup: boolean;
  mineOnStartup: boolean;
  setRunOnStartup: (value: boolean) => void;
  setMineOnStartup: (value: boolean) => void;
}

const useGeneralSettingsStore = create<GeneralSettingsStore>((set) => ({
  runOnStartup: false,
  mineOnStartup: false,
  setRunOnStartup: (value) => set({ runOnStartup: value }),
  setMineOnStartup: (value) => set({ mineOnStartup: value }),
}));

export default useGeneralSettingsStore;
