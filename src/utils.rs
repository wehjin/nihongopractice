use percent_encoding::{NON_ALPHANUMERIC, utf8_percent_encode};
use wasm_bindgen::JsValue;
use wasm_bindgen_futures::JsFuture;

pub fn set_panic_hook() {
	#[cfg(feature = "console_error_panic_hook")]
		console_error_panic_hook::set_once();
}

pub fn audio_url(search: &str) -> String {
	let textlen = search.chars().count();
	let encoded_search = utf8_percent_encode(search, NON_ALPHANUMERIC).to_string();
	format!("https://translate.google.com/translate_tts?ie=UTF-8&client=tw-ob&q={}&tl=ja&total=1&index=0&textlen={}", encoded_search, textlen)
}

pub fn now() -> f64 {
	let window = web_sys::window().expect("should have a window in this context");
	let performance = window
		.performance()
		.expect("performance should be available");
	performance.now()
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

pub fn drop_last_char(s: &str) -> String {
	let last_len = last_char(s).len();
	s[..s.len() - last_len].to_string()
}

pub fn last_char(s: &str) -> String {
	s.chars().last().unwrap().to_string()
}

pub mod mdc {
	use yew::{ComponentLink, Html, html};

	use crate::app::{Model, Msg};

	pub fn flat_button(label: &str, msg: Msg, link: &ComponentLink<Model>) -> Html {
		html! {
        <button
            class="mdl-button mdl-js-button mdl-button--primary mdl-js-ripple-effect"
            onclick=link.callback(move |_| msg)>
            { label }
        </button>
		}
	}

	pub fn center_flat_button(label: &str, msg: Msg, link: &ComponentLink<Model>) -> Html {
		html! {
        <button
            class="mdl-button mdl-js-button mdl-button--primary mdl-js-ripple-effect"
            style="width:100%"
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
			<div class="mdl-layout mdl-js-layout">
			  <header class="mdl-layout__header  mdl-layout__header--scroll">
			    <div class="mdl-layout__header-row">
			      <span class="mdl-layout-title">{title}</span>
			      <div class="mdl-layout-spacer"></div>
			      <nav class="mdl-navigation">
			        { if let Some(ref label) = action { nav_link(label)} else { empty_div()}}
			      </nav>
			    </div>
			  </header>
			  <div class="mdl-layout__drawer">
			    <span class="mdl-layout-title">{title}</span>
			    <nav class="mdl-navigation">
			        { if let Some(ref label) = action { nav_link(label)} else { empty_div()}}
			    </nav>
			  </div>
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

	pub fn text_list_item(text: &str) -> Html {
		html! {
			<li class="mdl-list__item">
				{primary_span(text)}
			</li>
		}
	}

	pub fn primary_span(text: &str) -> Html {
		html! {
			<span class="mdl-list__item-primary-content">{text}</span>
		}
	}
}