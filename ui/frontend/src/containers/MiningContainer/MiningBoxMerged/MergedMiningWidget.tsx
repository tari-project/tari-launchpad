import { useEffect } from 'react';
import { Typography, Box, Chip, Button, TextField } from '@mui/material';
import useMergedMiningStore from '../../../store/mergedMiningStore';
import { useTheme } from '@mui/material/styles';
import gradients from '../../../styles/styles/gradients';
import t from '../../../locales';
import typography from '../../../styles/styles/typography';
import SvgTariSignet from '../../../styles/Icons/TariSignet';
import SvgMoneroSignet from '../../../styles/Icons/MoneroSignet';
import SvgQuestion from '../../../styles/Icons/Question';
import { StyledIconButton } from '../../../components/StyledComponents';
import { useSnackbar } from 'notistack';
import GradientBox from '../../../components/GradientBox';
import {
  useStartMergeMining,
  useStopMergeMining,
} from '../../../api/hooks/useMiningStore';
import useAppStateStore from '../../../store/appStore';
import { useSetMoneroAddress } from '../../../api/hooks/useSettingsStore';

function MergedMiningWidget() {
  const theme = useTheme();
  const { isMergedMining, setIsMergedMining } = useMergedMiningStore();
  const { appState, moneroAddress, setMoneroAddress } = useAppStateStore();
  const { enqueueSnackbar } = useSnackbar();
  const startMining = useStartMergeMining();
  const stopMining = useStopMergeMining();
  const setMoneroAddressMutation = useSetMoneroAddress();

  function handleMoneroAddressChange(
    event: React.ChangeEvent<HTMLInputElement>
  ) {
    setMoneroAddress(event.target.value);
  }

  function handleSetAddress(save: boolean) {
    if (save) {
      setMoneroAddressMutation.mutate({ appState, moneroAddress });
    } else {
      setMoneroAddress(
        appState?.config?.settings?.saved_settings?.xmrig
          .monero_mining_address || ''
      );
    }
  }

  const mergedMiningHelp = () => {
    enqueueSnackbar(`${t.mergedMiningHelp.message1}`, {
      key: 'mergedMiningHelp.message1',
      persist: true,
    });
  };

  useEffect(() => {
    setMoneroAddress(
      appState?.config?.settings?.saved_settings?.xmrig
        ?.monero_mining_address || ''
    );
  }, [
    appState?.config?.settings?.saved_settings?.xmrig?.monero_mining_address,
  ]);

  async function start() {
    await startMining.mutateAsync({ appState });
    setIsMergedMining(true);
  }

  async function stop() {
    await stopMining.mutateAsync({ appState });
    setIsMergedMining(false);
  }

  return (
    <GradientBox isActive={isMergedMining} gradient={gradients.merged}>
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
                  <strong>{t.common.phrases.readyToSet}</strong>
                </span>
              }
              color="info"
            />
          </Box>
          <Box
            style={{
              display: 'flex',
              gap: theme.spacing(1),
              alignItems: 'center',
            }}
          >
            <Typography variant="h3" sx={typography.header}>
              {t.common.miningType.merged}
            </Typography>
            <StyledIconButton onClick={() => mergedMiningHelp()}>
              <SvgQuestion color={theme.palette.primary.main} />
            </StyledIconButton>
          </Box>
          <Typography variant="body2" sx={typography.defaultMedium}>
            {t.mining.setup.description}{' '}
            <span style={typography.defaultHeavy}>
              {t.mining.setup.descriptionBold}
            </span>
          </Typography>
        </Box>
        <Box
          style={{
            display: 'flex',
            flexDirection: 'column',
            gap: theme.spacing(2),
          }}
        >
          <SvgMoneroSignet
            color={theme.palette.divider}
            width="80px"
            height="80px"
          />
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
          value={moneroAddress}
          onChange={handleMoneroAddressChange}
        />
        <Button variant="contained" onClick={() => handleSetAddress(true)}>
          Save
        </Button>
        <Button variant="outlined" onClick={() => handleSetAddress(false)}>
          Cancel
        </Button>
      </Box>
      {isMergedMining ? (
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

export default MergedMiningWidget;
