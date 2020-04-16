use yew::{ComponentLink, Html, html};

use crate::app::{Model, Msg};
use crate::utils::mdc;

pub fn page(link: &ComponentLink<Model>) -> Html {
	mdc::page("Verb Trainer", None, link, &(), content)
}

fn content(link: &ComponentLink<Model>, _: &()) -> Html {
	html! {
		<div class="mdl-grid center-items">
			<div class="mdl-cell mdl-cell--4-col mdl-cell--middle">
		        <button
		            class="mdl-button mdl-js-button mdl-button--raised mdl-js-ripple-effect"
		            style="width:100%"
		            onclick=link.callback(|_| Msg::Recognition)>
		            { "Recognition Challenge" }
		        </button>
			</div>
		</div>
    }
}

