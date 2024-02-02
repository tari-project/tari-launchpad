import { useState } from "react";
// import reactLogo from "../src/assets/react.svg";

// import { invoke } from "@tauri-apps/api/tauri";
import "./App.css";
import { emit, listen } from '@tauri-apps/api/event'
import { Container, CssBaseline, Grid, ThemeProvider } from "@mui/material";
import { createTheme, useTheme } from '@mui/material/styles';
import { componentSettings, dark } from './theme/tokens'
import { GradientPaper } from './components/StyledComponents';
import PlayArrowIcon from '@mui/icons-material/PlayArrow';
import PauseIcon from '@mui/icons-material/Pause';

function App() {
  const [_greetMsg, _setGreetMsg] = useState("");
  const [_name, _setName] = useState("");
  const [appState, setAppState] = useState({});
  const [isMining, setIsMining] = useState(false);
  const [logs, setLogs] = useState("");

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

  return (
    <>
      <ThemeProvider theme={darkTheme}>
        <CssBaseline />
        <Grid container spacing={0} className="main-bg">
          <Container >
            <Grid container spacing={3} style={{
              paddingTop: theme.spacing(4),
              paddingBottom: theme.spacing(6),
            }}>
              <Grid item xs={12} md={12} lg={12}>
                <GradientPaper>

                  <Grid container spacing={3}>
                    <Grid item xs={12} md={12} lg={12} className="center-container">
                      <button id="bigOlButton" className="bob-button" onClick={() => mergeMine()}>
                        {/* <Icon>play_circle</Icon> */}
                        {isMining ? <PauseIcon /> : <PlayArrowIcon fontSize="large" />}

                      </button>
                    </Grid>

                  </Grid>

                </GradientPaper>
              </Grid>
            </Grid>

            {/* <div >
                <form
                  onSubmit={(e) => {
                    e.preventDefault();
                    greet();
                  }}
                >
                  <button type="submit">Connect</button>
                </form>

                <div>
                  <button id="bigOlButton" className="bob-button" onClick={() => mergeMine()}>
              {isMining ? <PauseIcon /> : <PlayArrowIcon fontSize="large" />}

            </button>
            <button id="bigOlButton" onClick={() => shaMine()}>Sha Mine You Fools!</button>
          </div>
          <div>
            <textarea id="bigOlTextArea" value={logs} readOnly={true} rows={10} cols={100}></textarea>
          </div>

        </div> */}


          </Container >

        </Grid >
      </ThemeProvider >
    </>
  );
}

export default App;
