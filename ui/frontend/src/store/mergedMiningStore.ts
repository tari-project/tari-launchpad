import { create } from 'zustand';
import { persist } from 'zustand/middleware';

interface Store {
  isMergedMining: boolean;
  setIsMergedMining: (mode: boolean) => void;
}

const useMergedMiningStore = create<Store>()(
  persist<Store>(
    (set) => ({
      isMergedMining: false,
      setIsMergedMining: (mode) => set({ isMergedMining: mode }),
    }),
    {
      name: 'isMergedMining',
    }
  )
);

export default useMergedMiningStore;
