use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;

pub fn window() -> web_sys::Window {
    web_sys::window().expect("no global `window` exists")
}

pub fn set_interval(f: &Closure<dyn FnMut()>, interval: i32) {
    window()
        .set_interval_with_callback_and_timeout_and_arguments_0(
            f.as_ref().unchecked_ref(),
            interval,
        )
        .expect("should register `setInterval` OK");
}

pub fn performance() -> web_sys::Performance {
    window()
        .performance()
        .expect("window.performance should be available")
}
