use yew::{ComponentLink, Html, html};

use crate::{app, mdc, shadow};

pub fn page(model: &shadow::Model, link: &ComponentLink<app::Model>) -> Html {
	mdc::page("Shadow 17.1", Some("Back".to_uppercase()), link, model, content)
}

fn content(_link: &ComponentLink<app::Model>, _model: &shadow::Model) -> Html {
	html! {
		<div class="mdl-grid">
			{ "Hello world" }
		</div>
	}
}