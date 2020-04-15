use yew::{ComponentLink, html, Html};

use crate::app::{Model, Msg};
use crate::recognition::{Challenge, ChallengeStep};
use crate::utils::mdc;

pub fn idle_page(link: &ComponentLink<Model>) -> Html {
	mdc::page("Verb Trainer", None, link, &(), idle_content)
}

fn idle_content(link: &ComponentLink<Model>, _: &()) -> Html {
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

pub fn recognition_page(game: &Challenge, link: &ComponentLink<Model>) -> Html {
	link.send_message(Msg::Play(false));
	let count = game.active_count();
	let page_action = if count == 1 {
		None
	} else {
		Some(format!("Quit ({} to go)", count).to_uppercase())
	};
	mdc::page("Recognition", page_action, link, game, recognition_content)
}

fn recognition_content(link: &ComponentLink<Model>, game: &Challenge) -> Html {
	html! {
		<div class="mdl-grid center-items ">
			<div class="mdl-cell mdl-cell--4-col mdl-cell--middle">
				<div class="mdl-card mdl-shadow--2dp" style="width:100%">
					<audio id="player" preload="auto" src=game.active_step().audio_url() ref=game.audio_ref.clone()/>
					<div class="mdl-card__title mdl-color--indigo-50">
						<h2 class="mdl-card__title_text">{ &game.active_step().name }</h2>
					</div>
					<div class="mdl-card__title">
						<div class="mdl-card__subtitle-text">
							{ game.active_step().card_answer(game.show_answer)}
				        </div>
					</div>
					<div class="mdl-card__supporting-text"/>
		            {
		                if game.show_answer {
							recognition_repeat_and_go(link)
		                } else {
		                    recognition_play_and_show(link)
		                }
		            }
				</div>
			</div>
		</div>
    }
}

fn recognition_play_and_show(link: &ComponentLink<Model>) -> Html {
	html! {
		<div class="mdl-card__actions mdl-card--border">
			{ mdc::flat_button("Play Again", Msg::Play(true), link) }
			{ mdc::flat_button("Reveal Answer", Msg::ShowAnswer, link) }
		</div>
	}
}

fn recognition_repeat_and_go(link: &ComponentLink<Model>) -> Html {
	html! {
		<div class="mdl-card__actions mdl-card--border">
			{ mdc::flat_button("Play Again", Msg::Play(true), link) }
			{ mdc::flat_button("Repeat it", Msg::Repeat, link)}
			{ mdc::flat_button("Nailed it !", Msg::Pass, link)}
		</div>
	}
}

impl ChallengeStep {
	fn card_answer(&self, show_answer: bool) -> Html {
		html! {
			<span class="mdl-chip mdl-chip--contact">
				<span class="mdl-chip__contact mdl-color--primary mdl-color-text--white">{"≈"}</span>
				<i>
				<span class="mdl-chip__text">
		            { if show_answer { self.answer() } else { "HIDDEN".to_string() } }
	            </span>
	            </i>
			</span>
		}
	}
}
