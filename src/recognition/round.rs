use crate::recognition::ChallengeStep;

#[cfg(test)]
mod tests {
	use crate::data::tests::{challenge_step_1, challenge_step_2};
	use crate::recognition::round::{ActiveRound, AfterStep, CompletedRound};

	#[test]
	fn new() {
		let round = active_round_1();
		assert_eq!(round.remaining(), 1);
		assert_eq!(round.passed(), 0);
		assert_eq!(round.failed(), 0);
		assert_eq!(round.active_step(), &challenge_step_1())
	}

	#[test]
	fn pass_last_step() {
		let round = active_round_1();
		let after = round.pass();
		match after {
			AfterStep::Continue(_) => panic!("Last pass must not continue"),
			AfterStep::Retire(completed) => {
				assert_eq!(completed, CompletedRound { passed: 1, failed: 0 });
			}
		}
	}

	#[test]
	fn fail_last_step() {
		let round = active_round_1();
		let (after, future) = round.fail(&Option::None);
		assert_eq!(future.scheduled(), 1);
		match after {
			AfterStep::Continue(_) => panic!("Last fail must not continue"),
			AfterStep::Retire(completed) => {
				assert_eq!(completed, CompletedRound { passed: 0, failed: 1 })
			}
		}
	}

	#[test]
	fn pass_first_step() {
		let round = active_round_2();
		match round.pass() {
			AfterStep::Retire(_) => panic!("First pass must not retire"),
			AfterStep::Continue(next) => {
				assert_eq!(next.remaining(), 1);
				assert_eq!(next.passed(), 1);
				assert_eq!(next.failed(), 0);
			}
		}
	}

	#[test]
	fn fail_first_step() {
		let round = active_round_2();
		let (after, future) = round.fail(&Option::None);
		assert_eq!(1, future.scheduled(), "scheduled");
		match after {
			AfterStep::Retire(_) => panic!("First fail must not retire"),
			AfterStep::Continue(next) => {
				assert_eq!(next.remaining(), 1, "remaining");
				assert_eq!(next.passed(), 0, "passed");
				assert_eq!(next.failed(), 1, "failed");
			}
		}
	}

	fn active_round_1() -> ActiveRound {
		ActiveRound::new(&vec![challenge_step_1()])
	}

	fn active_round_2() -> ActiveRound {
		ActiveRound::new(&vec![challenge_step_1(), challenge_step_2()])
	}
}


#[derive(Clone, PartialEq, Debug)]
pub struct ActiveRound {
	steps: Vec<ChallengeStep>,
	active_index: usize,
	passed: usize,
	failed: usize,
}

impl ActiveRound {
	pub fn pass(&self) -> AfterStep {
		if self.remaining() == 1 {
			AfterStep::Retire(CompletedRound { passed: self.passed + 1, failed: self.failed })
		} else {
			AfterStep::Continue(ActiveRound {
				steps: self.steps.to_vec(),
				active_index: self.active_index + 1,
				passed: self.passed + 1,
				failed: self.failed,
			})
		}
	}

	pub fn fail(&self, future_round: &Option<FutureRound>) -> (AfterStep, FutureRound) {
		let failed_step = &self.steps[self.active_index];
		let passed = self.passed;
		let failed = self.failed + 1;
		let future_round = match future_round {
			None => FutureRound::new(failed_step),
			Some(future_round) => future_round.schedule(failed_step),
		};
		let after_step = if self.remaining() == 1 {
			let completed_round = CompletedRound { passed, failed };
			AfterStep::Retire(completed_round)
		} else {
			let steps = self.steps.to_vec();
			let active_index = self.active_index + 1;
			let active_round = ActiveRound { steps, active_index, passed, failed };
			AfterStep::Continue(active_round)
		};
		(after_step, future_round)
	}

	pub fn remaining(&self) -> usize {
		self.steps.len() - self.active_index
	}

	pub fn passed(&self) -> usize {
		self.passed
	}

	pub fn failed(&self) -> usize {
		self.failed
	}

	pub fn active_step(&self) -> &ChallengeStep {
		&self.steps[self.active_index]
	}

	pub fn new(steps: &Vec<ChallengeStep>) -> ActiveRound {
		ActiveRound {
			steps: steps.to_vec(),
			active_index: 0,
			passed: 0,
			failed: 0,
		}
	}
}

#[derive(Clone, PartialEq, Debug)]
pub enum AfterStep {
	Continue(ActiveRound),
	Retire(CompletedRound),
}

#[derive(Clone, PartialEq, Debug)]
pub struct FutureRound {
	steps: Vec<ChallengeStep>,
}

impl FutureRound {
	pub fn new(step: &ChallengeStep) -> Self {
		FutureRound {
			steps: vec![step.to_owned()]
		}
	}

	pub fn schedule(&self, step: &ChallengeStep) -> Self {
		let mut steps = self.steps.to_vec();
		steps.push(step.to_owned());
		FutureRound { steps }
	}

	pub fn scheduled(&self) -> usize {
		self.steps.len()
	}

	pub fn activate(&self) -> ActiveRound {
		ActiveRound::new(&self.steps)
	}
}

#[derive(Clone, PartialEq, Debug)]
pub struct CompletedRound {
	passed: usize,
	failed: usize,
}
