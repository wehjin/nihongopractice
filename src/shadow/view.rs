use yew::{ComponentLink, Html, html};

use crate::{app, mdc, shadow};

pub fn page(model: &shadow::Model, link: &ComponentLink<app::Model>) -> Html {
	mdc::page("Shadow 17.1", Some("Back".to_uppercase()), link, model, content)
}

fn content(_link: &ComponentLink<app::Model>, model: &shadow::Model) -> Html {
	let audio = mdc::audio::visible("shadow-player", DIALOG_URL, &model.audio_ref);
	html! {
	<>
		{ audio }
		<div class="mdl-grid">
			{ "Hello world" }
		</div>
	</>
	}
}

const DIALOG_URL: &str = "https://www.ccsf.edu/Departments/Language_Center/oll/japanese/ganbaroo_2/19cd17/02_dialogue_part_1.mp3";
