import { Typography, TextField, Button } from '@mui/material';
import typography from '../../styles/styles/typography';
import {
  SettingsBox,
  LabelBoxVertical,
  HorisontalButtons,
} from '../UI/StyledComponents';
import t from '../../locales';
import useAppStateStore from '../../store/appStore';
import { emit } from '@tauri-apps/api/event';

function WalletSettings() {
  const { tariAddress, setTariAddress, appState } = useAppStateStore();
  function handleTariAddressChange(event: React.ChangeEvent<HTMLInputElement>) {
    setTariAddress(event.target.value);
  }

  function handleAddressChange(save: boolean) {
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
  }

  return (
    <>
      <Typography variant="h3" style={typography.subheader}>
        {t.wallet.settings.title}
      </Typography>
      <SettingsBox>
        <LabelBoxVertical>
          <Typography variant="body1" style={typography.smallMedium}>
            {t.wallet.wallet.walletId} ({t.wallet.wallet.address})
          </Typography>
          <TextField
            placeholder={t.wallet.wallet.walletId}
            value={tariAddress}
            onChange={handleTariAddressChange}
          />
        </LabelBoxVertical>
        <Typography variant="body2" style={typography.smallMedium}>
          {t.wallet.settings.explanations.storage}{' '}
          {t.wallet.settings.explanations.send} (
          {t.wallet.settings.explanations.try}{' '}
          <span style={typography.smallUnder}>
            {t.wallet.settings.explanations.aurora}
          </span>{' '}
          {t.wallet.settings.explanations.itsGreat}){' '}
          {t.wallet.settings.explanations.extendedFunctionality}{' '}
          {t.wallet.settings.explanations.convert}{' '}
        </Typography>
        <HorisontalButtons>
          <Button variant="outlined" onClick={() => handleAddressChange(false)}>
            Cancel
          </Button>
          <Button variant="outlined" onClick={() => handleAddressChange(true)}>
            Save
          </Button>
        </HorisontalButtons>
      </SettingsBox>
    </>
  );
}

export default WalletSettings;
