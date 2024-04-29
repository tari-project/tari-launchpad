import { useMutation } from '@tanstack/react-query';
import { emit } from '@tauri-apps/api/event';

export const useSetTariAddress = () => {
  return useMutation({
    mutationFn: async ({
      appState,
      tariAddress,
    }: {
      appState: any;
      tariAddress: string;
    }) => {
      let state = appState;
      let settings = { ...state?.config?.settings?.saved_settings };

      settings.mm_proxy.wallet_payment_address = tariAddress;
      settings.sha3_miner.wallet_payment_address = tariAddress;
      emit('tari://actions', {
        Action: { type: 'SaveSettings', payload: settings },
      });
    },
    onSuccess: () => {
      console.log('Tari address updated');
    },
  });
};

export const useSetMoneroAddress = () => {
  return useMutation({
    mutationFn: async ({
      appState,
      moneroAddress,
    }: {
      appState: any;
      moneroAddress: string;
    }) => {
      let state = appState;
      let settings = { ...state?.config?.settings?.saved_settings };

      settings.xmrig.monero_mining_address = moneroAddress;
      emit('tari://actions', {
        Action: { type: 'SaveSettings', payload: settings },
      });
    },
    onSuccess: () => {
      console.log('Monero address updated');
    },
  });
};
