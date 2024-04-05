import {
  Dialog,
  DialogActions,
  DialogContent,
  DialogContentText,
  TextField,
  Button,
  Box,
} from '@mui/material';
import SettingsTabs from './SettingsTabs';
import { emit } from '@tauri-apps/api/event';
import useAppStateStore from '../../store/appStore';

function SettingsDialog() {
  const {
    openSettings,
    tariAddress,
    setTariAddress,
    appState,
    setOpenSettings,
  } = useAppStateStore();

  function handleTariAddressChange(event: React.ChangeEvent<HTMLInputElement>) {
    setTariAddress(event.target.value);
  }

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
      <Box
        style={{
          display: 'flex',
          justifyContent: 'center',
          alignItems: 'center',
          padding: '24px',
        }}
      >
        <DialogContent>
          <DialogContentText
            id="alert-dialog-description"
            style={{
              padding: '8px',
            }}
          >
            <TextField
              label="Tari Address"
              style={{
                width: 300,
              }}
              value={tariAddress}
              onChange={handleTariAddressChange}
            />
          </DialogContentText>
        </DialogContent>
        <DialogActions>
          <Button onClick={() => handleSettingsClose(true)}>Save</Button>
          <Button onClick={() => handleSettingsClose(false)}>Exit</Button>
        </DialogActions>
      </Box>
    </Dialog>
  );
}

export default SettingsDialog;
