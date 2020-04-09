pub fn set_panic_hook() {
	#[cfg(feature = "console_error_panic_hook")]
		console_error_panic_hook::set_once();
}

pub fn audio_url(search: &str) -> String {
	format!("http://translate.google.com/translate_tts?ie=UTF-8&client=tw-ob&q={}&tl=ja", search)
}

pub mod mdc {
	use yew::{ComponentLink, Html, html};

	use crate::app::{Model, Msg};

	pub fn flat_button(label: &str, msg: Msg, link: &ComponentLink<Model>) -> Html {
		html! {
        <button class="mdl-button mdl-js-button mdl-button--primary" onclick=link.callback(move |_| msg)>{ label }</button>
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