import { TextField, Button, Typography, Box, Chip } from '@mui/material';
import useMiningStore from '../../store/miningStore';
import { StyledPaper } from '../UI/StyledComponents';
import { useTheme } from '@mui/material/styles';
import gradients from '../../styles/styles/gradients';
import t from '../../locales';
import typography from '../../styles/styles/typography';
import SvgTariSignet from '../../styles/Icons/TariSignet';
import { styled } from '@mui/material/styles';
import useAppStateStore from '../../store/appStore';
import { emit } from '@tauri-apps/api/event';

export const MiningBox = styled(Box)(({ theme }) => ({
  display: 'flex',
  flexDirection: 'column',
  gap: theme.spacing(2),
}));

function MiningWidget() {
  const { isMining } = useMiningStore();
  const { tariAddress, setTariAddress, appState } = useAppStateStore();
  const theme = useTheme();

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
  }

  return (
    <StyledPaper
      style={{
        background: isMining ? gradients.tari : theme.palette.background.paper,
      }}
    >
      <MiningBox>
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
          <Button variant="contained" onClick={() => handleSettingsClose(true)}>
            Save
          </Button>
        </Box>
      </MiningBox>
    </StyledPaper>
  );
}

export default MiningWidget;
