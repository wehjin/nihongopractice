use yew::NodeRef;

use crate::utils::now;
use crate::verb::{Audience, Kind, Mode, random_audience, random_mode, random_tense, Tense, Verb, verbs};

pub use self::step::*;

mod step;

fn challenge_steps() -> Vec<ChallengeStep> {
	verbs().into_iter().enumerate().map(|(index, verb)| {
		ChallengeStep {
			name: format!("Question {}", index + 1),
			verb,
			tense: random_tense(),
			audience: random_audience(),
			mode: random_mode(),
		}
	}).collect::<Vec<ChallengeStep>>()
}

#[derive(Clone)]
pub struct Challenge {
	active_steps: Vec<ChallengeStep>,
	resting_steps: Vec<ChallengeStep>,
	pub show_answer: bool,
	pub audio_ref: NodeRef,
	pub play_request_time: f64,
	pub play_time: f64,
}

impl Challenge {
	pub fn new() -> Self {
		let now = now();
		let active_steps = challenge_steps();
		Challenge {
			active_steps,
			resting_steps: Vec::new(),
			show_answer: false,
			audio_ref: NodeRef::default(),
			play_time: now - 3600.0,
			play_request_time: now,
		}
	}

	pub fn active_step(&self) -> &ChallengeStep {
		self.active_steps.first().expect("Expected active step")
	}

	pub fn active_count(&self) -> usize {
		self.active_steps.len()
	}

	pub fn show_answer(&self) -> Self {
		Challenge { show_answer: true, ..self.clone() }
	}

	pub fn pass_question(&self) -> Option<Self> {
		let step = self.active_steps.first().unwrap().clone();
		let active_steps: Vec<ChallengeStep> = self.active_steps.iter().skip(1).cloned().collect();
		if active_steps.is_empty() {
			None
		} else {
			let mut resting_steps = self.resting_steps.to_vec();
			resting_steps.push(step);
			Some(Challenge {
				active_steps,
				resting_steps,
				show_answer: false,
				audio_ref: NodeRef::default(),
				play_request_time: now(),
				play_time: self.play_time,
			})
		}
	}

	pub fn repeat_question(&self) -> Self {
		let step = self.active_steps.first().unwrap().clone();
		let mut active_steps: Vec<ChallengeStep> = self.active_steps.iter().skip(1).cloned().collect();
		active_steps.push(step);
		let resting_steps: Vec<ChallengeStep> = self.resting_steps.clone().into_iter().collect();
		Challenge {
			active_steps,
			resting_steps,
			show_answer: false,
			audio_ref: NodeRef::default(),
			play_request_time: now(),
			play_time: self.play_time,
		}
	}
}
