use wasm_bindgen::JsValue;
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
