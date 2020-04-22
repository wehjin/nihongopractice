use wasm_bindgen::closure::Closure;
use wasm_bindgen::JsCast;

pub fn set_panic_hook() {
	#[cfg(feature = "console_error_panic_hook")]
		console_error_panic_hook::set_once();
}

pub mod audio {
	use wasm_bindgen::{JsCast, JsValue};
	use wasm_bindgen::closure::Closure;
	use wasm_bindgen_futures::JsFuture;
	use yew::NodeRef;

	pub fn play_segment(audio_ref: NodeRef, start: f64, end: f64) {
		pause(audio_ref.clone());
		if let Some(audio) = audio_ref.cast::<web_sys::HtmlAudioElement>() {
			audio.set_current_time(start);
			let closure_audio_ref = audio_ref.clone();
			let closure = Closure::wrap(Box::new(move |_: web_sys::Event| {
				let should_pause = should_pause(closure_audio_ref.clone(), end);
				if should_pause {
					pause(closure_audio_ref.clone());
				}
			}) as Box<dyn FnMut(_)>);
			audio.set_ontimeupdate(Some(closure.as_ref().unchecked_ref()));
			closure.forget();
			play(audio_ref);
		}
	}

	fn should_pause(audio_ref: NodeRef, end_time: f64) -> bool {
		if let Some(audio) = audio_ref.cast::<web_sys::HtmlAudioElement>() {
			audio.current_time() >= end_time
		} else {
			true
		}
	}

	fn pause(audio_ref: NodeRef) {
		if let Some(audio) = audio_ref.cast::<web_sys::HtmlAudioElement>() {
			audio.pause().expect("audio pause OK");
			audio.set_ontimeupdate(None)
		}
	}

	pub fn play(audio_ref: NodeRef) {
		if let Some(audio) = audio_ref.cast::<web_sys::HtmlAudioElement>() {
			let promise = audio.play().expect("audio should play");
			let future = wasm_bindgen_futures::JsFuture::from(promise);
			wasm_bindgen_futures::spawn_local(async move {
				play_from_js(future).await.expect("audio future should play");
			})
		}
	}

	async fn play_from_js(future: JsFuture) -> Result<JsValue, JsValue> {
		let result = future.await?;
		Ok(result)
	}
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
