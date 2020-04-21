use yew::{Callback, ComponentLink, Html, html};

use crate::app::{Model, Msg};
use crate::data::DEFAULT_CHALLENGE_SIZE;
use crate::mdc;
use crate::recognition;

pub fn page(link: &ComponentLink<Model>) -> Html {
	mdc::page("日本語 Tools", None, link, &(), content)
}

fn content(link: &ComponentLink<Model>, _: &()) -> Html {
	let recognition_card = recognition_card(link.callback(move |_| Msg::Recognition));
	html! {
		<div class="mdl-grid"> { recognition_card } </div>
    }
}

fn recognition_card(on_click: Callback<web_sys::MouseEvent>) -> Html {
	home_card(
		recognition::view::TITLE,
		&format!(
			"Listen to and translate {} verbs selected at random from chapters 1-17.",
			DEFAULT_CHALLENGE_SIZE
		),
		on_click,
	)
}

fn home_card(title: &str, message: &str, action: Callback<web_sys::MouseEvent>) -> Html {
	html! {
		<div class="mdl-cell mdl-cell--4-col">
			<div class=" mdl-card mdl-shadow--2dp tool-card">
				<div class="mdl-card__title mdl-color--primary-dark" style="color:#fff">
				    <h2 class="mdl-card__title-text">{ title }</h2>
				</div>
				<div class="mdl-card__title mdl-color--primary-dark">
					<div class="mdl-card__subtitle-text"></div>
				</div>
				<div class="mdl-card__supporting-text"> {message} </div>
				<div class="mdl-card__actions mdl-card--border">
					{ mdc::button::flat("Get Started", action) }
				</div>
			</div>
		</div>
	}
}
