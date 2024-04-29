import { useMutation } from '@tanstack/react-query';
import { emit } from '@tauri-apps/api/event';

export const useStartMining = () => {
  return useMutation({
    mutationFn: async ({
      appState,
      mergeMiningEnabled,
      shaMiningEnabled,
    }: {
      appState: any;
      mergeMiningEnabled: boolean;
      shaMiningEnabled: boolean;
    }) => {
      let state = appState;
      let stateSession = { ...state?.config?.session };
      stateSession.merge_layer_active = mergeMiningEnabled;
      stateSession.sha3x_layer_active = shaMiningEnabled;
      emit('tari://actions', {
        Action: { type: 'ChangeSession', payload: stateSession },
      });
    },
    onSuccess: () => {
      console.log('Mining started');
    },
  });
};

export const useStartShaMining = () => {
  return useMutation({
    mutationFn: async ({ appState }: any) => {
      let state = appState;
      let stateSession = { ...state?.config?.session };
      stateSession.sha3x_layer_active = true;
      emit('tari://actions', {
        Action: { type: 'ChangeSession', payload: stateSession },
      });
    },
    onSuccess: () => {
      console.log('Sha Mining started');
    },
  });
};

export const useStartMergeMining = () => {
  return useMutation({
    mutationFn: async ({ appState }: any) => {
      let state = appState;
      let stateSession = { ...state?.config?.session };
      stateSession.merge_layer_active = true;
      emit('tari://actions', {
        Action: { type: 'ChangeSession', payload: stateSession },
      });
    },
    onSuccess: () => {
      console.log('Merge Mining started');
    },
  });
};

export const useStopMining = () => {
  return useMutation({
    mutationFn: async ({ appState }: any) => {
      let state = appState;
      let stateSession = { ...state?.config?.session };
      stateSession.merge_layer_active = false;
      stateSession.sha3x_layer_active = false;
      emit('tari://actions', {
        Action: { type: 'ChangeSession', payload: stateSession },
      });
    },
    onSuccess: () => {
      console.log('Mining stopped');
    },
  });
};

export const useStopShaMining = () => {
  return useMutation({
    mutationFn: async ({ appState }: any) => {
      let state = appState;
      let stateSession = { ...state?.config?.session };
      stateSession.sha3x_layer_active = false;
      emit('tari://actions', {
        Action: { type: 'ChangeSession', payload: stateSession },
      });
    },
    onSuccess: () => {
      console.log('Sha Mining stopped');
    },
  });
};

export const useStopMergeMining = () => {
  return useMutation({
    mutationFn: async ({ appState }: any) => {
      let state = appState;
      let stateSession = { ...state?.config?.session };
      stateSession.merge_layer_active = false;
      emit('tari://actions', {
        Action: { type: 'ChangeSession', payload: stateSession },
      });
    },
    onSuccess: () => {
      console.log('Merge Mining stopped');
    },
  });
};
