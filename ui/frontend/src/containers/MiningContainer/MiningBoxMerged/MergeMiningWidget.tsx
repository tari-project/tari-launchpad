import { useEffect, useState } from 'react';
import { Button, Chip, Typography, TextField, Box } from '@mui/material';
import t from '../../../locales';
import typography from '../../../styles/styles/typography';
import useAppStateStore from '../../../store/appStateStore';
import { MergeMiningStatus } from '../../../store/types';
import {
  StatusChip,
  TransparentButton,
  StyledIconButton,
  HorisontalButtons,
} from '../../../components/StyledComponents';
import {
  MiningBoxOuter,
  MiningBoxInner,
  MergeMiningBox,
  ContentBox,
  CircularProgressLight,
} from '../styles';
import { useTheme } from '@mui/material/styles';
import SvgTariSignet from '../../../styles/Icons/TariSignet';
import SvgMoneroSignet from '../../../styles/Icons/MoneroSignet';
import SvgQuestion from '../../../styles/Icons/Question';
import CopyToClipboard from '../../../components/CopyToClipboard';
import { useSnackbar } from 'notistack';
import Timer from '../components/Timer';
import Amount from '../components/Amount';

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
  } = useAppStateStore();
  const theme = useTheme();
  const [miningStatus, setMiningStatus] = useState<Status>('inactive');
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
        <StyledIconButton onClick={() => mergeMiningHelp()}>
          <SvgQuestion
            color={
              miningStatus === 'inactive' ? theme.palette.primary.main : '#FFF'
            }
          />
        </StyledIconButton>
      </Box>
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
          flexDirection: 'column',
          width: '100%',
          alignItems: 'flex-start',
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
        <HorisontalButtons>
          <Button variant="contained" onClick={() => handleSetAddress(true)}>
            Save
          </Button>
          <Button variant="outlined" onClick={() => handleSetAddress(false)}>
            Cancel
          </Button>
        </HorisontalButtons>
      </Box>
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
            <Timer
              miningType="Merge"
              setTimerOn={setMergeTimerOn}
              time={mergeTime}
              setTime={setMergeTime}
            />
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
              <CircularProgressLight />
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
