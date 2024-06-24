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
import { MergeMiningStatus } from '../../../store/types';
import {
  StatusChip,
  TransparentButton,
  StyledIconButton,
  LabelBoxVertical,
} from '../../../components/StyledComponents';
import {
  MiningBoxOuter,
  MiningBoxInner,
  MergeMiningBox,
  ContentBox,
} from '../styles';
import { useTheme } from '@mui/material/styles';
import SvgTariSignet from '../../../styles/Icons/TariSignet';
import SvgMoneroSignet from '../../../styles/Icons/MoneroSignet';
import CopyToClipboard from '../../../components/CopyToClipboard';
import Timer from '../components/Timer';
import Amount from '../components/Amount';
import CloseRoundedIcon from '@mui/icons-material/CloseRounded';
import CheckRoundedIcon from '@mui/icons-material/CheckRounded';
import { MergedMiningHelp } from '../MiningHelp/Messages';

type Status = 'inactive' | 'pending' | 'active';

function MergeMiningWidget() {
  const {
    appState,
    containers,
    moneroAddress,
    setMoneroAddress,
    saveMoneroAddress,
    startMining,
    stopMining,
    setMergeTimerOn,
    mergeTime,
    setMergeTime,
  } = useAppStateStore((state) => ({
    appState: state.appState,
    containers: state.containers,
    moneroAddress: state.moneroAddress,
    setMoneroAddress: state.setMoneroAddress,
    saveMoneroAddress: state.saveMoneroAddress,
    startMining: state.startMining,
    stopMining: state.stopMining,
    setMergeTimerOn: state.setMergeTimerOn,
    mergeTime: state.mergeTime,
    setMergeTime: state.setMergeTime,
  }));
  const theme = useTheme();
  const [miningStatus, setMiningStatus] = useState<Status>('inactive');

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

  useEffect(() => {
    if (
      containers?.mmProxy?.status === MergeMiningStatus.WAITING ||
      containers?.mmProxy?.status === MergeMiningStatus.SHUTTINGDOWN ||
      containers?.mmProxy?.status === MergeMiningStatus.STARTING ||
      containers?.mmProxy?.status === MergeMiningStatus.PENDING
    ) {
      setMiningStatus('pending');
    } else if (containers?.mmProxy?.status === MergeMiningStatus.ACTIVE) {
      setMiningStatus('active');
    } else {
      setMiningStatus('inactive');
    }
  }, [containers?.mmProxy?.status]);

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
        <MergedMiningHelp miningStatus={miningStatus} />
      </Box>
    );
  };

  const MoneroAddressTextField = () => {
    const [localAddress, setLocalAddress] = useState(moneroAddress);
    const [isDirty, setIsDirty] = useState(false);

    const handleLocalAddressChange = (
      event: React.ChangeEvent<HTMLInputElement>
    ) => {
      setLocalAddress(event.target.value);
      setIsDirty(true);
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
          sx={typography.defaultMedium}
          style={{
            color: theme.palette.text.secondary,
          }}
        >
          {t.mining.settings.moneroAddressLabel}
        </Typography>
        <TextField
          placeholder={t.mining.settings.moneroAddressLabel}
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
        <MergeMiningBox>
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
            <ContentBox>
              <Typography variant="body1" sx={typography.smallMedium}>
                <span style={typography.smallHeavy}>Hash rate:</span> 0KH
              </Typography>
              <Timer
                miningType="Merge"
                setTimerOn={setMergeTimerOn}
                time={mergeTime}
                setTime={setMergeTime}
              />
            </ContentBox>
          </MiningBoxInner>
          <SignetBox />
        </MergeMiningBox>
      );
    case 'pending':
      return (
        <MergeMiningBox>
          <MiningBoxInner>
            <ContentBox>
              <StatusChip
                label={
                  <span>
                    <strong>{containers.mmProxy?.status}</strong>
                  </span>
                }
                color="info"
              />
              <MiningTitle />
            </ContentBox>
            {containers?.mmProxy?.status !== MergeMiningStatus.SHUTTINGDOWN && (
              <TransparentButton onClick={stop}>
                {t.common.verbs.cancel}
              </TransparentButton>
            )}
          </MiningBoxInner>
          <SignetBox />
        </MergeMiningBox>
      );
    case 'inactive':
    default:
      return moneroAddress === '' ? (
        <MiningBoxOuter>
          <MiningBoxInner>
            <ContentBox>
              <Chip
                label={
                  <span>
                    <strong>{t.common.phrases.readyToSet}</strong>
                  </span>
                }
                color="info"
              />
              <MiningTitle />
              <Typography
                variant="body2"
                sx={typography.defaultMedium}
                style={{
                  color: theme.palette.text.secondary,
                }}
              >
                {t.mining.setup.description}{' '}
                <span style={typography.defaultHeavy}>
                  {t.mining.setup.descriptionBold}
                </span>
              </Typography>
              <MoneroAddressTextField />
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
                    <strong>{t.common.phrases.readyToGo}</strong>
                  </span>
                }
                color="info"
              />
              <MiningTitle />
              <Typography
                variant="body2"
                sx={typography.defaultMedium}
                style={{
                  color: theme.palette.text.secondary,
                }}
              >
                {t.mining.readyToMiningText}
              </Typography>
              <MoneroAddressTextField />
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

export default MergeMiningWidget;
