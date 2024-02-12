import { useEffect, useState } from "react";
// import reactLogo from "../src/assets/react.svg";

// import { invoke } from "@tauri-apps/api/tauri";
import "./App.css";
import { emit, listen } from '@tauri-apps/api/event'
import { Button, CircularProgress, Container, CssBaseline, Dialog, DialogActions, DialogContent, DialogContentText, DialogTitle, Divider, Grid, Switch, ThemeProvider, Typography } from "@mui/material";
import { createTheme, useTheme } from '@mui/material/styles';
import { componentSettings, dark } from './theme/tokens'
import { GradientPaper, TypographyData } from './components/StyledComponents';
import PlayArrowIcon from '@mui/icons-material/PlayArrow';
import PauseIcon from '@mui/icons-material/Pause';
import Logo from "./assets/Logo";
import { exit } from "@tauri-apps/api/process";
import { open } from '@tauri-apps/api/shell';

function App() {
  const [appState, setAppState]: [any, any] = useState({});
  const [containers, setContainers]: [any, any] = useState({});
  const [isMining, setIsMining] = useState(false);
  // const [logs, setLogs] = useState("");
  const [shaMiningEnabled, setShaMiningEnabled] = useState(true);
  const [mergeMiningEnabled, setMergeMiningEnabled] = useState(true);
  const [isChangingMining, setIsChangingMining] = useState(false);
  const [openDockerWarning, setOpenDockerWarning] = useState(false);

  const darkTheme = createTheme({
    ...dark,
    ...componentSettings,
  });

  const theme = useTheme();


  // async function connect() {
  //   // Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
  //   //setGreetMsg(await invoke("greet", { name }));
  //   emit("tari://actions", { "Action": { type: "Connect" } });
  // }

  // This only happens onces
  useEffect(() => {
    emit("tari://actions", { "Action": { type: "Connect" } });
  }, []);

  // this needs to happen every state refresh
  useEffect(() => {

    let unlisten = (async () => await listen("tari://reactions", (event) => {

      let payload: any = event.payload;
      //console.log(event);
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
          newContainers.tor = normalizeContainer(payload?.State?.containers["Tor"]);
          newContainers.baseNode = normalizeContainer(payload?.State?.containers["Base Node"]);
          newContainers.sha3Miner = normalizeContainer(payload?.State?.containers["Sha3Miner"]);
          newContainers.sharedVolume = normalizeContainer(payload?.State?.containers["SharedVolume"]);
          newContainers.mmProxy = normalizeContainer(payload?.State?.containers["MM proxy"]);
          newContainers.loki = normalizeContainer(payload?.State?.containers["Loki"]);
          newContainers.grafana = normalizeContainer(payload?.State?.containers["Grafana"]);
          newContainers.xmrig = normalizeContainer(payload?.State?.containers["Xmrig"]);
          setContainers(newContainers);

          setIsMining(payload?.State?.config?.session?.merge_layer_active || payload?.State?.config?.session?.sha3x_layer_active);
          setIsChangingMining(false);
        }
      }
      if (payload?.Delta !== undefined) {
        if (payload?.Delta.UpdateSession) {
          let newState: any = appState;
          newState.config.session = payload?.Delta.UpdateSession;
          setIsChangingMining(false);
          setAppState(newState);
          setIsMining(newState.config?.session?.merge_layer_active || newState.config?.session?.sha3x_layer_active);
        }
        if (payload?.Delta.TaskDelta) {

          let delta: any = payload?.Delta.TaskDelta?.delta;
          console.log(delta);
          let id = payload?.Delta.TaskDelta?.id;
          if (delta.UpdateStatus) {
            let newState: any = { ...appState };
            // console.log(delta.UpdateStatus);

            newState.containers[payload?.Delta.TaskDelta?.id].status = delta.UpdateStatus;
            // if (delta.UpdateStatus?.Progress) {
            // newState.containers[payload?.Delta.TaskDelta?.id].status = delta.UpdateStatus?.Progress?.stage;
            // setAppState(newState);
            // }
            setAppState(newState);
            let newContainers: any = {
              ...containers
            };
            if (id === "Tor") {
              newContainers.tor.status = printStatus(delta.UpdateStatus);
            }
            if (id === "Base Node") {
              newContainers.baseNode.status = printStatus(delta.UpdateStatus);
            }
            if (id === "Sha3Miner") {
              newContainers.sha3Miner.status = printStatus(delta.UpdateStatus);
            }
            if (id === "SharedVolume") {
              newContainers.sharedVolume.status = printStatus(delta.UpdateStatus);
            }
            if (id === "MM proxy") {
              newContainers.mmProxy.status = printStatus(delta.UpdateStatus);
            }
            if (id === "Loki") {
              newContainers.loki.status = printStatus(delta.UpdateStatus);
            }
            if (id === "Grafana") {
              newContainers.grafana.status = printStatus(delta.UpdateStatus);
            }
            if (id === "Xmrig") {
              newContainers.xmrig.status = printStatus(delta.UpdateStatus);
            }
            setContainers(newContainers);

          }
          // stats records
          if (delta.StatsRecord) {
            let newContainers: any = {
              ...containers
            };
            if (id === "Tor") {
              //            console.log(delta.StatsRecord);
              if (delta.StatsRecord.timestamp !== newContainers.tor.stats?.timestamp) {
                let last_cpu = newContainers.tor.stats?.cpu_usage;
                let last_system_cpu = newContainers.tor.stats?.system_cpu_usage;
                newContainers.tor.stats = delta.StatsRecord;
                newContainers.tor.stats.cpu = (delta.StatsRecord.cpu_usage - last_cpu) / (delta.StatsRecord.system_cpu_usage - last_system_cpu) * 100;
              }
            }
            if (id === "Base Node") {
              if (delta.StatsRecord.timestamp !== newContainers.baseNode.stats?.timestamp) {
                let last_cpu = newContainers.baseNode.stats?.cpu_usage;
                let last_system_cpu = newContainers.baseNode.stats?.system_cpu_usage;
                newContainers.baseNode.stats = delta.StatsRecord;
                newContainers.baseNode.stats.cpu = (delta.StatsRecord.cpu_usage - last_cpu) / (delta.StatsRecord.system_cpu_usage - last_system_cpu) * 100;
              }
            }
            if (id === "Sha3Miner") {
              if (delta.StatsRecord.timestamp !== newContainers.sha3Miner.stats?.timestamp) {
                let last_cpu = newContainers.sha3Miner.stats?.cpu_usage;
                let last_system_cpu = newContainers.sha3Miner.stats?.system_cpu_usage;
                newContainers.sha3Miner.stats = delta.StatsRecord;
                newContainers.sha3Miner.stats.cpu = (delta.StatsRecord.cpu_usage - last_cpu) / (delta.StatsRecord.system_cpu_usage - last_system_cpu) * 100;
              }
            }
            if (id === "SharedVolume") {
              if (delta.StatsRecord.timestamp !== newContainers.sharedVolume.stats?.timestamp) {
                let last_cpu = newContainers.sharedVolume.stats?.cpu_usage;
                let last_system_cpu = newContainers.sharedVolume.stats?.system_cpu_usage;
                newContainers.sharedVolume.stats = delta.StatsRecord;
                newContainers.sharedVolume.stats.cpu = (delta.StatsRecord.cpu_usage - last_cpu) / (delta.StatsRecord.system_cpu_usage - last_system_cpu) * 100;
              }
            }
            if (id === "MM proxy") {
              if (delta.StatsRecord.timestamp !== newContainers.mmProxy.stats?.timestamp) {
                let last_cpu = newContainers.mmProxy.stats?.cpu_usage;
                let last_system_cpu = newContainers.mmProxy.stats?.system_cpu_usage;
                newContainers.mmProxy.stats = delta.StatsRecord;
                newContainers.mmProxy.stats.cpu = (delta.StatsRecord.cpu_usage - last_cpu) / (delta.StatsRecord.system_cpu_usage - last_system_cpu) * 100;
              }
            }
            if (id === "Loki") {
              if (delta.StatsRecord.timestamp !== newContainers.loki.stats?.timestamp) {
                let last_cpu = newContainers.loki.stats?.cpu_usage;
                let last_system_cpu = newContainers.loki.stats?.system_cpu_usage;
                newContainers.loki.stats = delta.StatsRecord;
                newContainers.loki.stats.cpu = (delta.StatsRecord.cpu_usage - last_cpu) / (delta.StatsRecord.system_cpu_usage - last_system_cpu) * 100;
              }
            }
            if (id === "Grafana") {
              if (delta.StatsRecord.timestamp !== newContainers.grafana.stats?.timestamp) {
                let last_cpu = newContainers.grafana.stats?.cpu_usage;
                let last_system_cpu = newContainers.grafana.stats?.system_cpu_usage;
                newContainers.grafana.stats = delta.StatsRecord;
                newContainers.grafana.stats.cpu = (delta.StatsRecord.cpu_usage - last_cpu) / (delta.StatsRecord.system_cpu_usage - last_system_cpu) * 100;
              }
            }
            if (id === "Xmrig") {
              if (delta.StatsRecord.timestamp !== newContainers.xmrig.stats?.timestamp) {
                let last_cpu = newContainers.xmrig.stats?.cpu_usage;
                let last_system_cpu = newContainers.xmrig.stats?.system_cpu_usage;
                newContainers.xmrig.stats = delta.StatsRecord;
                newContainers.xmrig.stats.cpu = (delta.StatsRecord.cpu_usage - last_cpu) / (delta.StatsRecord.system_cpu_usage - last_system_cpu) * 100;
              }
            }
            setContainers(newContainers);
          }
          if (!delta.UpdateStatus && !delta.StatsRecord && !delta.LogRecord) {
            // No need for log records at this point
            console.log("Unknown delta: " + JSON.stringify(delta));
          }
        }
      }
    }))();

    return () => {
      (async () => (await unlisten)())();
    }
  });

  async function toggleMining() {
    if (isChangingMining) {
      return
    }
    setIsChangingMining(true);
    if (isMining) {
      stopMining();
    }
    else {
      startMining();
    }
  }

  async function startMining() {
    let state: any = appState;
    let stateSession = { ...state?.config?.session };
    stateSession.merge_layer_active = mergeMiningEnabled;
    stateSession.sha3x_layer_active = shaMiningEnabled;
    emit("tari://actions", { "Action": { type: "ChangeSession", payload: stateSession } });
  }

  async function stopMining() {
    let state: any = appState;
    let stateSession = { ...state?.config?.session };
    stateSession.merge_layer_active = false;
    stateSession.sha3x_layer_active = false;
    emit("tari://actions", { "Action": { type: "ChangeSession", payload: stateSession } });
  }

  async function toggleMergeMiningEnabled(event: React.ChangeEvent<HTMLInputElement>) {
    setMergeMiningEnabled(event.target.checked);
  }

  async function toggleShaMiningEnabled(event: React.ChangeEvent<HTMLInputElement>) {
    setShaMiningEnabled(event.target.checked);
  }

  async function handleDockerClose() {
    setOpenDockerWarning(false);
    await exit(1);
  };


  function printStatus(status: any) {
    if (status === undefined) {
      return "..."
    }
    // Some clever developer thought it was a good idea sometimes to return a string and sometimes an object
    if (status.hasOwnProperty("Progress")) {
      return status.Progress.stage;
    }

    if (status.hasOwnProperty("Failed")) {
      setOpenDockerWarning(true);
      return "Failed:" + status.Failed;
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
      status: printStatus(container.status)
    }
  }

  async function openDockerInstall(evt: any) {
    evt.preventDefault();
    open("https://docs.docker.com/engine/install/");
  }


  const col1 = 6;
  const col2 = 3;
  const col3 = 1;
  // const col4 = 3;
  // const col5 = 1;
  // const col6 = 1;

  // let state: any = appState;
  //  let containers: any = state.containers;
  // console.log(containers);

  return (
    <>
      <ThemeProvider theme={darkTheme}>
        <CssBaseline />
        <Grid container spacing={0} className="main-bg">
          <Container >
            <Grid container spacing={3} pt={4}>
              <Grid item xs={12} md={12} lg={12}>
                <Logo />
              </Grid>
            </Grid>
            <Grid container spacing={6} style={{
              paddingTop: theme.spacing(4),
              paddingBottom: theme.spacing(6),
            }}>
              <Grid item xs={12} md={12} lg={12}>
                {/* <GradientPaper> */}

                <Grid container spacing={3}>
                  <Grid item xs={12} md={12} lg={12} className="center-container">
                    {isChangingMining ? <CircularProgress color="inherit" style={{
                      width: "60px",
                      height: "60px",
                      margin: "32px"
                    }} /> :
                      <button id="bigOlButton" className="bob-button" onClick={() => toggleMining()}>
                        {/* <Icon>play_circle</Icon> */}

                        {isMining ? <PauseIcon /> : <PlayArrowIcon fontSize="large" />}

                      </button>
                    }
                    <Typography pt={2} variant="body2">
                      {isChangingMining ?
                        "Busy ..." : isMining ? "Pause Mining" : "Start Mining"}
                    </Typography>
                  </Grid>

                </Grid>
              </Grid>
              <Grid item xs={12} md={12} lg={12}>
                <Grid container spacing={3}>
                  <Grid item xs={3} md={3} lg={3} > </Grid>
                  <Grid item xs={6} md={6} lg={6} pt={20} >
                    <GradientPaper >

                      <Grid container spacing={3}>
                        <Grid item xs={col1} md={col1} lg={col1}>
                          <Typography variant="body2">Algorithm</Typography>
                        </Grid>
                        <Grid item xs={col2} md={col2} lg={col2}>
                          <Typography variant="body2">Status</Typography>
                        </Grid>
                        <Grid item xs={col3} md={col3} lg={col3}>
                          <Typography variant="body2">Enabled</Typography>
                        </Grid>
                      </Grid>
                      <Grid container spacing={3}>
                        <Grid item xs={12} md={12} lg={12} >
                          <Divider color={theme.palette.background.paper} />
                        </Grid>
                        <Grid item xs={col1} md={col1} lg={col1}>
                          <TypographyData > Merge Mining with Monero</TypographyData>
                        </Grid>
                        <Grid item xs={col2} md={col2} lg={col2}>
                          <TypographyData >{containers ? containers.xmrig?.status : "..."}</TypographyData>
                        </Grid>
                        <Grid item xs={col3} md={col3} lg={col3}>
                          <Switch checked={mergeMiningEnabled} onChange={toggleMergeMiningEnabled} />
                        </Grid>
                        <Grid item xs={12} md={12} lg={12} >
                          <Divider color={theme.palette.background.paper} />
                        </Grid>
                        <Grid item xs={col1} md={col1} lg={col1}>
                          <TypographyData >SHA3</TypographyData>
                        </Grid>
                        <Grid item xs={col2} md={col2} lg={col2}>
                          <TypographyData >{containers ? containers?.sha3Miner?.status : "..."}</TypographyData>
                        </Grid>
                        <Grid item xs={col3} md={col3} lg={col3}>
                          <Switch checked={shaMiningEnabled} onChange={toggleShaMiningEnabled} />
                        </Grid>
                      </Grid>
                    </GradientPaper>

                  </Grid>
                  <Grid item xs={3} md={3} lg={3} > </Grid>
                </Grid>
                {/* </GradientPaper> */}
              </Grid>
            </Grid>
          </Container >
          {/* </ThemeProvider >

      <ThemeProvider theme={darkTheme}> */}
          <Grid item xs={12} md={12} lg={12}>
            <Grid container spacing={3}>
              <Grid item xs={3} md={3} lg={3} > </Grid>
              <Grid item xs={6} md={6} lg={6} pt={20} >
                {/* <GradientPaper > */}
                <Grid container spacing={3}>
                  <Grid item xs={12} md={12} lg={12} >
                    <Typography variant="h4" style={{ textAlign: "center" }}>System Information</Typography>
                  </Grid>
                  <Grid item xs={col1} md={col1} lg={col1}>
                    <Typography variant="body2">Container</Typography>
                  </Grid>
                  <Grid item xs={col2} md={col2} lg={col2}>
                    <Typography variant="body2">Status</Typography>
                  </Grid>
                  <Grid item xs={col2} md={col3} lg={col3}>
                    <Typography variant="body2">CPU</Typography>
                  </Grid>
                </Grid>
                <Grid container spacing={3}>
                  <Grid item xs={12} md={12} lg={12} >
                    <Divider color={theme.palette.background.paper} />
                  </Grid>
                  {/* Tor */}
                  <Grid item xs={col1} md={col1} lg={col1}>
                    <TypographyData >tor</TypographyData>
                  </Grid>
                  <Grid item xs={col2} md={col2} lg={col2}>
                    <TypographyData >{containers ? containers.tor?.status : "..."}</TypographyData>
                  </Grid>
                  <Grid item xs={col3} md={col3} lg={col3}>
                    <TypographyData >{containers ? containers.tor?.stats?.cpu?.toFixed(2) : "..."}</TypographyData>
                  </Grid>
                  <Grid item xs={12} md={12} lg={12} >
                    <Divider color={theme.palette.background.paper} />
                  </Grid>

                  {/* Base Node */}
                  <Grid item xs={col1} md={col1} lg={col1}>
                    <TypographyData >minotari node</TypographyData>
                  </Grid>
                  <Grid item xs={col2} md={col2} lg={col2}>
                    <TypographyData >{containers ? containers.baseNode?.status : "..."}</TypographyData>
                  </Grid>
                  <Grid item xs={col3} md={col3} lg={col3}>
                    <TypographyData >{containers ? containers.baseNode?.stats?.cpu?.toFixed(2) : "..."}</TypographyData>
                  </Grid>
                  <Grid item xs={12} md={12} lg={12} >
                    <Divider color={theme.palette.background.paper} />
                  </Grid>
                  {/* Sha miner */}
                  <Grid item xs={col1} md={col1} lg={col1}>
                    <TypographyData >sha3 miner</TypographyData>
                  </Grid>
                  <Grid item xs={col2} md={col2} lg={col2}>
                    <TypographyData >{containers ? containers.sha3Miner?.status : "..."}</TypographyData>
                  </Grid>
                  <Grid item xs={col3} md={col3} lg={col3}>
                    <TypographyData >{containers ? containers.sha3Miner?.stats?.cpu?.toFixed(2) : "..."}</TypographyData>
                  </Grid>
                  <Grid item xs={12} md={12} lg={12} >
                    <Divider color={theme.palette.background.paper} />
                  </Grid>

                  {/* SharedVolume */}
                  <Grid item xs={col1} md={col1} lg={col1}>
                    <TypographyData >shared volume</TypographyData>
                  </Grid>
                  <Grid item xs={col2} md={col2} lg={col2}>
                    <TypographyData >{containers ? containers.sharedVolume?.status : "..."}</TypographyData>
                  </Grid>
                  <Grid item xs={col3} md={col3} lg={col3}>
                    <TypographyData >{containers ? containers.sharedVolume?.stats?.cpu?.toFixed(2) : "..."}</TypographyData>
                  </Grid>
                  <Grid item xs={12} md={12} lg={12} >
                    <Divider color={theme.palette.background.paper} />
                  </Grid>
                  {/* MM Proxy */}
                  <Grid item xs={col1} md={col1} lg={col1}>
                    <TypographyData >merge mining proxy</TypographyData>
                  </Grid>
                  <Grid item xs={col2} md={col2} lg={col2}>
                    <TypographyData >{containers ? containers.mmProxy?.status : "..."}</TypographyData>
                  </Grid>
                  <Grid item xs={col3} md={col3} lg={col3}>
                    <TypographyData >{containers ? containers.mmProxy?.stats?.cpu?.toFixed(2) : "..."}</TypographyData>
                  </Grid>
                  <Grid item xs={12} md={12} lg={12} >
                    <Divider color={theme.palette.background.paper} />
                  </Grid>
                  {/* Loki */}
                  <Grid item xs={col1} md={col1} lg={col1}>
                    <TypographyData >loki</TypographyData>
                  </Grid>
                  <Grid item xs={col2} md={col2} lg={col2}>
                    <TypographyData >{containers?.loki?.status}</TypographyData>
                  </Grid>
                  <Grid item xs={col3} md={col3} lg={col3}>
                    <TypographyData >{containers ? containers.loki?.stats?.cpu?.toFixed(2) : "..."}</TypographyData>
                  </Grid>
                  <Grid item xs={12} md={12} lg={12} >
                    <Divider color={theme.palette.background.paper} />
                  </Grid>
                  {/* Grafana */}
                  <Grid item xs={col1} md={col1} lg={col1}>
                    <TypographyData >grafana</TypographyData>
                  </Grid>
                  <Grid item xs={col2} md={col2} lg={col2}>
                    <TypographyData >{containers?.grafana?.status}</TypographyData>
                  </Grid>
                  <Grid item xs={col3} md={col3} lg={col3}>
                    <TypographyData >{containers ? containers.grafana?.stats?.cpu?.toFixed(2) : "..."}</TypographyData>
                  </Grid>
                  <Grid item xs={12} md={12} lg={12} >
                    <Divider color={theme.palette.background.paper} />
                  </Grid>
                  {/* Xmrig */}
                  <Grid item xs={col1} md={col1} lg={col1}>
                    <TypographyData >xmrig</TypographyData>
                  </Grid>
                  <Grid item xs={col2} md={col2} lg={col2}>
                    <TypographyData >{containers?.xmrig?.status}</TypographyData>
                  </Grid>
                  <Grid item xs={col3} md={col3} lg={col3}>
                    <TypographyData >{containers ? containers.xmrig?.stats?.cpu?.toFixed(2) : "..."}</TypographyData>
                  </Grid>
                  <Grid item xs={12} md={12} lg={12} >
                    <Divider color={theme.palette.background.paper} />
                  </Grid>
                  <Grid item xs={12} md={12} lg={12} >
                    <TypographyData>Sync Status: Height: {appState?.node?.chain_height} Status: {appState?.node?.sync_status}</TypographyData>
                  </Grid>

                  {/* </GradientPaper> */}
                </Grid>
              </Grid>
            </Grid>
          </Grid >
        </Grid>
      </ThemeProvider >
      <Dialog
        open={openDockerWarning}
        onClose={handleDockerClose}
        aria-labelledby="alert-dialog-title"
        aria-describedby="alert-dialog-description"
      >
        <DialogTitle id="alert-dialog-title">
          Docker is not running
        </DialogTitle>
        <DialogContent>
          <DialogContentText id="alert-dialog-description">
            Tari Launchpad requires Docker to be running. Please start Docker and try again.
            If you don't have Docker installed, you can download it from <a onClick={(evt) => openDockerInstall(evt)} href="#">here</a>.
          </DialogContentText>
        </DialogContent>
        <DialogActions>
          <Button onClick={handleDockerClose}>Exit</Button>
        </DialogActions>
      </Dialog>
    </>
  );
}

export default App;
