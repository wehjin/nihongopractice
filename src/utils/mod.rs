use wasm_bindgen::{JsCast, JsValue};
use wasm_bindgen::closure::Closure;
use wasm_bindgen_futures::JsFuture;

pub fn set_panic_hook() {
	#[cfg(feature = "console_error_panic_hook")]
		console_error_panic_hook::set_once();
}

pub fn play_audio(audio: &web_sys::HtmlAudioElement) {
	let promise = audio.play().expect("audio should play");
	let future = wasm_bindgen_futures::JsFuture::from(promise);
	wasm_bindgen_futures::spawn_local(async move {
		play_audio_from_js(future).await.expect("audio future should play");
	})
}

async fn play_audio_from_js(future: JsFuture) -> Result<JsValue, JsValue> {
	let result = future.await?;
	Ok(result)
}

fn window() -> web_sys::Window {
	web_sys::window().expect("no global `window` exists")
}

pub fn request_animation_frame(f: impl FnMut() + 'static) {
	let f = Closure::wrap(Box::new(f) as Box<dyn FnMut()>);
	window()
		.request_animation_frame(f.as_ref().unchecked_ref())
		.expect("should register `requestAnimationFrame` OK");
	f.forget();
}
