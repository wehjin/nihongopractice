use yew::{ComponentLink, html, Html};

use crate::app::{Model, Msg, RecognitionGame};
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

pub fn recognition_page(game: &RecognitionGame, link: &ComponentLink<Model>) -> Html {
	let page_action = Some(format!("Quit ({} remaining)", game.remaining + 1).to_uppercase());
	mdc::page("Recognition", page_action, link, game, recognition_content)
}

fn recognition_content(link: &ComponentLink<Model>, game: &RecognitionGame) -> Html {
	html! {
			<ul>
			<p><audio controls=true src=audio_url(&game.active_verb.search)></audio></p>
			<p>{ recognition_answer_row(game.show_answer, link) }</p>
			{
				if game.show_answer {
					recognition_action_row(link)
				} else {
					mdc::empty_div()
				}
			}
            </ul>
        }
}

fn recognition_answer_row(show_answer: bool, link: &ComponentLink<Model>) -> Html {
	if show_answer {
		recognition_answer_text()
	} else {
		mdc::flat_button("Show Answer", Msg::ShowAnswer, link)
	}
}

fn recognition_answer_text() -> Html {
	let answer = &format!("{}", "can forget");
	html! {
		<span class="mdl-chip mdl-chip--contact">
			<span class="mdl-chip__contact mdl-color--primary mdl-color-text--white">{"≈"}</span>
            <span class="mdl-chip__text">{answer}</span>
		</span>
	}
}

fn recognition_action_row(link: &ComponentLink<Model>) -> Html {
	html! {
	<p>
		{ mdc::primary_span("How was it?")}
		{mdc::flat_button("Hard • Repeat", Msg::Repeat, link)}
		{mdc::flat_button("Easy • Retire", Msg::Pass, link)}
	</p>
	}
}

