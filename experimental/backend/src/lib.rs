// use anyhow::Error;
// use tari_launchpad_protocol::{ACTIONS, REACTIONS};
// use tari_sdm_launchpad::LaunchpadBus;
// use tauri::{App, Manager, Wry};
//
// pub fn bus_setup(app: &mut App<Wry>) -> Result<(), Box<dyn std::error::Error>> {
// let handle = app.handle();
// let bus = LaunchpadBus::start()?;
//
// let in_tx = bus.incoming;
// let _id = app.listen_global(ACTIONS, move |event| {
// if let Some(payload) = event.payload() {
// let res = serde_json::from_str(payload);
// match res {
// Ok(incoming) => {
// log::trace!("Incoming event: {:?}", incoming);
// if let Err(err) = in_tx.send(incoming) {
// log::error!("Can't forward an incoming event: {:?}", err);
// }
// },
// Err(err) => {
// log::error!("Can't parse incoming event: {}", err);
// },
// }
// }
// });
//
// let mut out_rx = bus.outgoing;
// tauri::async_runtime::spawn(async move {
// while let Some(event) = out_rx.recv().await {
// handle.emit_all(REACTIONS, event)?;
// }
// Ok::<(), Error>(())
// });
//
// Ok(())
// }
