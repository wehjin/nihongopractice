use verb::{Kind, random, Verb};

use crate::data;
use crate::recognition::ChallengeStep;
use crate::shadow::{Line, Shadow};

#[cfg(test)]
pub(crate) mod tests {
	use verb::{Audience, Form, Kind, Mode, Polarity, Tense, Verb};

	use crate::recognition::ChallengeStep;

	pub fn challenge_step_1() -> ChallengeStep {
		ChallengeStep {
			name: "1".to_string(),
			verb: miru(),
			form: present_plain_affirmative_immediate(),
		}
	}

	#[cfg(test)]
	pub fn challenge_step_2() -> ChallengeStep {
		ChallengeStep {
			name: "2".to_string(),
			verb: kau(),
			form: present_plain_affirmative_immediate(),
		}
	}

	#[cfg(test)]
	pub fn miru() -> Verb {
		Verb {
			ch: 100,
			kind: Kind::Ru,
			search: "見る".to_string(),
			english: "see".to_string(),
		}
	}

	#[cfg(test)]
	pub fn kau() -> Verb {
		Verb {
			ch: 100,
			kind: Kind::U,
			search: "買う".to_string(),
			english: "buy".to_string(),
		}
	}

	#[cfg(test)]
	pub fn present_plain_affirmative_immediate() -> Form {
		Form {
			tense: Tense::Present,
			audience: Audience::Plain,
			polarity: Polarity::Affirmative,
			mode: Mode::Immediate,
		}
	}
}

pub fn ch17_1_shadow() -> Shadow {
	include!["shadow.in"]
}

pub const DEFAULT_CHALLENGE_SIZE: usize = 10;

#[cfg(ignore)]
pub fn verb_steps() -> Vec<ChallengeStep> {
	let verb = Verb {
		ch: 15,
		kind: Kind::U,
		search: "つくる".to_string(),
		english: "create".to_string(),
	};
	verb::all::forms().into_iter().enumerate().map(|(index, form)| {
		ChallengeStep {
			name: format!("Verb {}", verb.english.to_owned()),
			verb: verb.clone(),
			form,
		}
	}).collect()
}

pub fn random_steps() -> Vec<ChallengeStep> {
	data::n_shuffled(DEFAULT_CHALLENGE_SIZE).into_iter().enumerate().map(|(index, verb)| {
		ChallengeStep {
			name: format!("Question {}", index + 1),
			verb,
			form: random::form(),
		}
	}).collect::<Vec<ChallengeStep>>()
}

pub fn n_shuffled(count: usize) -> Vec<Verb> {
	let mut all = verbs();
	let count = count.min(all.len());
	let mut shuffled = Vec::new();
	while shuffled.len() < count && all.len() > 0 {
		let mut buffer = [0u8, 0u8];
		getrandom::getrandom(&mut buffer).unwrap();
		let index = (((buffer[0] as usize) << 8) + (buffer[1] as usize)) % all.len();
		let selected = all.remove(index);
		shuffled.push(selected)
	}
	shuffled
}


fn verbs() -> Vec<Verb> {
	include!("verbs.in")
}
