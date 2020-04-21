use yew::{ComponentLink, Html, html, NodeRef};

use crate::{app, mdc, shadow};

use super::*;

pub fn page(model: &shadow::Model, link: &ComponentLink<app::Model>) -> Html {
	mdc::page("Shadow 17.1", Some("Back".to_uppercase()), link, model, content)
}

fn content(link: &ComponentLink<app::Model>, model: &shadow::Model) -> Html {
	match model {
		Model::Loading => loading(link),
		Model::Loaded { audio_ref } => loaded(audio_ref),
	}
}

fn loaded(audio_ref: &NodeRef) -> Html {
	let audio = mdc::audio::visible("shadow-player", DIALOG_URL, audio_ref);
	html! {
		<>
		<p>{ "Loaded" }</p>
		<p>{ audio }</p>
		</>
	}
}

fn loading(link: &ComponentLink<app::Model>) -> Html {
	let link = link.clone();
	let button = mdc::button::flat("Continue", move || { link.send_message(app::Msg::Shadow(Msg::FinishedLoading)); });
	html! {
		<>
			<p>{ "Loading" }</p>
			<p><iframe src="https://www.ccsf.edu/Departments/Language_Center/oll/japanese/ganbaroo_2/list17.html" width = "320" height="480"/></p>
			<p>{ button}</p>
		</>
	}
}

const DIALOG_URL: &str = "https://www.ccsf.edu/Departments/Language_Center/oll/japanese/ganbaroo_2/19cd17/02_dialogue_part_1.mp3";
