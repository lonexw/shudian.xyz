mod api;
mod app;
mod route;
mod types;
mod pages;
mod components;

use crate::app::App;

fn main() {
    wasm_logger::init(wasm_logger::Config::new(log::Level::Trace));
    console_error_panic_hook::set_once();

    yew::start_app::<App>();
}
