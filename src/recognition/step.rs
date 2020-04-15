use verb::Form;

use crate::verb::Verb;

#[derive(Clone, PartialEq, Debug)]
pub struct ChallengeStep {
	pub name: String,
	pub verb: Verb,
	pub form: Form,
}

impl ChallengeStep {
	pub fn audio_url(&self) -> String {
		format!("clips/{}/{}.mp3", self.verb.name(), self.form.name())
	}

	pub fn answer(&self) -> String {
		self.verb.translate(self.form.tense, self.form.polarity, self.form.mode)
	}
}

#[cfg(test)]
mod tests {
	use verb::{Form, Polarity};

	use crate::recognition::ChallengeStep;
	use crate::verb::{Audience, Kind, Mode, Tense, Verb};

	#[test]
	fn has_audio() {
		let step = miru_past_plain_immediate();
		let tag = step.audio_url();
		assert_eq!(tag.as_str(), "clips/100_見る/Past_Plain_Affirmative_Immediate.mp3");
	}

	#[test]
	fn has_answer() {
		let step = miru_past_plain_immediate();
		let answer = step.answer();
		assert_eq!(answer.as_str(), "did see")
	}

	fn miru_past_plain_immediate() -> ChallengeStep {
		let verb = miru();
		let form = Form {
			tense: Tense::Past,
			audience: Audience::Plain,
			polarity: Polarity::Affirmative,
			mode: Mode::Immediate,
		};
		ChallengeStep { name: "Test".to_string(), verb, form }
	}

	fn miru() -> Verb {
		Verb { ch: 100, kind: Kind::Ru, search: "見る".to_string(), english: "see".to_string() }
	}
}
