use yew::{ComponentLink, Html, html};

use crate::app::{Model, Msg};

pub mod button {
	use yew::{Callback, Html, html};

	pub fn flat(label: &str, on_click: impl Fn() + 'static) -> Html {
		let on_click: Callback<web_sys::MouseEvent> = Callback::from(move |_| on_click());
		html! {
	        <button class="mdl-button mdl-js-button mdl-button--primary" onclick=on_click>
	            { label }
	        </button>
		}
	}
}

pub mod card {
	use yew::{Html, html};

	use super::*;

	pub fn grid(title: &str, message: &str, (label, action): (&str, impl Fn() + 'static)) -> Html {
		let button = button::flat(label, action);
		html! {
			<div class="mdl-cell mdl-cell--4-col">
				<div class=" mdl-card mdl-shadow--2dp tool-card">
					<div class="mdl-card__title mdl-color--primary-dark" style="color:#fff">
					    <h2 class="mdl-card__title-text">{ title }</h2>
					</div>
					<div class="mdl-card__title mdl-color--primary-dark">
						<div class="mdl-card__subtitle-text"></div>
					</div>
					<div class="mdl-card__supporting-text" style="height: 3em">
					    {message}
					</div>
					<div class="mdl-card__actions mdl-card--border">
						{ button }
					</div>
				</div>
			</div>
		}
	}
}

pub fn flat_button(label: &str, msg: Msg, link: &ComponentLink<Model>) -> Html {
	let link = link.clone();
	button::flat(label, move || { link.send_message(msg); })
}

pub mod audio {
	use yew::{Html, html, NodeRef};

	pub fn hidden(id: &str, src: &str, node_ref: &NodeRef) -> Html {
		audio(id, false, src, node_ref)
	}

	fn audio(id: &str, controls: bool, src: &str, node_ref: &NodeRef) -> Html {
		let node_ref = node_ref.clone();
		html! {
			<audio id=id controls=controls preload="auto" src=src ref=node_ref/>
		}
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
