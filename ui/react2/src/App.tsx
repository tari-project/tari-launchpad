import { useEffect, useState } from "react";
// import reactLogo from "../src/assets/react.svg";

// import { invoke } from "@tauri-apps/api/tauri";
import "./App.css";
import { emit, listen } from '@tauri-apps/api/event'
import { CircularProgress, Container, CssBaseline, Divider, Grid, Switch, ThemeProvider, Typography } from "@mui/material";
import { createTheme, useTheme } from '@mui/material/styles';
import { componentSettings, dark } from './theme/tokens'
import { GradientPaper, TypographyData } from './components/StyledComponents';
import PlayArrowIcon from '@mui/icons-material/PlayArrow';
import PauseIcon from '@mui/icons-material/Pause';
import Logo from "./assets/Logo";

function App() {
  const [appState, setAppState] = useState({});
  const [isMining, setIsMining] = useState(false);
  // const [logs, setLogs] = useState("");
  const [shaMiningEnabled, setShaMiningEnabled] = useState(true);
  const [mergeMiningEnabled, setMergeMiningEnabled] = useState(true);
  const [isChangingMining, setIsChangingMining] = useState(false);

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
  useEffect(() => {
    emit("tari://actions", { "Action": { type: "Connect" } });
  }, []);

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


  // async function shaMine() {
  //   let state: any = appState;
  //   let stateSession = { ...state?.config?.session };
  //   stateSession.sha3x_layer_active = stateSession.sha3x_layer_active ? false : true;
  //   emit("tari://actions", { "Action": { type: "ChangeSession", payload: stateSession } });
  // }


  listen("tari://reactions", (event) => {

    let payload: any = event.payload;
    console.log(payload);
    if (payload?.State !== undefined) {
      setAppState(payload?.State);
      setIsMining(payload?.State?.config?.session?.merge_layer_active || payload?.State?.config?.session?.sha3x_layer_active);
      setIsChangingMining(false);
    }
    if (payload?.Delta !== undefined) {
      // console.log("Don't know what todo with delta");
      // alert(logs);
      // setLogs(logs + "\n" + JSON.stringify(payload?.Delta));
    }
  })

  const col1 = 6;
  const col2 = 3;
  const col3 = 1;
  // const col4 = 3;
  // const col5 = 1;
  // const col6 = 1;

  let state: any = appState;
  let containers: any = state.containers;

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
                          <TypographyData >{isMining ? "Mining" : "Idle"}</TypographyData>
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
                          <TypographyData >{isMining ? "Mining" : "Idle"}</TypographyData>
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
                    <TypographyData >{containers ? containers["Tor"]?.status : "..."}</TypographyData>
                  </Grid>
                  <Grid item xs={12} md={12} lg={12} >
                    <Divider color={theme.palette.background.paper} />
                  </Grid>

                  {/* Base Node */}
                  <Grid item xs={col1} md={col1} lg={col1}>
                    <TypographyData >minotari node</TypographyData>
                  </Grid>
                  <Grid item xs={col2} md={col2} lg={col2}>
                    <TypographyData >{containers ? containers["Base Node"]?.status : "..."}</TypographyData>
                  </Grid>
                  <Grid item xs={12} md={12} lg={12} >
                    <Divider color={theme.palette.background.paper} />
                  </Grid>
                  {/* Sha miner */}
                  <Grid item xs={col1} md={col1} lg={col1}>
                    <TypographyData >sha3 miner</TypographyData>
                  </Grid>
                  <Grid item xs={col2} md={col2} lg={col2}>
                    <TypographyData >{containers ? containers["Sha3Miner"]?.status : "..."}</TypographyData>
                  </Grid>
                  <Grid item xs={12} md={12} lg={12} >
                    <Divider color={theme.palette.background.paper} />
                  </Grid>

                  {/* SharedVolume */}
                  <Grid item xs={col1} md={col1} lg={col1}>
                    <TypographyData >shard volume</TypographyData>
                  </Grid>
                  <Grid item xs={col2} md={col2} lg={col2}>
                    <TypographyData >{containers ? containers["SharedVolume"]?.status : "..."}</TypographyData>
                  </Grid>
                  <Grid item xs={12} md={12} lg={12} >
                    <Divider color={theme.palette.background.paper} />
                  </Grid>
                  {/* MM Proxy */}
                  <Grid item xs={col1} md={col1} lg={col1}>
                    <TypographyData >merge mining proxy</TypographyData>
                  </Grid>
                  <Grid item xs={col2} md={col2} lg={col2}>
                    <TypographyData >{containers ? containers["MM proxy"]?.status : "..."}</TypographyData>
                  </Grid>
                  <Grid item xs={12} md={12} lg={12} >
                    <Divider color={theme.palette.background.paper} />
                  </Grid>
                  {/* Loki */}
                  <Grid item xs={col1} md={col1} lg={col1}>
                    <TypographyData >loki</TypographyData>
                  </Grid>
                  <Grid item xs={col2} md={col2} lg={col2}>
                    <TypographyData >{containers?.Loki?.status}</TypographyData>
                  </Grid>
                  <Grid item xs={12} md={12} lg={12} >
                    <Divider color={theme.palette.background.paper} />
                  </Grid>
                  {/* Grafana */}
                  <Grid item xs={col1} md={col1} lg={col1}>
                    <TypographyData >grafana</TypographyData>
                  </Grid>
                  <Grid item xs={col2} md={col2} lg={col2}>
                    <TypographyData >{containers?.Grafana?.status}</TypographyData>
                  </Grid>
                  <Grid item xs={12} md={12} lg={12} >
                    <Divider color={theme.palette.background.paper} />
                  </Grid>
                  {/* Xmrig */}
                  <Grid item xs={col1} md={col1} lg={col1}>
                    <TypographyData >xmrig</TypographyData>
                  </Grid>
                  <Grid item xs={col2} md={col2} lg={col2}>
                    <TypographyData >{containers?.Xmrig?.status}</TypographyData>
                  </Grid>
                  <Grid item xs={12} md={12} lg={12} >
                    <Divider color={theme.palette.background.paper} />
                  </Grid>
                  <Grid item xs={12} md={12} lg={12} >
                    <TypographyData>Sync Status: Height: {state?.node?.chain_height} Status: {state?.node?.sync_status}</TypographyData>
                  </Grid>

                  {/* </GradientPaper> */}
                </Grid>
              </Grid>
            </Grid>
          </Grid >
        </Grid>
      </ThemeProvider >
    </>
  );
}

export default App;
