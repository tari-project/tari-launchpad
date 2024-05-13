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
import { MergeMiningStatus } from '../../../store/types';
import {
  StatusChip,
  MiningBoxOuter,
  MiningBoxInner,
  MiningButtonBox,
  TransparentButton,
  MergeMiningBox,
  StyledIconButton,
} from '../../../components/StyledComponents';
import { useTheme } from '@mui/material/styles';
import SvgTariSignet from '../../../styles/Icons/TariSignet';
import SvgMoneroSignet from '../../../styles/Icons/MoneroSignet';
import SvgQuestion from '../../../styles/Icons/Question';
import CopyToClipboard from '../../../components/CopyToClipboard';
import { useSnackbar } from 'notistack';

function MergeMiningWidget() {
  const {
    appState,
    containers,
    moneroAddress,
    setMoneroAddress,
    saveMoneroAddress,
    startMining,
    stopMining,
  } = useAppStateStore();
  const theme = useTheme();
  const { enqueueSnackbar } = useSnackbar();

  const mergeMiningHelp = () => {
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

  function start() {
    startMining('Merge');
  }

  function stop() {
    stopMining('Merge');
  }

  const SignetBox = () => {
    return (
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
    );
  };

  const MiningTitle = () => {
    return (
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
        <StyledIconButton onClick={() => mergeMiningHelp()}>
          <SvgQuestion color={theme.palette.primary.main} />
        </StyledIconButton>
      </Box>
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

  const MoneroAddressTextField = () => {
    const [localAddress, setLocalAddress] = useState(moneroAddress);

    const handleLocalAddressChange = (
      event: React.ChangeEvent<HTMLInputElement>
    ) => {
      setLocalAddress(event.target.value);
    };

    function handleSetAddress(save: boolean) {
      if (save) {
        saveMoneroAddress(localAddress);
      } else {
        setMoneroAddress(
          appState?.config?.settings?.saved_settings?.xmrig
            .monero_mining_address || ''
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
          placeholder="Monero Address"
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

  const MergeMining = () => {
    switch (containers?.mmProxy?.status) {
      case MergeMiningStatus.WAITING:
      case MergeMiningStatus.SHUTTINGDOWN:
      case MergeMiningStatus.STARTING:
      case MergeMiningStatus.PENDING:
        return (
          <MergeMiningBox>
            <MiningBoxInner>
              <StatusChip
                label={
                  <span>
                    <strong>{containers.mmProxy?.status}</strong>
                  </span>
                }
                color="info"
              />
              <MiningTitle />
              <CircularProgress />
              <Box>
                <TransparentButton onClick={stop}>
                  {t.common.verbs.cancel}
                </TransparentButton>
              </Box>
            </MiningBoxInner>
            <SignetBox />
          </MergeMiningBox>
        );
      case MergeMiningStatus.ACTIVE:
        return (
          <MergeMiningBox>
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
          </MergeMiningBox>
        );
      case MergeMiningStatus.INACTIVE:
      default:
        return (
          <MiningBoxOuter>
            <MiningBoxInner>
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
              <MiningTitle />
              <Typography variant="body2" sx={typography.defaultMedium}>
                {t.mining.setup.description}{' '}
                <span style={typography.defaultHeavy}>
                  {t.mining.setup.descriptionBold}
                </span>
              </Typography>
              <MoneroAddressTextField />
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

  return <MergeMining />;
}

export default MergeMiningWidget;
