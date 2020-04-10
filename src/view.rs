use yew::{ComponentLink, html, Html};

use crate::app::{Model, Msg};
use crate::recognition::Challenge;
use crate::utils::{audio_url, mdc};

pub fn idle_page(link: &ComponentLink<Model>) -> Html {
	mdc::page("Verb Trainer", None, link, &(), idle_content)
}


fn idle_content(link: &ComponentLink<Model>, _: &()) -> Html {
	html! {
<ol>
	<li>
	{ mdc::flat_button( "Recognition Game", Msg::Recognition, link)}
    </li>
</ol>
    }
}

pub fn recognition_page(game: &Challenge, link: &ComponentLink<Model>) -> Html {
	link.send_message(Msg::Play(false));
	let page_action = Some(format!("Quit ({} remaining)", game.remaining + 1).to_uppercase());
	mdc::page("Recognition", page_action, link, game, recognition_content)
}

fn recognition_content(link: &ComponentLink<Model>, game: &Challenge) -> Html {
	html! {
		<section>
			<div class="mdl-card__title">
				<h2 class="mdl-card__title_text">{ &game.active_step.name }</h2>
			</div>
			<div class="mdl-card__supporting-text">
				<audio src=audio_url(&game.active_step.audio_tag()) ref=game.audio_ref.clone()/>
				{ recognition_answer_row(game.show_answer, &game.active_step.answer(), link) }
				{
					if game.show_answer {
						recognition_action_row(link)
					} else {
						mdc::empty_div()
					}
				}
	        </div>
		</section>
    }
}

fn recognition_answer_row(show_answer: bool, answer: &str, link: &ComponentLink<Model>) -> Html {
	if show_answer {
		recognition_answer_text(answer)
	} else {
		recognition_play_and_show(link)
	}
}

fn recognition_play_and_show(link: &ComponentLink<Model>) -> Html {
	html! {
	<>
	<p/><p/>
	<div>
	{ mdc::flat_button("Play", Msg::Play(true), link) }
	{ mdc::flat_button("Show Answer", Msg::ShowAnswer, link) }
	</div>
	</>
	}
}

fn recognition_answer_text(answer: &str) -> Html {
	html! {
		<span class="mdl-chip mdl-chip--contact">
			<span class="mdl-chip__contact mdl-color--primary mdl-color-text--white">{"≈"}</span>
            <span class="mdl-chip__text">{answer}</span>
		</span>
	}
}

fn recognition_action_row(link: &ComponentLink<Model>) -> Html {
	html! {
	<div class="mdl-dialog__content">
		{" How did it go? "}
		{mdc::flat_button("Hard • Repeat it", Msg::Repeat, link)}
		{mdc::flat_button("Easy • Retire it", Msg::Pass, link)}
	</div>
	}
}

