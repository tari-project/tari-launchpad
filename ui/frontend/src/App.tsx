import { useEffect } from 'react';

// import { invoke } from "@tauri-apps/api/tauri";
// import './App.css';
import { emit, listen } from '@tauri-apps/api/event';
import MainLayout from './theme/MainLayout';
import useAppStateStore from './store/appStateStore';
import useConfigStore from './store/configStore';
import MainTabs from './containers/Dashboard/DashboardContainer/MainTabs';
import SettingsDialog from './containers/SettingsContainer/SettingsDialog';
import DockerWarning from './containers/DockerWarning/DockerWarning';
import MiningScheduleDialog from './containers/MiningContainer/MiningSchedule/MiningScheduleDialog';
import { useShallow } from 'zustand/react/shallow';

const containerIdToKey: { [key: string]: string | undefined} = {
  'Tor': 'tor',
  'Base Node': 'baseNode',
  'Sha3Miner': 'sha3Miner',
  'SharedVolume': 'sharedVolume',
  'MM proxy': 'mmProxy',
  'Loki': 'loki',
  'Grafana': 'grafana',
  'Xmrig': 'xmrig',
};

function App() {
  const {
    appState,
    setAppState,
    containers,
    setContainers,
    setIsMining,
    setIsChangingMining,
    openDockerWarning,
    setOpenDockerWarning,
    setTariAddress,
    setNetwork,
    openSettings,
    shaTime,
    setShaTime,
    shaTimerOn,
    mergeTime,
    setMergeTime,
    mergeTimerOn,
    openSchedule,
    startBaseNode,
    startMining,
    setIsZippingLogs
  } = useAppStateStore(
    useShallow((state) => ({
      appState: state.appState,
      setAppState: state.setAppState,
      containers: state.containers,
      setContainers: state.setContainers,
      setIsMining: state.setIsMining,
      setIsChangingMining: state.setIsChangingMining,
      openDockerWarning: state.openDockerWarning,
      setOpenDockerWarning: state.setOpenDockerWarning,
      setTariAddress: state.setTariAddress,
      setNetwork: state.setNetwork,
      openSettings: state.openSettings,
      shaTime: state.shaTime,
      setShaTime: state.setShaTime,
      shaTimerOn: state.shaTimerOn,
      mergeTime: state.mergeTime,
      setMergeTime: state.setMergeTime,
      mergeTimerOn: state.mergeTimerOn,
      openSchedule: state.openSchedule,
      startBaseNode: state.startBaseNode,
      startMining: state.startMining,
      setIsZippingLogs: state.setIsZippingLogs,
    }))
  );
  const { startupConfig } = useConfigStore();

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
            Object.keys(containerIdToKey).forEach((id) => {
              const key = containerIdToKey[id];
              if (key) {
                newContainers[key] = normalizeContainer(payload?.State?.containers[id]);
              }
            });

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
            const { id, delta } = payload?.Delta.TaskDelta;
            const key = containerIdToKey[id];
            const newContainers = { ...containers };
            
            if (delta.UpdateStatus) {
              if (appState.containers[id]) {
                const newState = { ...appState };
                newState.containers[id].status = delta.UpdateStatus;
                setAppState(newState);
              }
              
              if (key && newContainers[key]) {
                newContainers[key].status = printStatus(delta.UpdateStatus);
                if (newContainers[key].status === "Inactive") {
                  newContainers[key].stats = {};
                }
                setContainers(newContainers);
              }
            }
          
            if (delta.StatsRecord && key) {
              if (delta.StatsRecord.timestamp !== newContainers[key].stats?.timestamp) {
                
                const lastCpu = newContainers[key].stats?.cpu_usage;
                const lastSystemCpu = newContainers[key].stats?.system_cpu_usage;
                newContainers[key].stats = delta.StatsRecord;

                const cpuUsagePercent = ((delta.StatsRecord.cpu_usage - lastCpu) / (delta.StatsRecord.system_cpu_usage - lastSystemCpu)) * 100;
                newContainers[key].stats.cpu = !isNaN(cpuUsagePercent) ? cpuUsagePercent : 0;
                setContainers(newContainers);
              }
            }
          
            if (!delta.UpdateStatus && !delta.StatsRecord && !delta.LogRecord) {
              console.log('Unknown delta: ' + JSON.stringify(delta));
            }
          }
        }
        if (payload === "LogsZipped") {
          setIsZippingLogs(false);
        }
      }))();

    return () => {
      (async () => (await unlisten)())();
    };
  });

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

  // let state: any = appState;
  //  let containers: any = state.containers;
  // console.log(containers);

  useEffect(() => {
    setNetwork(appState?.config?.settings?.saved_settings?.tari_network || '');
  }, [appState?.config?.settings?.saved_settings?.tari_network]);

  // settings that should run on first startup
  useEffect(() => {
    if (startupConfig.shaMine) {
      setTimeout(() => {
        startMining('Sha');
      }, 2000);
    }
    if (startupConfig.mergeMine) {
      setTimeout(() => {
        startMining('Merge');
      }, 2000);
    }
    if (startupConfig.baseNode) {
      setTimeout(() => {
        startBaseNode();
      }, 2000);
    }
  }, []);

  console.log(appState);

  useEffect(() => {
    let intervalId: any;
    let prevTime = shaTime;

    if (shaTimerOn) {
      intervalId = setInterval(() => {
        prevTime = prevTime + 1;
        setShaTime(prevTime);
      }, 1000);
    } else {
      clearInterval(intervalId);
    }

    return () => clearInterval(intervalId);
  }, [shaTimerOn]);

  useEffect(() => {
    let intervalId: any;
    let prevTime = mergeTime;

    if (mergeTimerOn) {
      intervalId = setInterval(() => {
        prevTime = prevTime + 1;
        setMergeTime(prevTime);
      }, 1000);
    } else {
      clearInterval(intervalId);
    }

    return () => clearInterval(intervalId);
  }, [mergeTimerOn]);

  return (
    <MainLayout>
      <MainTabs />
      {openDockerWarning && <DockerWarning />}
      {openSettings && <SettingsDialog />}
      {openSchedule && <MiningScheduleDialog />}
    </MainLayout>
  );
}

export default App;
