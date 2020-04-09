use yew::{Component, ComponentLink, Html, ShouldRender};

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
				let game = RecognitionGame { active_verb: any_verb(), show_answer: false, remaining: 4 };
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
			State::Idle => view::idle_page(&self.link),
			State::Recognition { game } => view::recognition_page(game, &self.link),
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

#[derive(Clone)]
pub struct RecognitionGame {
	pub active_verb: Verb,
	pub show_answer: bool,
	pub remaining: usize,
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
				active_verb: any_verb(),
				remaining: self.remaining - 1,
			})
		}
	}

	fn repeat_question(&self) -> Self {
		RecognitionGame {
			show_answer: false,
			active_verb: any_verb(),
			remaining: self.remaining,
		}
	}
}

#[derive(Clone)]
pub struct Verb {
	pub search: String
}

pub fn any_verb() -> Verb {
	Verb { search: "wasureraremasu".to_string() }
}
