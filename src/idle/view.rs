use yew::{ComponentLink, Html, html};

use crate::app::{Model, Msg};
use crate::data::DEFAULT_CHALLENGE_SIZE;
use crate::mdc;
use crate::recognition;

pub fn page(link: &ComponentLink<Model>) -> Html {
	mdc::page("日本語 Tools", None, link, &(), content)
}

fn content(link: &ComponentLink<Model>, _: &()) -> Html {
	let shadow_card = shadow(link.clone());
	let recognition_card = recognition(link.clone());
	html! {
		<div class="mdl-grid">
			{ recognition_card }
			{ shadow_card }
		</div>
    }
}

fn shadow(link: ComponentLink<Model>) -> Html {
	mdc::card::grid(
		"Shadow 17.1",
		"Speak dialog lines while listening.",
		("Get Started", move || link.send_message(Msg::Shadow)),
	)
}

fn recognition(link: ComponentLink<Model>) -> Html {
	mdc::card::grid(
		recognition::view::TITLE,
		&format!(
			"Listen to and translate {} verbs selected at random from chapters 1-17.",
			DEFAULT_CHALLENGE_SIZE
		),
		("Get Started", move || link.send_message(Msg::Recognition)),
	)
}
