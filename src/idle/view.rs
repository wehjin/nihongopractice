use yew::{ComponentLink, Html, html};

use crate::app::{Model, Msg};
use crate::data::DEFAULT_CHALLENGE_SIZE;
use crate::mdc;
use crate::shadow;

pub fn page(link: &ComponentLink<Model>) -> Html {
	mdc::page("日本語 Tools", None, link, &(), content)
}

fn content(link: &ComponentLink<Model>, _: &()) -> Html {
	let shadow_card = shadow(link.clone());
	let recognition_card = recognition(link.clone());
	html! {
		<div class="mdl-grid">
			{ shadow_card }
			{ recognition_card }
		</div>
    }
}

fn shadow(link: ComponentLink<Model>) -> Html {
	let title = "Shadow: 17.1 花見";
	let message = "Practice vocalization by simultaneously listening to and speaking each line of dialog.";
	mdc::card::grid(
		title,
		message,
		("Get Started", move || link.send_message(Msg::Shadow(shadow::Msg::Start))),
	)
}

fn recognition(link: ComponentLink<Model>) -> Html {
	mdc::card::grid(
		&format!("Verb Recognition"),
		&format!("Practice recognizing {} verbs selected at random from chapters 1-17.", DEFAULT_CHALLENGE_SIZE),
		("Get Started", move || link.send_message(Msg::Recognition)),
	)
}
