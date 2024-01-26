import { useState } from "react";
// import reactLogo from "../src/assets/react.svg";

// import { invoke } from "@tauri-apps/api/tauri";
import "./App.css";
import { emit, listen } from '@tauri-apps/api/event'

function App() {
  const [greetMsg, _setGreetMsg] = useState("");
  const [_name, setName] = useState("");
  const [appState, setAppState] = useState({});
  const [logs, setLogs] = useState("");

  async function greet() {
    // Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
    //setGreetMsg(await invoke("greet", { name }));
    emit("tari://actions", { "Action": { type: "Connect" } });
  }

  async function mergeMine() {
    let state: any = appState;
    let stateSession = { ...state?.config?.session };
    stateSession.merge_layer_active = stateSession.merge_layer_active ? false : true;
    emit("tari://actions", { "Action": { type: "ChangeSession", payload: stateSession } });
  }

  async function shaMine() {
    let state: any = appState;
    let stateSession = { ...state?.config?.session };
    stateSession.sha3x_layer_active = stateSession.sha3x_layer_active ? false : true;
    emit("tari://actions", { "Action": { type: "ChangeSession", payload: stateSession } });
  }


  listen("tari://reactions", (event) => {

    let payload: any = event.payload;
    console.log(payload);
    if (payload?.State !== undefined) {
      setAppState(payload?.State);
    }
    if (payload?.Delta !== undefined) {
      console.log("Don't know what todo with delta");
      setLogs(logs + "\n" + JSON.stringify(payload?.Delta));
    }
  })

  return (
    <div className="container">
      <form
        className="row"
        onSubmit={(e) => {
          e.preventDefault();
          greet();
        }}
      >
        <input
          id="greet-input"
          onChange={(e) => setName(e.currentTarget.value)}
          placeholder="Enter a name..."
        />
        <button type="submit">Connect</button>
      </form>

      <div>
        <button id="bigOlButton" onClick={() => mergeMine()}>Merge Mine You Fools!</button>
        <button id="bigOlButton" onClick={() => shaMine()}>Sha Mine You Fools!</button>
      </div>
      <div>
        <textarea id="bigOlTextArea" value={logs} readOnly={true} rows={10} cols={100}></textarea>
      </div>

      <p>{greetMsg}</p>
    </div>
  );
}

export default App;
