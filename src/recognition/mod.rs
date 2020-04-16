use yew::NodeRef;

use crate::recognition::round::{ActiveRound, AfterStep, CompletedRound, FutureRound};

pub use self::step::*;

mod step;
mod round;
pub mod view;

#[cfg(test)]
mod tests {
	use crate::data::tests::challenge_step_1;
	use crate::recognition::Challenge;

	#[test]
	fn new_challenge() {
		let challenge = challenge_1();
		assert_eq!(challenge.active_count(), 1);
		assert_eq!(challenge.active_step(), &challenge_step_1());
		assert_eq!(challenge.is_answer_visible, false);
	}

	#[test]
	fn pass_last_step() {
		let challenge = challenge_1();
		assert_eq!(challenge.pass_question(), Option::None)
	}

	fn challenge_1() -> Challenge {
		let challenge = Challenge::new(&vec![challenge_step_1()]);
		challenge
	}
}

#[derive(Clone, PartialEq, Debug)]
pub struct Challenge {
	pub active_round: ActiveRound,
	pub future_round: Option<FutureRound>,
	pub completed_rounds: Vec<CompletedRound>,
	pub is_answer_visible: bool,
	pub audio_ref: NodeRef,
	pub play: bool,
}

impl Challenge {
	pub fn pass_question(&self) -> Option<Self> {
		match self.active_round.pass() {
			AfterStep::Continue(active_round) => {
				Some(Challenge {
					active_round,
					future_round: self.future_round.to_owned(),
					completed_rounds: self.completed_rounds.to_vec(),
					is_answer_visible: false,
					audio_ref: Default::default(),
					play: true,
				})
			}
			AfterStep::Retire(completed_round) => {
				let mut completed_rounds = self.completed_rounds.to_vec();
				completed_rounds.push(completed_round);
				match &self.future_round {
					None => None,
					Some(future_round) => Some(Challenge {
						active_round: future_round.activate(),
						future_round: Option::None,
						completed_rounds,
						is_answer_visible: false,
						audio_ref: Default::default(),
						play: true,
					}),
				}
			}
		}
	}

	pub fn repeat_question(&self) -> Self {
		let (after_step, future_round) = self.active_round.fail(&self.future_round);
		match after_step {
			AfterStep::Continue(active_round) => {
				Challenge {
					active_round,
					future_round: Some(future_round),
					completed_rounds: self.completed_rounds.to_vec(),
					is_answer_visible: false,
					audio_ref: Default::default(),
					play: true,
				}
			}
			AfterStep::Retire(completed_round) => {
				let mut completed_rounds = self.completed_rounds.to_vec();
				completed_rounds.push(completed_round);
				Challenge {
					active_round: future_round.activate(),
					future_round: None,
					completed_rounds,
					is_answer_visible: false,
					audio_ref: Default::default(),
					play: true,
				}
			}
		}
	}

	pub fn show_answer(&self) -> Self {
		Challenge { is_answer_visible: true, ..self.clone() }
	}

	pub fn active_step(&self) -> &ChallengeStep {
		self.active_round.active_step()
	}

	pub fn active_count(&self) -> usize {
		self.active_round.remaining()
	}

	pub fn new(steps: &Vec<ChallengeStep>) -> Self {
		let active_round = ActiveRound::new(steps);
		Challenge {
			active_round,
			future_round: Option::None,
			completed_rounds: Vec::new(),
			is_answer_visible: false,
			audio_ref: NodeRef::default(),
			play: true,
		}
	}
}
