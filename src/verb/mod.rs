use crate::utils::{drop_last_char, last_char};

pub use self::core::*;

mod core;

pub struct Verb {
	pub kind: Kind,
	pub search: String,
}

impl Verb {
	pub fn conjugate(&self, tense: Tense, audience: Audience, mode: Mode) -> String {
		match mode {
			Mode::Immediate => conjugate_verb(&self.search, self.kind, tense, audience),
			Mode::Potential => {
				let potential = potential(&self.search, self.kind);
				conjugate_verb(&potential, Kind::Ru, tense, audience)
			}
		}
	}

	fn dropped_u(&self) -> String { drop_last_char(&self.search) }
}

fn potential(verb: &str, kind: Kind) -> String {
	let dropped_u = drop_last_char(verb);
	let new_ending = potential_ending(verb, kind);
	format!("{}{}", dropped_u, new_ending)
}

fn potential_ending(verb: &str, kind: Kind) -> String {
	match kind {
		Kind::Ru => "られる",
		Kind::U => {
			let u: &str = &last_char(verb);
			match u {
				"う" => "える",
				"つ" => "てる",
				"る" => "れる",
				"む" => "める",
				"ぶ" => "べる",
				"ぬ" => "ねる",
				"く" => "ける",
				"ぐ" => "げる",
				"す" => "せる",
				_ => panic!("Invalid ending")
			}
		}
	}.to_string()
}

fn conjugate_verb(verb: &str, kind: Kind, tense: Tense, audience: Audience) -> String {
	match (tense, audience) {
		(Tense::Present, Audience::Plain) => verb.to_string(),
		(Tense::Present, Audience::Polite) => format!("{}ます", pre_masu(verb, kind)),
		(Tense::Past, Audience::Plain) => ta(verb, kind),
		(Tense::Past, Audience::Polite) => format!("{}ました", pre_masu(verb, kind)),
	}
}

fn ta(verb: &str, kind: Kind) -> String {
	let ta = match kind {
		Kind::Ru => "た",
		Kind::U => {
			let u: &str = &last_char(verb);
			match u {
				"う" => "った",
				"つ" => "った",
				"る" => "った",
				"む" => "んだ",
				"ぶ" => "んだ",
				"ぬ" => "んだ",
				"く" => "いた",
				"ぐ" => "いだ",
				"す" => "した",
				_ => panic!("Invalid ending")
			}
		}
	};
	format!("{}{}", drop_last_char(verb), ta)
}

fn pre_masu(verb: &str, kind: Kind) -> String {
	let dropped_u = drop_last_char(verb);
	match kind {
		Kind::Ru => dropped_u,
		Kind::U => {
			let u: &str = &last_char(verb);
			let i = match u {
				"う" => "い",
				"つ" => "ち",
				"る" => "り",
				"む" => "み",
				"ぶ" => "び",
				"ぬ" => "に",
				"く" => "き",
				"ぐ" => "ぎ",
				"す" => "し",
				_ => panic!("Invalid ending")
			};
			format!("{}{}", dropped_u, i)
		}
	}.to_string()
}

#[cfg(test)]
mod tests {
	use crate::verb::Audience::{Plain, Polite};
	use crate::verb::Mode::{Immediate, Potential};
	use crate::verb::Tense::{Past, Present};

	use super::*;

	#[test]
	fn past_plain() {
		let tests = vec![
			(miru(), "みた"),
			(kau(), "かった"),
			(matsu(), "まった"),
			(kaeru(), "かえった"),
			(yomu(), "よんだ"),
			(asobu(), "あそんだ"),
			(shinu(), "しんだ"),
			(kaku(), "かいた"),
			(nugu(), "ぬいだ"),
			(sagasu(), "さがした"),
		];
		assert_conjugations(tests, Past, Plain, Immediate);
	}

	fn assert_conjugations(tests: Vec<(Verb, &str)>, tense: Tense, audience: Audience, mode: Mode) {
		tests.iter().for_each(|(verb, expected)| {
			let conjugation = verb.conjugate(tense, audience, mode);
			assert_eq!(conjugation, expected.to_string());
		})
	}

	#[test]
	fn present_polite() {
		let tests = vec![
			(miru(), "みます"),
			(kau(), "かいます"),
			(matsu(), "まちます"),
			(kaeru(), "かえります"),
			(yomu(), "よみます"),
			(asobu(), "あそびます"),
			(shinu(), "しにます"),
			(kaku(), "かきます"),
			(nugu(), "ぬぎます"),
			(sagasu(), "さがします"),
		];
		assert_conjugations(tests, Present, Polite, Immediate);
	}

	#[test]
	fn past_polite_potential() {
		let tests = vec![
			(miru(), "みられました"),
			(kau(), "かえました"),
			(matsu(), "まてました"),
			(kaeru(), "かえれました"),
			(yomu(), "よめました"),
			(asobu(), "あそべました"),
			(shinu(), "しねました"),
			(kaku(), "かけました"),
			(nugu(), "ぬげました"),
			(sagasu(), "さがせました"),
		];
		assert_conjugations(tests, Past, Polite, Potential);
	}

	#[test]
	fn ru_verb() {
		let verb = miru();
		assert_eq!(verb.conjugate(Present, Plain, Immediate), "みる", "ru present plain");
		assert_eq!(verb.conjugate(Present, Polite, Immediate), "みます", "ru present polite");
		assert_eq!(verb.conjugate(Past, Plain, Immediate), "みた", "ru past plain");
		assert_eq!(verb.conjugate(Past, Polite, Immediate), "みました", "ru past polite");
		assert_eq!(verb.conjugate(Present, Plain, Potential), "みられる", "ru present plain potential");
		assert_eq!(verb.conjugate(Present, Polite, Potential), "みられます", "ru present polite potential");
		assert_eq!(verb.conjugate(Past, Plain, Potential), "みられた", "ru past plain potential");
		assert_eq!(verb.conjugate(Past, Polite, Potential), "みられました", "ru past polite potential");
	}

	#[test]
	fn u_verb() {
		let verb = kaeru();
		assert_eq!(verb.conjugate(Present, Plain, Immediate), "かえる", "u present plain");
		assert_eq!(verb.conjugate(Present, Polite, Immediate), "かえります", "u present polite");
		assert_eq!(verb.conjugate(Past, Plain, Immediate), "かえった", "u past plain");
		assert_eq!(verb.conjugate(Past, Polite, Immediate), "かえりました", "u past polite");
		assert_eq!(verb.conjugate(Present, Plain, Potential), "かえれる", "u present plain potential");
		assert_eq!(verb.conjugate(Present, Polite, Potential), "かえれます", "u present polite potential");
		assert_eq!(verb.conjugate(Past, Plain, Potential), "かえれた", "u past plain potential");
		assert_eq!(verb.conjugate(Past, Polite, Potential), "かえれました", "u past polite potential");
	}

	fn miru() -> Verb { Verb { kind: Kind::Ru, search: "みる".to_string() } }

	fn kau() -> Verb { Verb { kind: Kind::U, search: "かう".to_string() } }

	fn matsu() -> Verb { Verb { kind: Kind::U, search: "まつ".to_string() } }

	fn kaeru() -> Verb { Verb { kind: Kind::U, search: "かえる".to_string() } }

	fn yomu() -> Verb { Verb { kind: Kind::U, search: "よむ".to_string() } }

	fn asobu() -> Verb { Verb { kind: Kind::U, search: "あそぶ".to_string() } }

	fn shinu() -> Verb { Verb { kind: Kind::U, search: "しぬ".to_string() } }

	fn kaku() -> Verb { Verb { kind: Kind::U, search: "かく".to_string() } }

	fn nugu() -> Verb { Verb { kind: Kind::U, search: "ぬぐ".to_string() } }

	fn sagasu() -> Verb { Verb { kind: Kind::U, search: "さがす".to_string() } }
}

