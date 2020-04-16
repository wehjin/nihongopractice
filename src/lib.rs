#![recursion_limit = "512"]
#[macro_use]
extern crate log;
extern crate percent_encoding;
extern crate verb;
extern crate web_sys;

use wasm_bindgen::prelude::*;

mod utils;
mod app;
mod view;
mod recognition;
mod data;
mod idle;

#[wasm_bindgen]
extern {
	fn alert(s: &str);
}

#[wasm_bindgen]
pub fn run_app() -> Result<(), JsValue> {
	web_logger::init();
	utils::set_panic_hook();
	info!("starting up");
	yew::initialize();
	let window: web_sys::Window = web_sys::window().unwrap();
	let document: web_sys::Document = window.document().unwrap();
	let element: web_sys::Element = document.get_element_by_id("yew-entry").unwrap();
	yew::App::<app::Model>::new().mount(element);
	yew::run_loop();
	Ok(())
}

