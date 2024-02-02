import { useState } from "react";
// import reactLogo from "../src/assets/react.svg";

// import { invoke } from "@tauri-apps/api/tauri";
import "./App.css";
import { emit, listen } from '@tauri-apps/api/event'
import { Container, CssBaseline, Divider, Grid, ThemeProvider, Typography } from "@mui/material";
import { createTheme, useTheme } from '@mui/material/styles';
import { componentSettings, light, dark } from './theme/tokens'
import { GradientPaper, TypographyData } from './components/StyledComponents';
import PlayArrowIcon from '@mui/icons-material/PlayArrow';
import PauseIcon from '@mui/icons-material/Pause';
import Logo from "./assets/Logo";

function App() {
  const [_greetMsg, _setGreetMsg] = useState("");
  const [_name, _setName] = useState("");
  const [appState, setAppState] = useState({});
  const [isMining, setIsMining] = useState(false);
  const [logs, setLogs] = useState("");

  // const lightTheme = createTheme({
  //   ...light,
  //   ...componentSettings,
  // });

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

  async function mergeMine() {
    let state: any = appState;
    let stateSession = { ...state?.config?.session };
    stateSession.merge_layer_active = stateSession.merge_layer_active ? false : true;
    emit("tari://actions", { "Action": { type: "ChangeSession", payload: stateSession } });
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
    }
    if (payload?.Delta !== undefined) {
      console.log("Don't know what todo with delta");
      setLogs(logs + "\n" + JSON.stringify(payload?.Delta));
    }
  })

  const col1 = 8;
  const col2 = 3;
  // const col3 = 2;
  // const col4 = 3;
  // const col5 = 1;
  // const col6 = 1;

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
                    <button id="bigOlButton" className="bob-button" onClick={() => mergeMine()}>
                      {/* <Icon>play_circle</Icon> */}
                      {isMining ? <PauseIcon /> : <PlayArrowIcon fontSize="large" />}

                    </button>
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
                      </Grid>
                      <Grid container spacing={3}>
                        <Grid item xs={12} md={12} lg={12} >
                          <Divider color={theme.palette.background.paper} />
                        </Grid>
                        <Grid item xs={col1} md={col1} lg={col1}>
                          <TypographyData >Merge Mining with Monero</TypographyData>
                        </Grid>
                        <Grid item xs={col2} md={col2} lg={col2}>
                          <TypographyData >{isMining ? "Mining" : "Idle"}</TypographyData>
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
                <GradientPaper >

                  <Grid container spacing={3}>
                    <Grid item xs={col1} md={col1} lg={col1}>
                      <Typography variant="body2">Container</Typography>
                    </Grid>
                    <Grid item xs={col2} md={col2} lg={col2}>
                      <Typography variant="body2">CPU</Typography>
                    </Grid>
                  </Grid>
                  <Grid container spacing={3}>
                    <Grid item xs={12} md={12} lg={12} >
                      <Divider color={theme.palette.background.paper} />
                    </Grid>
                    <Grid item xs={col1} md={col1} lg={col1}>
                      <TypographyData >Tor</TypographyData>
                    </Grid>
                    <Grid item xs={col2} md={col2} lg={col2}>
                      <TypographyData >{isMining ? "Mining" : "Idle"}</TypographyData>
                    </Grid>
                    <Grid item xs={12} md={12} lg={12} >
                      <Divider color={theme.palette.background.paper} />
                    </Grid>
                  </Grid>
                </GradientPaper>
              </Grid>
            </Grid>
          </Grid>
        </Grid >
      </ThemeProvider>
    </>
  );
}

export default App;
