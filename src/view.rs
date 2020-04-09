use yew::{ComponentLink, html, Html};

use crate::app::{Model, Msg, RecognitionGame};

pub fn answer_button(link: &ComponentLink<Model>) -> Html {
	html! {
	<button onclick=link.callback(|_| Msg::ShowAnswer)>{ "Show Answer" }</button>
	}
}


pub fn idle(link: &ComponentLink<Model>) -> Html {
	page("Verb Trainer", link, idle_content)
}

fn idle_content(link: &ComponentLink<Model>) -> Html {
	html! {
<ol>
	<li>
	{ flat_button( "Recognition Game", Msg::Recognition, link)}
    </li>
</ol>
    }
}


fn recognition_answer() -> Html {
	html! {
			<div>
			<p>{ "＝ can forget" }</p>
			</div>
		}
}

fn recognition_answer_row(show_answer: bool, link: &ComponentLink<Model>) -> Html {
	if show_answer {
		recognition_answer()
	} else {
		answer_button(link)
	}
}

fn recognition_action_row(link: &ComponentLink<Model>) -> Html {
	html! {
			<div>
			<p>
			<button onclick=link.callback(|_| Msg::Repeat)>{ "Hard. Repeat" }</button>
			</p>
			<p>
			<button onclick=link.callback(|_| Msg::Pass)>{ "Easy. Next" }</button>
			</p>
			</div>
		}
}

pub fn recognition_page(game: &RecognitionGame, link: &ComponentLink<Model>) -> Html {
	let RecognitionGame { active_verb, show_answer, remaining } = game;
	html! {
			<div>

			<h1>
			{"Recognition"}
			<button onclick=link.callback(|_| Msg::Quit)>{ format!("Quit ({} remaining)", remaining+1) }</button>
			</h1>
			<div>
			</div>
			<p>
			<audio controls=true src=audio_url(&active_verb.search)></audio>
			</p>
			<p>
			{ recognition_answer_row(*show_answer, link) }
			</p>
			{
				if *show_answer {
					recognition_action_row(link)
				} else {
					html!{}
				}
			}
            </div>
        }
}

fn audio_url(search: &str) -> String {
	format!("http://translate.google.com/translate_tts?ie=UTF-8&client=tw-ob&q={}&tl=ja", search)
}

pub fn flat_button(label: &str, msg: Msg, link: &ComponentLink<Model>) -> Html {
	html! {
        <button class="mdl-button mdl-js-button mdl-button--primary" onclick=link.callback(move |_| msg)>{ label }</button>
	}
}

pub fn page(title: &str, link: &ComponentLink<Model>, content: impl Fn(&ComponentLink<Model>) -> Html) -> Html {
	let link_text = "Link";
	html! {
<div class="mdl-layout mdl-js-layout">
  <header class="mdl-layout__header  mdl-layout__header--scroll">
    <div class="mdl-layout__header-row">
      <span class="mdl-layout-title">{title}</span>
      <div class="mdl-layout-spacer"></div>
      <nav class="mdl-navigation">
        <a class="mdl-navigation__link" href="">{link_text}</a>
      </nav>
    </div>
  </header>
  <div class="mdl-layout__drawer">
    <span class="mdl-layout-title">{title}</span>
    <nav class="mdl-navigation">
      <a class="mdl-navigation__link" href="">{link_text}</a>
    </nav>
  </div>
  <main class="mdl-layout__content">{content(link)}</main>
</div>
	}
}
