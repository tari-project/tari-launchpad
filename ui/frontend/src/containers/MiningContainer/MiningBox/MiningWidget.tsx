import { useEffect, useState } from 'react';
import {
  Button,
  Chip,
  Typography,
  TextField,
  Box,
  CircularProgress,
} from '@mui/material';
import t from '../../../locales';
import typography from '../../../styles/styles/typography';
import useAppStateStore from '../../../store/appStateStore';
import { ShaMiningStatus } from '../../../store/types';
import {
  StatusChip,
  MiningBoxOuter,
  MiningBoxInner,
  MiningButtonBox,
  TransparentButton,
  ShaMiningBox,
} from '../../../components/StyledComponents';
import { useTheme } from '@mui/material/styles';
import SvgTariSignet from '../../../styles/Icons/TariSignet';
import CopyToClipboard from '../../../components/CopyToClipboard';

function MiningWidget() {
  const {
    appState,
    containers,
    tariAddress,
    setTariAddress,
    saveTariAddress,
    startMining,
    stopMining,
  } = useAppStateStore();
  const theme = useTheme();

  // function handleTariAddressChange(event: React.ChangeEvent<HTMLInputElement>) {
  //   setTariAddress(event.target.value);
  // }

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
  function start() {
    startMining('Sha3');
  }

  function stop() {
    stopMining('Sha3');
  }

  const SignetBox = () => {
    return (
      <Box>
        <SvgTariSignet
          color={theme.palette.divider}
          width="80px"
          height="80px"
        />
      </Box>
    );
  };

  const MiningTitle = () => {
    return (
      <Typography variant="h3" sx={typography.header}>
        {t.common.miningType.tari}
      </Typography>
    );
  };

  const MiningButton = () => {
    return (
      <MiningButtonBox>
        <Typography variant="body2" sx={typography.smallHeavy} pr={1}>
          0:00:00
        </Typography>
        <Typography variant="body2" sx={typography.smallHeavy}>
          |
        </Typography>
        <Button
          variant="text"
          onClick={stop}
          style={{
            color: '#fff',
          }}
        >
          {t.common.verbs.pause}
        </Button>
      </MiningButtonBox>
    );
  };

  const TariAddressTextField = () => {
    const [localAddress, setLocalAddress] = useState(tariAddress);

    const handleLocalAddressChange = (
      event: React.ChangeEvent<HTMLInputElement>
    ) => {
      setLocalAddress(event.target.value);
    };

    function handleSetAddress(save: boolean) {
      if (save) {
        saveTariAddress(localAddress);
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
      <Box
        style={{
          display: 'flex',
          gap: theme.spacing(1),
        }}
      >
        <TextField
          placeholder="Tari Address"
          value={localAddress}
          onChange={handleLocalAddressChange}
          InputProps={{
            endAdornment: <CopyToClipboard copy={localAddress} />,
          }}
        />
        <Button variant="contained" onClick={() => handleSetAddress(true)}>
          Save
        </Button>
        <Button variant="outlined" onClick={() => handleSetAddress(false)}>
          Cancel
        </Button>
      </Box>
    );
  };

  const ShaMining = () => {
    switch (containers?.sha3Miner?.status) {
      case ShaMiningStatus.WAITING:
      case ShaMiningStatus.SHUTTINGDOWN:
      case ShaMiningStatus.STARTING:
      case ShaMiningStatus.PENDING:
        return (
          <ShaMiningBox>
            <MiningBoxInner>
              <Box>
                <StatusChip
                  label={
                    <span>
                      <strong>{containers.sha3Miner?.status}</strong>
                    </span>
                  }
                  color="info"
                />
              </Box>
              <MiningTitle />
              <CircularProgress />
              <Box>
                <TransparentButton onClick={stop}>
                  {t.common.verbs.cancel}
                </TransparentButton>
              </Box>
            </MiningBoxInner>
            <SignetBox />
          </ShaMiningBox>
        );
      case ShaMiningStatus.ACTIVE:
        return (
          <ShaMiningBox>
            <MiningBoxInner>
              <Box>
                <StatusChip
                  label={
                    <span>
                      <strong>{t.common.adjectives.running}</strong>
                    </span>
                  }
                  color="success"
                />
              </Box>
              <MiningTitle />
              <Typography variant="body1" sx={typography.defaultMedium}>
                00 000 XTR
              </Typography>
              <MiningButton />
            </MiningBoxInner>
            <SignetBox />
          </ShaMiningBox>
        );
      case ShaMiningStatus.INACTIVE:
      default:
        return (
          <MiningBoxOuter>
            <MiningBoxInner>
              <Chip
                label={
                  <span>
                    <strong>{t.common.phrases.startHere}</strong>
                  </span>
                }
                color="info"
              />
              <MiningTitle />
              <Typography variant="body1" sx={typography.defaultMedium}>
                {t.walletPasswordWizard.description}
              </Typography>
              <TariAddressTextField />

              <Button
                variant="contained"
                onClick={start}
                style={{
                  minWidth: '120px',
                }}
              >
                {t.common.verbs.start}
              </Button>
            </MiningBoxInner>
            <SignetBox />
          </MiningBoxOuter>
        );
    }
  };

  return <ShaMining />;
}

export default MiningWidget;
