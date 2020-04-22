use yew::{Component, ComponentLink, Html, ShouldRender};

use crate::{idle, recognition, shadow};
use crate::data::random_steps;
use crate::recognition::Challenge;
use crate::utils::audio;

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

	fn mounted(&mut self) -> ShouldRender {
		info!("Mounted called");
		false
	}

	fn update(&mut self, msg: Self::Message) -> ShouldRender {
		let new_state = match msg {
			Msg::Quit => Some(State::Idle),
			Msg::Shadow(msg) => {
				match &self.state {
					State::Idle => Some(shadow::Model::start()),
					State::Recognition { .. } => None,
					State::Shadow(shadow_model) => shadow_model.update(msg),
				}.map(|it| State::Shadow(it))
			}
			Msg::Recognition => Some(State::Recognition { game: Challenge::new(&random_steps()) }),
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
			Msg::Play(replay) => {
				if let State::Recognition { ref mut game } = self.state {
					if replay {
						game.play = true;
					}
					if game.play {
						audio::play(game.audio_ref.clone());
						game.play = false;
					}
				}
				None
			}
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
			State::Idle => idle::view::page(&self.link),
			State::Recognition { game } => recognition::view::page(game, &self.link),
			State::Shadow(model) => shadow::view::page(model, &self.link),
		}
	}
}

pub enum State {
	Idle,
	Recognition { game: Challenge },
	Shadow(shadow::Model),
}

#[derive(Copy, Clone)]
pub enum Msg {
	Quit,
	Recognition,
	ShowAnswer,
	Repeat,
	Pass,
	Play(bool),
	Shadow(shadow::Msg),
}
