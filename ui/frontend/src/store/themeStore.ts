import { create } from 'zustand';
import { persist } from 'zustand/middleware';

interface Store {
  themeMode: 'light' | 'dark';
  setThemeMode: (mode: 'light' | 'dark') => void;
}

const useThemeStore = create<Store>()(
  persist<Store>(
    (set) => ({
      themeMode: 'light',
      setThemeMode: (mode) => set({ themeMode: mode }),
    }),
    {
      name: 'tari-theme',
    }
  )
);

export default useThemeStore;
