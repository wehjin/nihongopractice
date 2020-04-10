use yew::NodeRef;

use crate::utils::now;
use crate::verb::{Audience, Kind, Mode, Tense, Verb};

pub use self::step::*;

mod step;

#[derive(Clone)]
pub struct Challenge {
	pub active_step: ChallengeStep,
	pub show_answer: bool,
	pub remaining: usize,
	pub audio_ref: NodeRef,
	pub play_request_time: f64,
	pub play_time: f64,
}

impl Challenge {
	pub fn new() -> Self {
		let now = now();
		Challenge {
			active_step: any_step(),
			show_answer: false,
			remaining: 4,
			audio_ref: NodeRef::default(),
			play_time: now - 3600.0,
			play_request_time: now,
		}
	}

	pub fn show_answer(&self) -> Self {
		Challenge { show_answer: true, ..self.clone() }
	}

	pub fn pass_question(&self) -> Option<Self> {
		if self.remaining == 0 {
			None
		} else {
			Some(Challenge {
				show_answer: false,
				active_step: any_step(),
				remaining: self.remaining - 1,
				audio_ref: NodeRef::default(),
				play_request_time: now(),
				play_time: self.play_time,
			})
		}
	}

	pub fn repeat_question(&self) -> Self {
		Challenge {
			show_answer: false,
			active_step: any_step(),
			remaining: self.remaining,
			audio_ref: NodeRef::default(),
			play_request_time: now(),
			play_time: self.play_time,
		}
	}
}

pub fn any_step() -> ChallengeStep {
	ChallengeStep {
		name: "Question 1".to_string(),
		verb: Verb {
			kind: Kind::U,
			search: "あそぶ".to_string(),
			english: "play".to_string(),
		},
		tense: Tense::Present,
		audience: Audience::Plain,
		mode: Mode::Potential,
	}
}
