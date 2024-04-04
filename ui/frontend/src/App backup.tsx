import { useEffect } from 'react';
// import reactLogo from "../src/assets/react.svg";

// import { invoke } from "@tauri-apps/api/tauri";
// import './App.css';
import { emit, listen } from '@tauri-apps/api/event';
import {
  Button,
  CircularProgress,
  Dialog,
  DialogActions,
  DialogContent,
  DialogContentText,
  DialogTitle,
  Divider,
  IconButton,
  Switch,
  TextField,
  Typography,
  Box,
} from '@mui/material';
import { IoChevronForward as PlayArrowIcon } from 'react-icons/io5';
import { IoPauseCircleOutline as PauseIcon } from 'react-icons/io5';
import { IoSettingsOutline as SettingsIcon } from 'react-icons/io5';
import { exit } from '@tauri-apps/api/process';
import { open } from '@tauri-apps/api/shell';
import MainLayout from './MainLayout';
import { StyledPaper } from './components/UI/StyledComponents';
import { styled } from '@mui/material/styles';
import useAppStateStore from './store/appStore';
import MainTabs from './components/Main/MainTabs';
import SettingsTabs from './components/Settings/SettingsTabs';

const CustomGrid = styled(Box)(({ theme }) => ({
  display: 'grid',
  gridTemplateColumns: '1fr 1fr 100px',
  gridGap: theme.spacing(1),
  width: '100%',
}));

const CustomGridContainer = styled(Box)(({ theme }) => ({
  display: 'flex',
  width: '100%',
  flexDirection: 'column',
  gap: theme.spacing(1),
}));

function App() {
  const {
    appState,
    setAppState,
    containers,
    setContainers,
    isMining,
    setIsMining,
    shaMiningEnabled,
    setShaMiningEnabled,
    mergeMiningEnabled,
    setMergeMiningEnabled,
    isChangingMining,
    setIsChangingMining,
    openDockerWarning,
    setOpenDockerWarning,
    openSettings,
    setOpenSettings,
    tariAddress,
    setTariAddress,
  } = useAppStateStore();

  //   async function connect() {
  //     // Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
  //     //setGreetMsg(await invoke("greet", { name }));
  //     emit('tari://actions', { Action: { type: 'Connect' } });
  //   }

  // This only happens once
  useEffect(() => {
    // wait for listener to have been set up
    setTimeout(() => {
      console.log('Connecting');
      emit('tari://actions', { Action: { type: 'Connect' } });
    }, 1000);

    // setInterval(function () {
    //   emit("tari://actions", { "Action": { type: "Connect" } });
    // }, 1000);
  }, []);

  // this needs to happen every state refresh
  useEffect(() => {
    let unlisten = (async () =>
      await listen('tari://reactions', (event) => {
        let payload: any = event.payload;
        // console.log(event);
        if (payload?.State !== undefined) {
          setAppState(payload?.State);
          //  console.log(payload?.State);
          let newContainers: any = { ...containers };
          if (payload?.State?.containers !== undefined) {
            // Check if docker is running
            // if (payload?.State?.containers["Tor"].status.hasOwnProperty("Failed")) {
            // console.log("Docker is not running");
            // setOpenDockerWarning(true);
            // return;
            // }

            // We have to do this because some supersmart developer
            // used strings as keys with spaces in them
            newContainers.tor = normalizeContainer(
              payload?.State?.containers['Tor']
            );
            newContainers.baseNode = normalizeContainer(
              payload?.State?.containers['Base Node']
            );
            newContainers.sha3Miner = normalizeContainer(
              payload?.State?.containers['Sha3Miner']
            );
            newContainers.sharedVolume = normalizeContainer(
              payload?.State?.containers['SharedVolume']
            );
            newContainers.mmProxy = normalizeContainer(
              payload?.State?.containers['MM proxy']
            );
            newContainers.loki = normalizeContainer(
              payload?.State?.containers['Loki']
            );
            newContainers.grafana = normalizeContainer(
              payload?.State?.containers['Grafana']
            );
            newContainers.xmrig = normalizeContainer(
              payload?.State?.containers['Xmrig']
            );
            setContainers(newContainers);

            setIsMining(
              payload?.State?.config?.session?.merge_layer_active ||
                payload?.State?.config?.session?.sha3x_layer_active
            );
            setIsChangingMining(false);
            setTariAddress(
              appState?.config?.settings?.saved_settings?.mm_proxy
                .wallet_payment_address ||
                appState?.config?.settings?.saved_settings?.sha3_miner
                  ?.wallet_payment_address ||
                ''
            );
          }
        }
        if (payload?.Delta !== undefined) {
          if (payload?.Delta.UpdateSession) {
            let newState: any = appState;
            newState.config.session = payload?.Delta.UpdateSession;
            console.log(newState);
            setIsChangingMining(false);
            setAppState(newState);
            setIsMining(
              newState.config?.session?.merge_layer_active ||
                newState.config?.session?.sha3x_layer_active
            );
          }
          if (payload?.Delta.TaskDelta) {
            let delta: any = payload?.Delta.TaskDelta?.delta;
            // console.log(delta);
            let id = payload?.Delta.TaskDelta?.id;
            if (delta.UpdateStatus) {
              let newState: any = { ...appState };
              // console.log(delta.UpdateStatus);

              newState.containers[payload?.Delta.TaskDelta?.id].status =
                delta.UpdateStatus;
              // if (delta.UpdateStatus?.Progress) {
              // newState.containers[payload?.Delta.TaskDelta?.id].status = delta.UpdateStatus?.Progress?.stage;
              // setAppState(newState);
              // }
              setAppState(newState);
              let newContainers: any = {
                ...containers,
              };
              if (id === 'Tor') {
                newContainers.tor.status = printStatus(delta.UpdateStatus);
              }
              if (id === 'Base Node') {
                newContainers.baseNode.status = printStatus(delta.UpdateStatus);
              }
              if (id === 'Sha3Miner') {
                newContainers.sha3Miner.status = printStatus(
                  delta.UpdateStatus
                );
              }
              if (id === 'SharedVolume') {
                newContainers.sharedVolume.status = printStatus(
                  delta.UpdateStatus
                );
              }
              if (id === 'MM proxy') {
                newContainers.mmProxy.status = printStatus(delta.UpdateStatus);
              }
              if (id === 'Loki') {
                newContainers.loki.status = printStatus(delta.UpdateStatus);
              }
              if (id === 'Grafana') {
                newContainers.grafana.status = printStatus(delta.UpdateStatus);
              }
              if (id === 'Xmrig') {
                newContainers.xmrig.status = printStatus(delta.UpdateStatus);
              }
              setContainers(newContainers);
            }
            // stats records
            if (delta.StatsRecord) {
              let newContainers: any = {
                ...containers,
              };
              if (id === 'Tor') {
                // console.log(delta.StatsRecord);
                if (
                  delta.StatsRecord.timestamp !==
                  newContainers.tor.stats?.timestamp
                ) {
                  let last_cpu = newContainers.tor.stats?.cpu_usage;
                  let last_system_cpu =
                    newContainers.tor.stats?.system_cpu_usage;
                  newContainers.tor.stats = delta.StatsRecord;
                  newContainers.tor.stats.cpu =
                    ((delta.StatsRecord.cpu_usage - last_cpu) /
                      (delta.StatsRecord.system_cpu_usage - last_system_cpu)) *
                    100;
                }
              }
              if (id === 'Base Node') {
                if (
                  delta.StatsRecord.timestamp !==
                  newContainers.baseNode.stats?.timestamp
                ) {
                  let last_cpu = newContainers.baseNode.stats?.cpu_usage;
                  let last_system_cpu =
                    newContainers.baseNode.stats?.system_cpu_usage;
                  newContainers.baseNode.stats = delta.StatsRecord;
                  newContainers.baseNode.stats.cpu =
                    ((delta.StatsRecord.cpu_usage - last_cpu) /
                      (delta.StatsRecord.system_cpu_usage - last_system_cpu)) *
                    100;
                }
              }
              if (id === 'Sha3Miner') {
                if (
                  delta.StatsRecord.timestamp !==
                  newContainers.sha3Miner.stats?.timestamp
                ) {
                  let last_cpu = newContainers.sha3Miner.stats?.cpu_usage;
                  let last_system_cpu =
                    newContainers.sha3Miner.stats?.system_cpu_usage;
                  newContainers.sha3Miner.stats = delta.StatsRecord;
                  newContainers.sha3Miner.stats.cpu =
                    ((delta.StatsRecord.cpu_usage - last_cpu) /
                      (delta.StatsRecord.system_cpu_usage - last_system_cpu)) *
                    100;
                }
              }
              if (id === 'SharedVolume') {
                if (
                  delta.StatsRecord.timestamp !==
                  newContainers.sharedVolume.stats?.timestamp
                ) {
                  let last_cpu = newContainers.sharedVolume.stats?.cpu_usage;
                  let last_system_cpu =
                    newContainers.sharedVolume.stats?.system_cpu_usage;
                  newContainers.sharedVolume.stats = delta.StatsRecord;
                  newContainers.sharedVolume.stats.cpu =
                    ((delta.StatsRecord.cpu_usage - last_cpu) /
                      (delta.StatsRecord.system_cpu_usage - last_system_cpu)) *
                    100;
                }
              }
              if (id === 'MM proxy') {
                if (
                  delta.StatsRecord.timestamp !==
                  newContainers.mmProxy.stats?.timestamp
                ) {
                  let last_cpu = newContainers.mmProxy.stats?.cpu_usage;
                  let last_system_cpu =
                    newContainers.mmProxy.stats?.system_cpu_usage;
                  newContainers.mmProxy.stats = delta.StatsRecord;
                  newContainers.mmProxy.stats.cpu =
                    ((delta.StatsRecord.cpu_usage - last_cpu) /
                      (delta.StatsRecord.system_cpu_usage - last_system_cpu)) *
                    100;
                }
              }
              if (id === 'Loki') {
                if (
                  delta.StatsRecord.timestamp !==
                  newContainers.loki.stats?.timestamp
                ) {
                  let last_cpu = newContainers.loki.stats?.cpu_usage;
                  let last_system_cpu =
                    newContainers.loki.stats?.system_cpu_usage;
                  newContainers.loki.stats = delta.StatsRecord;
                  newContainers.loki.stats.cpu =
                    ((delta.StatsRecord.cpu_usage - last_cpu) /
                      (delta.StatsRecord.system_cpu_usage - last_system_cpu)) *
                    100;
                }
              }
              if (id === 'Grafana') {
                if (
                  delta.StatsRecord.timestamp !==
                  newContainers.grafana.stats?.timestamp
                ) {
                  let last_cpu = newContainers.grafana.stats?.cpu_usage;
                  let last_system_cpu =
                    newContainers.grafana.stats?.system_cpu_usage;
                  newContainers.grafana.stats = delta.StatsRecord;
                  newContainers.grafana.stats.cpu =
                    ((delta.StatsRecord.cpu_usage - last_cpu) /
                      (delta.StatsRecord.system_cpu_usage - last_system_cpu)) *
                    100;
                }
              }
              if (id === 'Xmrig') {
                if (
                  delta.StatsRecord.timestamp !==
                  newContainers.xmrig.stats?.timestamp
                ) {
                  let last_cpu = newContainers.xmrig.stats?.cpu_usage;
                  let last_system_cpu =
                    newContainers.xmrig.stats?.system_cpu_usage;
                  newContainers.xmrig.stats = delta.StatsRecord;
                  newContainers.xmrig.stats.cpu =
                    ((delta.StatsRecord.cpu_usage - last_cpu) /
                      (delta.StatsRecord.system_cpu_usage - last_system_cpu)) *
                    100;
                }
              }
              setContainers(newContainers);
            }
            if (!delta.UpdateStatus && !delta.StatsRecord && !delta.LogRecord) {
              // No need for log records at this point
              console.log('Unknown delta: ' + JSON.stringify(delta));
            }
          }
        }
      }))();

    return () => {
      (async () => (await unlisten)())();
    };
  });

  async function toggleMining() {
    if (isChangingMining) {
      return;
    }
    setIsChangingMining(true);
    if (isMining) {
      stopMining();
    } else {
      startMining();
    }
  }

  async function startMining() {
    let state: any = appState;
    let stateSession = { ...state?.config?.session };
    stateSession.merge_layer_active = mergeMiningEnabled;
    stateSession.sha3x_layer_active = shaMiningEnabled;
    emit('tari://actions', {
      Action: { type: 'ChangeSession', payload: stateSession },
    });
  }

  async function stopMining() {
    let state: any = appState;
    let stateSession = { ...state?.config?.session };
    stateSession.merge_layer_active = false;
    stateSession.sha3x_layer_active = false;
    emit('tari://actions', {
      Action: { type: 'ChangeSession', payload: stateSession },
    });
  }

  async function toggleMergeMiningEnabled(
    event: React.ChangeEvent<HTMLInputElement>
  ) {
    setMergeMiningEnabled(event.target.checked);
  }

  async function toggleShaMiningEnabled(
    event: React.ChangeEvent<HTMLInputElement>
  ) {
    setShaMiningEnabled(event.target.checked);
  }

  async function handleDockerClose() {
    setOpenDockerWarning(false);
    await exit(1);
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

  function handleTariAddressChange(event: React.ChangeEvent<HTMLInputElement>) {
    setTariAddress(event.target.value);
  }

  function printStatus(status: any) {
    if (status === undefined) {
      return '...';
    }
    // Some clever developer thought it was a good idea sometimes to return a string and sometimes an object
    if (status.hasOwnProperty('Progress')) {
      return status.Progress.stage;
    }

    if (status.hasOwnProperty('Failed')) {
      setOpenDockerWarning(true);
      return 'Failed:' + status.Failed;
    }
    return status;
  }

  // async function shaMine() {
  //   let state: any = appState;
  //   let stateSession = { ...state?.config?.session };
  //   stateSession.sha3x_layer_active = stateSession.sha3x_layer_active ? false : true;
  //   emit("tari://actions", { "Action": { type: "ChangeSession", payload: stateSession } });
  // }

  function normalizeContainer(container: any) {
    return {
      ...container,
      status: printStatus(container.status),
    };
  }

  async function openDockerInstall(evt: any) {
    evt.preventDefault();
    open('https://docs.docker.com/engine/install/');
  }

  function handleOpenSettings() {
    setTariAddress(
      appState?.config?.settings?.saved_settings?.mm_proxy
        .wallet_payment_address ||
        appState?.config?.settings?.saved_settings?.sha3_miner
          ?.wallet_payment_address ||
        ''
    );
    setOpenSettings(true);
  }

  // let state: any = appState;
  //  let containers: any = state.containers;
  // console.log(containers);

  return (
    <MainLayout>
      <IconButton onClick={() => handleOpenSettings()}>
        <SettingsIcon />
      </IconButton>
      {isChangingMining ? (
        <CircularProgress color="inherit" />
      ) : (
        <IconButton onClick={() => toggleMining()}>
          {isMining ? <PauseIcon /> : <PlayArrowIcon />}
        </IconButton>
      )}
      <Typography pt={2} variant="body2">
        {isChangingMining
          ? 'Busy ...'
          : isMining
          ? 'Pause Mining'
          : 'Start Mining'}
      </Typography>
      <MainTabs />
      <StyledPaper
        style={{
          width: '100%',
        }}
      >
        <CustomGridContainer>
          <CustomGrid>
            <Typography variant="h6">Algorithm</Typography>
            <Typography variant="h6">Status</Typography>
            <Typography variant="h6">Enabled</Typography>
          </CustomGrid>
          <Divider />
          <CustomGrid>
            <Typography variant="h6">Merge Mining with Monero</Typography>
            <Typography variant="body2">
              {containers ? containers.xmrig?.status : '...'}
            </Typography>
            <Switch
              checked={mergeMiningEnabled}
              onChange={toggleMergeMiningEnabled}
            />
          </CustomGrid>
          <Divider />
          <CustomGrid>
            <Typography variant="h6">SHA3</Typography>
            <Typography variant="body2">
              {containers ? containers?.sha3Miner?.status : '...'}
            </Typography>
            <Switch
              checked={shaMiningEnabled}
              onChange={toggleShaMiningEnabled}
            />
          </CustomGrid>
        </CustomGridContainer>
      </StyledPaper>
      <Dialog
        open={openDockerWarning}
        onClose={handleDockerClose}
        aria-labelledby="alert-dialog-title"
        aria-describedby="alert-dialog-description"
      >
        <DialogTitle id="alert-dialog-title">Docker is not running</DialogTitle>
        <DialogContent>
          <DialogContentText id="alert-dialog-description">
            Tari Launchpad requires Docker to be running. Please start Docker
            and try again. If you don't have Docker installed, you can download
            it from{' '}
            <a onClick={(evt) => openDockerInstall(evt)} href="#">
              here
            </a>
            .
          </DialogContentText>
        </DialogContent>
        <DialogActions>
          <Button onClick={handleDockerClose}>Exit</Button>
        </DialogActions>
      </Dialog>
      <Dialog
        open={openSettings}
        onClose={() => handleSettingsClose(false)}
        aria-labelledby="alert-dialog-title"
        aria-describedby="alert-dialog-description"
        fullWidth
        maxWidth="md"
      >
        <DialogContent>
          <SettingsTabs />
          <DialogContentText id="alert-dialog-description">
            <TextField
              label="Tari Address"
              style={{
                width: 300,
              }}
              value={tariAddress}
              onChange={handleTariAddressChange}
              size="small"
            />
          </DialogContentText>
        </DialogContent>
        <DialogActions>
          <Button onClick={() => handleSettingsClose(true)}>Save</Button>
          <Button onClick={() => handleSettingsClose(false)}>Exit</Button>
        </DialogActions>
      </Dialog>
    </MainLayout>
  );
}

export default App;
