import { useEffect } from 'react';
import { TextField, Button, Typography, Box, Chip } from '@mui/material';
import useMiningStore from '../../../store/miningStore';
import { useTheme } from '@mui/material/styles';
import gradients from '../../../styles/styles/gradients';
import t from '../../../locales';
import typography from '../../../styles/styles/typography';
import SvgTariSignet from '../../../styles/Icons/TariSignet';
import useAppStateStore from '../../../store/appStore';
import GradientBox from '../../../components/GradientBox';
import {
  useStartShaMining,
  useStopShaMining,
} from '../../../api/hooks/useMiningStore';
import { useSetTariAddress } from '../../../api/hooks/useSettingsStore';

function MiningWidget() {
  const theme = useTheme();
  const { isMining, setIsMining } = useMiningStore();
  const { tariAddress, setTariAddress, appState } = useAppStateStore();
  const startMining = useStartShaMining();
  const stopMining = useStopShaMining();
  const setTariAddressMutation = useSetTariAddress();

  function handleTariAddressChange(event: React.ChangeEvent<HTMLInputElement>) {
    setTariAddress(event.target.value);
  }

  useEffect(() => {
    setTariAddress(
      appState?.config?.settings?.saved_settings?.mm_proxy
        .wallet_payment_address ||
        appState?.config?.settings?.saved_settings?.sha3_miner
          .wallet_payment_address ||
        ''
    );
  }, [
    appState?.config?.settings?.saved_settings?.mm_proxy.wallet_payment_address,
    appState?.config?.settings?.saved_settings?.sha3_miner,
  ]);

  async function start() {
    await startMining.mutateAsync({ appState });
    setIsMining(true);
  }

  async function stop() {
    await stopMining.mutateAsync({ appState });
    setIsMining(false);
  }

  function handleSetAddress(save: boolean) {
    if (save) {
      setTariAddressMutation.mutate({ appState, tariAddress });
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
    <GradientBox isActive={isMining} gradient={gradients.tari}>
      <Box
        style={{
          display: 'flex',
          justifyContent: 'space-between',
          gap: theme.spacing(3),
        }}
      >
        <Box
          style={{
            display: 'flex',
            flexDirection: 'column',
            gap: theme.spacing(1),
          }}
        >
          <Box>
            <Chip
              label={
                <span>
                  <strong>{t.common.phrases.startHere}</strong>
                </span>
              }
              color="info"
            />
          </Box>
          <Typography variant="h3" sx={typography.header}>
            {t.common.miningType.tari}
          </Typography>
          <Typography variant="body1" sx={typography.defaultMedium}>
            {t.walletPasswordWizard.description}
          </Typography>
        </Box>
        <Box>
          <SvgTariSignet
            color={theme.palette.divider}
            width="80px"
            height="80px"
          />
        </Box>
      </Box>
      <Box
        style={{
          display: 'flex',
          gap: theme.spacing(1),
        }}
      >
        <TextField
          placeholder="Tari Address"
          value={tariAddress}
          onChange={handleTariAddressChange}
        />
        <Button variant="contained" onClick={() => handleSetAddress(true)}>
          Save
        </Button>
        <Button variant="outlined" onClick={() => handleSetAddress(false)}>
          Cancel
        </Button>
      </Box>
      {isMining ? (
        <Button variant="contained" onClick={stop}>
          Pause
        </Button>
      ) : (
        <Button variant="contained" onClick={start}>
          Start
        </Button>
      )}
    </GradientBox>
  );
}

export default MiningWidget;
