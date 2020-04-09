#![recursion_limit = "512"]
#[macro_use]
extern crate log;
extern crate web_sys;

use wasm_bindgen::prelude::*;

mod utils;
mod app;
mod view;
mod verb;

#[wasm_bindgen]
extern {
	fn alert(s: &str);
}

#[wasm_bindgen]
pub fn run_app() -> Result<(), JsValue> {
	web_logger::init();
	utils::set_panic_hook();
	info!("starting up");
	yew::start_app::<app::Model>();
	Ok(())
}

