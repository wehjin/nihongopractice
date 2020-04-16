use yew::{ComponentLink, Html, html};

use crate::app::{Model, Msg};
use crate::utils::mdc;

pub fn page(link: &ComponentLink<Model>) -> Html {
	mdc::page("Verb Trainer", None, link, &(), content)
}

fn content(link: &ComponentLink<Model>, _: &()) -> Html {
	html! {
		<div class="mdl-grid">
			<div class="mdl-cell mdl-cell--4-col">
				<div class=" mdl-card mdl-shadow--2dp">
					<div class="mdl-card__title mdl-color--primary-dark" style="color:#fff">
					    <h2 class="mdl-card__title-text">{"Recognition"}</h2>
					</div>
					<div class="mdl-card__title mdl-color--primary-dark">
						<div class="mdl-card__subtitle-text"></div>
					</div>
					<div class="mdl-card__supporting-text">
						{"Listen to and translate 20 verbs selected at random from chapters 1-17."}
					</div>
					<div class="mdl-card__actions mdl-card--border">
						{ mdc::flat_button("Get Started", Msg::Recognition, link) }
					</div>
				</div>
			</div>
		</div>
    }
}

