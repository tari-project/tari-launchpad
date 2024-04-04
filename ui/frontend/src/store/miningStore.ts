import { create } from 'zustand';
import { persist } from 'zustand/middleware';

interface Store {
  isMining: boolean;
  setIsMining: (mode: boolean) => void;
}

const useMiningStore = create<Store>()(
  persist<Store>(
    (set) => ({
      isMining: false,
      setIsMining: (mode) => set({ isMining: mode }),
    }),
    {
      name: 'isMining',
    }
  )
);

export default useMiningStore;
