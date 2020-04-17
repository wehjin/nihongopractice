use yew::{ComponentLink, html, Html};

use crate::app::{Model, Msg};
use crate::recognition::{Challenge, ChallengeStep};
use crate::recognition::round::CompletedRound;
use crate::utils::mdc;

pub const TITLE: &str = "Verb Recognition";

pub fn page(game: &Challenge, link: &ComponentLink<Model>) -> Html {
	link.send_message(Msg::Play(false));
	let page_action = Some(format!("Quit").to_uppercase());
	mdc::page(TITLE, page_action, link, game, content)
}

fn content(link: &ComponentLink<Model>, game: &Challenge) -> Html {
	let completed_rounds = game.completed_rounds.iter().enumerate().map(completed_round).rev();
	html! {
		<div class="mdl-grid">
			{ active_round(link, game) }
			{ for completed_rounds }
		</div>
	}
}

fn completed_round(index_round: (usize, &CompletedRound)) -> Html {
	let (index, round) = index_round;
	let title = format!("Round {}", index + 1);
	let status = format!("completed");
	let details = format!("{} of {} accepted", round.passed, round.passed + round.failed);
	html! {
		<div class="mdl-cell mdl-cell--4-col">
			<div class="mdl-card mdl-shadow--2dp quiz-card">
				<div class="mdl-card__title mdl-color--primary">
					<div class="mdl-card__subtitle-text" style="color:#fff8"> { status } </div>
				</div>
				<div class="mdl-card__title mdl-color--primary">
					<h2 class="mdl-card__title-text" style="color:#fff"> { title } </h2>
				</div>
				<div class="mdl-card__supporting-text">
				    <p>{ details }</p>
				</div>
			</div>
		</div>
    }
}

fn active_round(link: &ComponentLink<Model>, game: &Challenge) -> Html {
	let status =
		format!("Round {}", game.completed_rounds.len() + 1);
	let title =
		format!("# {} of {}", game.active_round.active_index + 1, game.active_round.steps.len());
	html! {
		<div class="mdl-cell mdl-cell--4-col">
			<div class="mdl-card mdl-shadow--2dp quiz-card">
				<audio id="player" preload="auto" src=game.active_step().audio_url() ref=game.audio_ref.clone()/>
				<div class="mdl-card__title mdl-color--primary-dark">
					<div class="mdl-card__subtitle-text" style="color:#fffa">
						{ status }
					</div>
				</div>
				<div class="mdl-card__title mdl-color--primary-dark">
					<h2 class="mdl-card__title-text" style="color:#fff"> { title } </h2>
				</div>
				<div class="mdl-card__supporting-text">
					<p>{ "Write the answer down in english." }</p>
					{ game.active_step().card_answer(game.is_answer_visible)}
				</div>
	            {
	                if game.is_answer_visible {
						repeat_and_go(link)
	                } else {
	                    play_and_show(link)
	                }
	            }
			</div>
		</div>
    }
}

fn play_and_show(link: &ComponentLink<Model>) -> Html {
	html! {
		<div class="mdl-card__actions mdl-card--border">
			{ mdc::flat_button("Replay", Msg::Play(true), link) }
			{ mdc::flat_button("Show Answer", Msg::ShowAnswer, link) }
		</div>
	}
}

fn repeat_and_go(link: &ComponentLink<Model>) -> Html {
	html! {
		<div class="mdl-card__actions mdl-card--border">
			{ mdc::flat_button("Replay", Msg::Play(true), link) }
			{ mdc::flat_button("Repeat", Msg::Repeat, link)}
			{ mdc::flat_button("Accept", Msg::Pass, link)}
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


