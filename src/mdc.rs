use yew::{ComponentLink, Html, html};

use crate::app::{Model, Msg};

pub mod button {
	use yew::{Callback, Html, html};

	pub fn flat(label: &str, callback: Callback<web_sys::MouseEvent>) -> Html {
		html! {
	        <button class="mdl-button mdl-js-button mdl-button--primary" onclick=callback>
	            { label }
	        </button>
		}
	}
}

pub fn flat_button(label: &str, msg: Msg, link: &ComponentLink<Model>) -> Html {
	button::flat(label, link.callback(move |_| msg))
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
