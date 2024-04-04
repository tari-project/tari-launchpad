import { create } from 'zustand';
import { persist } from 'zustand/middleware';

interface Message {
  id: string;
  text: string;
}

interface Store {
  messages: Message[];
  pushMessage: (text: string) => void;
}

const useTBotStore = create<Store>()(
  persist<Store>(
    (set) => ({
      messages: [],
      pushMessage: (text) =>
        set((state) => ({
          messages: [
            ...state.messages,
            { id: Math.random().toString(36).substring(7), text },
          ],
        })),
    }),
    {
      name: 'tBotMessages',
    }
  )
);

export default useTBotStore;
