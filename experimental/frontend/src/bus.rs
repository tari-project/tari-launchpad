use anyhow::{anyhow, Error};
use serde::Deserialize;
use tari_launchpad_protocol::{
    launchpad::{Action, LaunchpadAction, Reaction},
    ACTIONS,
    REACTIONS,
};
use wasm_bindgen::prelude::{wasm_bindgen, Closure, JsValue};

use crate::states::remote_state::REMOTE_STATE;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = ["__TAURI__", "event"])]
    async fn listen(event: &str, f: &Closure<dyn FnMut(JsValue)>) -> JsValue;
    #[wasm_bindgen(js_namespace = ["__TAURI__", "event"])]
    fn emit(event: &str, object: JsValue) -> JsValue;
}

pub fn request(incoming: Action) {
    log::debug!("Sending: {:?}", incoming);
    if let Err(err) = request_impl(incoming) {
        log::error!("Can't serialize a request: {}", err);
    }
}

fn request_impl(incoming: Action) -> Result<(), Error> {
    let value = serde_json::to_string(&incoming)?;
    let js_value = JsValue::from_str(&value);
    emit(ACTIONS, js_value);
    Ok(())
}

pub async fn connect_to_bus() {
    let closure = Closure::new(response);
    let _unlisten_promise = listen(REACTIONS, &closure).await;
    closure.forget();

    // Start transferring
    let msg = Action::Action(LaunchpadAction::Connect);
    request(msg);
}

#[derive(Deserialize)]
struct Event {
    payload: Reaction,
}

fn response(value: JsValue) {
    log::debug!("Receiving: {:?}", value);
    if let Err(err) = response_impl(value) {
        log::error!("Can't deserialize a response: {}", err);
    }
}

fn response_impl(value: JsValue) -> Result<(), Error> {
    let event: Event =
        serde_wasm_bindgen::from_value(value).map_err(|err| anyhow!("Can't deserialize event: {}", err))?;
    REMOTE_STATE.update(event.payload);
    Ok(())
}
