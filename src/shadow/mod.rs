pub use shadow_core::*;
use yew::NodeRef;

use crate::data::ch17_1_shadow;
use crate::utils::audio;

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
			shadow: ch17_1_shadow(),
		}
	}
}

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub enum Msg {
	Start,
	Authorized,
	Play(usize),
}
