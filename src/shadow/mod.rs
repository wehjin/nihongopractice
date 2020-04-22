use yew::NodeRef;

use crate::utils::audio;

pub use self::core::*;

pub mod core;
pub mod view;

#[derive(Clone, Debug)]
pub enum Model {
	Authorizing,
	Authorized {
		audio_ref: NodeRef,
		shadow: Shadow,
	},
}

impl Model {
	pub fn start() -> Self { Model::Authorizing }

	pub fn update(&self, msg: Msg) -> Option<Self> {
		match (self, msg) {
			(Model::Authorizing, Msg::Authorized) => Some(self.authorize()),
			(Model::Authorized { audio_ref, shadow }, Msg::Play(index)) => {
				let line = &shadow.lines[index];
				audio::play_segment(audio_ref.clone(), line.start, line.end);
				None
			}
			_ => None
		}
	}

	pub fn authorize(&self) -> Self {
		Model::Authorized {
			audio_ref: Default::default(),
			shadow: data::dialog_17_1(),
		}
	}
}

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub enum Msg {
	Start,
	Authorized,
	Play(usize),
}


mod data {
	use crate::shadow::{Line, Shadow};

	pub(crate) fn dialog_17_1() -> Shadow {
		Shadow {
			title: "花見".to_string(),
			lines: vec![
				Line {
					speaker: "narrator".to_string(),
					description: "えりさんとゆみさんは東京国さい大学の国さい学部でビジネスをせんこうしている大学生です。".to_string(),
					start: 7.42481,
					end: 14.609233,
				}
			],
			audio_url: "https://www.ccsf.edu/Departments/Language_Center/oll/japanese/ganbaroo_2/19cd17/02_dialogue_part_1.mp3".to_string(),
		}
	}
}