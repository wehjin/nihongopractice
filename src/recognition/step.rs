use crate::verb::{Audience, Mode, Tense, Verb};

#[derive(Clone, PartialEq, Debug)]
pub struct ChallengeStep {
	pub name: String,
	pub verb: Verb,
	pub tense: Tense,
	pub audience: Audience,
	pub mode: Mode,
}

impl ChallengeStep {
	pub fn audio_tag(&self) -> String {
		self.verb.conjugate(self.tense, self.audience, self.mode)
	}

	pub fn answer(&self) -> String {
		self.verb.translate(self.tense, self.mode)
	}
}

#[cfg(test)]
mod tests {
	use crate::recognition::ChallengeStep;
	use crate::verb::{Audience, Kind, Mode, Tense, Verb};

	#[test]
	fn has_audio_tag() {
		let step = miru_past_plain_immediate();
		let tag = step.audio_tag();
		assert_eq!(tag.as_str(), "見た");
	}

	#[test]
	fn has_answer() {
		let step = miru_past_plain_immediate();
		let answer = step.answer();
		assert_eq!(answer.as_str(), "did see")
	}

	fn miru_past_plain_immediate() -> ChallengeStep {
		let verb = miru();
		let tense = Tense::Past;
		let audience = Audience::Plain;
		let mode = Mode::Immediate;
		ChallengeStep { name: "Test".to_string(), verb, tense, audience, mode }
	}

	fn miru() -> Verb {
		Verb { kind: Kind::Ru, search: "見る".to_string(), english: "see".to_string() }
	}
}
