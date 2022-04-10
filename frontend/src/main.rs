mod components;
use crate::components::app::App;
mod live_cursor;
mod utils;

fn main() {
    wasm_logger::init(wasm_logger::Config::default());
    yew::start_app::<App>();
}
