import { Dialog, DialogActions, Button, Divider } from '@mui/material';
import SettingsTabs from './SettingsTabs';
import { emit } from '@tauri-apps/api/event';
import useAppStateStore from '../../store/appStore';
import t from '../../locales';

function SettingsDialog() {
  const {
    openSettings,
    tariAddress,
    setTariAddress,
    appState,
    setOpenSettings,
  } = useAppStateStore();

  function handleSettingsClose(save: boolean) {
    if (save) {
      let state: any = appState;
      console.log(state);
      let settings = { ...state?.config?.settings?.saved_settings };

      settings.mm_proxy.wallet_payment_address = tariAddress;
      settings.sha3_miner.wallet_payment_address = tariAddress;
      emit('tari://actions', {
        Action: { type: 'SaveSettings', payload: settings },
      });
    } else {
      setTariAddress(
        appState?.config?.settings?.saved_settings?.mm_proxy
          .wallet_payment_address ||
          appState?.config?.settings?.saved_settings?.sha3_miner
            ?.wallet_payment_address ||
          ''
      );
    }
    setOpenSettings(false);
  }

  const handleCancel = () => {
    setOpenSettings(false);
  };

  const handleSave = () => {
    console.log('Save settings');
    setOpenSettings(false);
  };

  return (
    <Dialog
      open={openSettings}
      onClose={() => handleSettingsClose(false)}
      aria-labelledby="alert-dialog-title"
      aria-describedby="alert-dialog-description"
      fullWidth
      maxWidth="md"
    >
      <SettingsTabs />
      <Divider variant="fullWidth" />
      <DialogActions>
        <Button variant="outlined" onClick={() => handleCancel()}>
          {t.common.verbs.cancel}
        </Button>
        <Button variant="contained" onClick={() => handleSave()}>
          {t.common.phrases.saveChanges}
        </Button>
      </DialogActions>
    </Dialog>
  );
}

export default SettingsDialog;
