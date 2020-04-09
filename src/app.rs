use yew::{Component, ComponentLink, html, Html, ShouldRender};

use super::view;

pub struct Model {
	link: ComponentLink<Self>,
	state: State,
}

impl Component for Model {
	type Message = Msg;
	type Properties = ();

	fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
		Model { link, state: State::Idle }
	}

	fn update(&mut self, msg: Self::Message) -> ShouldRender {
		let new_state = match msg {
			Msg::Quit => Some(State::Idle),
			Msg::Recognition => {
				let game = RecognitionGame { active_verb: active_verb(), show_answer: false, remaining: 4 };
				Some(State::Recognition { game })
			}
			Msg::ShowAnswer => match &self.state {
				State::Recognition { game } => {
					let new_game = game.show_answer();
					Some(State::Recognition { game: new_game })
				}
				_ => None,
			},
			Msg::Pass => match &self.state {
				State::Recognition { game } => {
					match game.pass_question() {
						Some(game) => Some(State::Recognition { game }),
						None => Some(State::Idle)
					}
				}
				_ => None
			},
			Msg::Repeat => match &self.state {
				State::Recognition { game } => {
					let new_game = game.repeat_question();
					Some(State::Recognition { game: new_game })
				}
				_ => None
			},
		};
		match new_state {
			Some(it) => {
				self.state = it;
				true
			}
			None => false
		}
	}

	fn view(&self) -> Html {
		match &self.state {
			State::Idle => view::idle(&self.link),
			State::Recognition { game } => self.view_recognition(game),
		}
	}
}

pub enum State {
	Idle,
	Recognition { game: RecognitionGame },
}

#[derive(Copy, Clone)]
pub enum Msg {
	Quit,
	Recognition,
	ShowAnswer,
	Repeat,
	Pass,
}

impl Model {
	fn view_answer_text(&self) -> Html {
		html! {
			<div>
			<p>{ "＝ can forget" }</p>
			</div>
		}
	}

	fn view_answer(&self, show_answer: bool) -> Html {
		if show_answer {
			self.view_answer_text()
		} else {
			view::answer_button(&self.link)
		}
	}

	fn view_next(&self) -> Html {
		html! {
			<div>
			<p>
			<button onclick=self.link.callback(|_| Msg::Repeat)>{ "Hard. Repeat" }</button>
			</p>
			<p>
			<button onclick=self.link.callback(|_| Msg::Pass)>{ "Easy. Next" }</button>
			</p>
			</div>
		}
	}

	fn view_recognition(&self, game: &RecognitionGame) -> Html {
		let RecognitionGame { active_verb, show_answer, remaining } = game;
		html! {
			<div>

			<h1>
			{"Recognition"}
			<button onclick=self.link.callback(|_| Msg::Quit)>{ format!("Quit ({} remaining)", remaining+1) }</button>
			</h1>
			<div>
			</div>
			<p>
			<audio controls=true src=audio_url(&active_verb.search)></audio>
			</p>
			<p>
			{ self.view_answer(*show_answer) }
			</p>
			{
				if *show_answer {
					self.view_next()
				} else {
					html!{}
				}
			}

            </div>
        }
	}
}

fn audio_url(search: &str) -> String {
	format!("http://translate.google.com/translate_tts?ie=UTF-8&client=tw-ob&q={}&tl=ja", search)
}

fn active_verb() -> Verb {
	let active_verb = Verb { search: "wasureraremasu".to_string() };
	active_verb
}

#[derive(Clone)]
pub struct RecognitionGame {
	active_verb: Verb,
	show_answer: bool,
	remaining: usize,
}

impl RecognitionGame {
	fn show_answer(&self) -> Self {
		RecognitionGame { show_answer: true, ..self.clone() }
	}

	fn pass_question(&self) -> Option<Self> {
		if self.remaining == 0 {
			None
		} else {
			Some(RecognitionGame {
				show_answer: false,
				active_verb: active_verb(),
				remaining: self.remaining - 1,
			})
		}
	}

	fn repeat_question(&self) -> Self {
		RecognitionGame {
			show_answer: false,
			active_verb: active_verb(),
			remaining: self.remaining,
		}
	}
}

#[derive(Clone)]
pub struct Verb {
	search: String
}
