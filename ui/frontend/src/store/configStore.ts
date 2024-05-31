import { create } from 'zustand';
import { persist } from 'zustand/middleware';

interface StartupConfig {
  baseNode: boolean;
  shaMine: boolean;
  mergeMine: boolean;
}

interface Store {
  startupConfig: StartupConfig;
  setStartupConfig: (key: keyof StartupConfig, value: boolean) => void;
}

const useConfigStore = create<Store>()(
  persist<Store>(
    (set) => ({
      startupConfig: {
        baseNode: false,
        shaMine: false,
        mergeMine: false,
      },
      setStartupConfig: (key, value) =>
        set((state) => ({
          startupConfig: {
            ...state.startupConfig,
            [key]: value,
          },
        })),
    }),
    {
      name: 'config-store',
    }
  )
);

export default useConfigStore;
