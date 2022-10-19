use frontend::{App, Pod};

fn main() {
    wasm_logger::init(wasm_logger::Config::new(log::Level::Trace));
    wasm_bindgen_futures::spawn_local(entrypoint());
}

async fn entrypoint() {
    frontend::bus::connect_to_bus().await;
    yew::start_app::<Pod<App>>();
}
