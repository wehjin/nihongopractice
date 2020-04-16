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

pub mod mdc {
	use yew::{ComponentLink, Html, html};

	use crate::app::{Model, Msg};

	pub fn flat_button(label: &str, msg: Msg, link: &ComponentLink<Model>) -> Html {
		html! {
        <button
            class="mdl-button mdl-js-button mdl-button--primary"
            onclick=link.callback(move |_| msg)>
            { label }
        </button>
		}
	}

	pub fn page<Ctx>(
		title: &str,
		action: Option<String>,
		link: &ComponentLink<Model>,
		ctx: &Ctx,
		content: impl Fn(&ComponentLink<Model>, &Ctx) -> Html,
	) -> Html {
		html! {
			<div class="mdl-layout mdl-js-layout mdl-layout--no-drawer-button mdl-layout--fixed-header">
			  <header class="mdl-layout__header">
			    <div class="mdl-layout__header-row">
			      <span class="mdl-layout-title">{title}</span>
			      <div class="mdl-layout-spacer"></div>
			      <nav class="mdl-navigation">
			        { if let Some(ref label) = action { nav_link(label)} else { empty_div()}}
			      </nav>
			    </div>
			  </header>
			  <main class="mdl-layout__content">{content(link, ctx)}</main>
			</div>
		}
	}

	pub fn nav_link(label: &str) -> Html {
		html! {
			<a class="mdl-navigation__link" href="">{label}</a>
		}
	}

	pub fn empty_div() -> Html {
		html! {}
	}
}