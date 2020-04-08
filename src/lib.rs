#![recursion_limit = "512"]

use wasm_bindgen::prelude::*;

mod utils;
mod app;
mod view;

#[wasm_bindgen]
extern {
	fn alert(s: &str);
}

#[wasm_bindgen]
pub fn run_app() -> Result<(), JsValue> {
	utils::set_panic_hook();
	web_logger::init();
	yew::start_app::<app::Model>();
	Ok(())
}

