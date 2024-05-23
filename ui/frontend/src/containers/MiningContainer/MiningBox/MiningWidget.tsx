import { useEffect, useState } from 'react';
import {
  Button,
  Chip,
  Typography,
  TextField,
  Box,
  ButtonGroup,
} from '@mui/material';
import t from '../../../locales';
import typography from '../../../styles/styles/typography';
import useAppStateStore from '../../../store/appStateStore';
import { ShaMiningStatus } from '../../../store/types';
import {
  StatusChip,
  TransparentButton,
  LabelBoxVertical,
  StyledIconButton,
} from '../../../components/StyledComponents';
import {
  ShaMiningBox,
  MiningBoxInner,
  MiningBoxOuter,
  ContentBox,
} from '../styles';
import { useTheme } from '@mui/material/styles';
import SvgTariSignet from '../../../styles/Icons/TariSignet';
import CopyToClipboard from '../../../components/CopyToClipboard';
import Timer from '../components/Timer';
import Amount from '../components/Amount';
import CloseRoundedIcon from '@mui/icons-material/CloseRounded';
import CheckRoundedIcon from '@mui/icons-material/CheckRounded';

type Status = 'inactive' | 'pending' | 'active';

function MiningWidget() {
  const {
    appState,
    containers,
    tariAddress,
    setTariAddress,
    saveTariAddress,
    startMining,
    stopMining,
    setShaTimerOn,
    shaTime,
    setShaTime,
  } = useAppStateStore((state) => ({
    appState: state.appState,
    containers: state.containers,
    tariAddress: state.tariAddress,
    setTariAddress: state.setTariAddress,
    saveTariAddress: state.saveTariAddress,
    startMining: state.startMining,
    stopMining: state.stopMining,
    setShaTimerOn: state.setShaTimerOn,
    shaTime: state.shaTime,
    setShaTime: state.setShaTime,
  }));
  const theme = useTheme();
  const [miningStatus, setMiningStatus] = useState<Status>('inactive');

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
    startMining('Sha');
  }

  function stop() {
    stopMining('Sha');
  }

  useEffect(() => {
    if (
      containers.sha3Miner?.status === ShaMiningStatus.WAITING ||
      containers.sha3Miner?.status === ShaMiningStatus.SHUTTINGDOWN ||
      containers.sha3Miner?.status === ShaMiningStatus.STARTING ||
      containers.sha3Miner?.status === ShaMiningStatus.PENDING
    ) {
      setMiningStatus('pending');
    } else if (containers.sha3Miner?.status === ShaMiningStatus.ACTIVE) {
      setMiningStatus('active');
    } else {
      setMiningStatus('inactive');
    }
  }, [containers.sha3Miner?.status]);

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

  const TariAddressTextField = () => {
    const [localAddress, setLocalAddress] = useState(tariAddress);
    const [isDirty, setIsDirty] = useState(false);

    const handleLocalAddressChange = (
      event: React.ChangeEvent<HTMLInputElement>
    ) => {
      setLocalAddress(event.target.value);
      setIsDirty(true);
    };

    const handleSetAddress = (save: boolean) => {
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
    };

    const handleKeyDown = (event: React.KeyboardEvent<HTMLInputElement>) => {
      if (event.key === 'Enter') {
        handleSetAddress(true);
        event.preventDefault();
      }
    };

    const ActionButtons = () => {
      return (
        <ButtonGroup>
          <StyledIconButton onClick={() => handleSetAddress(true)}>
            <CheckRoundedIcon
              style={{
                height: '16px',
                width: '16px',
              }}
            />
          </StyledIconButton>
          <StyledIconButton onClick={() => handleSetAddress(false)}>
            <CloseRoundedIcon
              style={{
                height: '16px',
                width: '16px',
              }}
            />
          </StyledIconButton>
        </ButtonGroup>
      );
    };

    return (
      <LabelBoxVertical
        style={{
          width: '100%',
        }}
      >
        <Typography
          variant="body1"
          sx={typography.smallMedium}
          style={{
            color: theme.palette.text.secondary,
          }}
        >
          {t.wallet.wallet.walletId} ({t.wallet.wallet.address})
        </Typography>
        <TextField
          placeholder={t.wallet.wallet.walletId}
          value={localAddress}
          onChange={handleLocalAddressChange}
          onKeyDown={handleKeyDown}
          InputProps={{
            endAdornment: isDirty ? (
              <ActionButtons />
            ) : (
              <CopyToClipboard copy={localAddress} />
            ),
            style: {
              paddingRight: '8px',
              paddingLeft: '8px',
            },
          }}
          style={{
            paddingRight: '0',
          }}
          fullWidth
        />
      </LabelBoxVertical>
    );
  };

  switch (miningStatus) {
    case 'active':
      return (
        <ShaMiningBox>
          <MiningBoxInner>
            <ContentBox>
              <StatusChip
                label={
                  <span>
                    <strong>{t.common.adjectives.running}</strong>
                  </span>
                }
                color="success"
              />
              <MiningTitle />
              <Amount amount={0} />
            </ContentBox>

            <Timer
              miningType="Sha"
              setTimerOn={setShaTimerOn}
              time={shaTime}
              setTime={setShaTime}
            />
          </MiningBoxInner>
          <SignetBox />
        </ShaMiningBox>
      );
    case 'pending':
      return (
        <ShaMiningBox>
          <MiningBoxInner>
            <ContentBox>
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
            </ContentBox>
            {containers.sha3Miner?.status !== ShaMiningStatus.SHUTTINGDOWN && (
              <Box>
                <TransparentButton onClick={stop}>
                  {t.common.verbs.cancel}
                </TransparentButton>
              </Box>
            )}
          </MiningBoxInner>
          <SignetBox />
        </ShaMiningBox>
      );
    case 'inactive':
    default:
      return tariAddress === '' ? (
        <MiningBoxOuter>
          <MiningBoxInner>
            <ContentBox>
              <Chip
                label={
                  <span>
                    <strong>{t.common.phrases.startHere}</strong>
                  </span>
                }
                color="info"
              />
              <MiningTitle />
              <Typography
                variant="body1"
                sx={typography.defaultMedium}
                style={{
                  color: theme.palette.text.secondary,
                }}
              >
                {t.walletPasswordWizard.description}
              </Typography>
              <TariAddressTextField />
            </ContentBox>
          </MiningBoxInner>
          <SignetBox />
        </MiningBoxOuter>
      ) : (
        <MiningBoxOuter>
          <MiningBoxInner>
            <ContentBox>
              <Chip
                label={
                  <span>
                    <strong>{t.common.phrases.startHere}</strong>
                  </span>
                }
                color="info"
              />
              <MiningTitle />
              <Typography
                variant="body1"
                sx={typography.defaultMedium}
                style={{
                  color: theme.palette.text.secondary,
                }}
              >
                {t.mining.readyToMiningText}
              </Typography>
              <TariAddressTextField />
            </ContentBox>
            <Button
              variant="contained"
              onClick={start}
              style={{
                minWidth: '120px',
              }}
            >
              {t.mining.actions.startMining}
            </Button>
          </MiningBoxInner>
          <SignetBox />
        </MiningBoxOuter>
      );
  }
}

export default MiningWidget;
