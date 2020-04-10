use crate::utils::{drop_last_char, last_char};

pub use self::core::*;

mod core;

#[derive(Clone, PartialEq, Debug)]
pub struct Verb {
	pub kind: Kind,
	pub search: String,
	pub english: String,
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

	pub fn translate(&self, tense: Tense, mode: Mode) -> String {
		match (mode, tense) {
			(Mode::Immediate, Tense::Present) => format!("will {}", self.english),
			(Mode::Immediate, Tense::Past) => format!("did {}", self.english),
			(Mode::Potential, Tense::Present) => format!("can {}", self.english),
			(Mode::Potential, Tense::Past) => format!("could {}", self.english),
		}
	}
}

fn potential(verb: &str, kind: Kind) -> String {
	let dropped_u = drop_last_char(verb);
	let new_ending = potential_ending(verb, kind);
	format!("{}{}", dropped_u, new_ending)
}

fn potential_ending(verb: &str, kind: Kind) -> String {
	match kind {
		Kind::Ru => "られる",
		Kind::U => match last_char(verb).as_str() {
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
		},
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
		Kind::U if verb.ends_with("行く") || verb == "いく" || verb.ends_with("ていく") => "った",
		Kind::U => match last_char(verb).as_str() {
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
		},
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
			(iku(), "行った"),
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
			(iku(), "行きます"),
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

	#[test]
	fn translate() {
		let verb = miru();
		let tests = vec![
			(Present, Immediate, "will see"),
			(Present, Potential, "can see"),
			(Past, Immediate, "did see"),
			(Past, Potential, "could see")
		];
		tests.iter().for_each(|(tense, mode, expected)| {
			let translation = verb.translate(*tense, *mode);
			assert_eq!(translation.as_str(), *expected);
		})
	}

	fn miru() -> Verb { Verb { kind: Kind::Ru, search: "みる".to_string(), english: "see".to_string() } }

	fn kau() -> Verb { Verb { kind: Kind::U, search: "かう".to_string(), english: "buy".to_string() } }

	fn matsu() -> Verb { Verb { kind: Kind::U, search: "まつ".to_string(), english: "wait".to_string() } }

	fn kaeru() -> Verb { Verb { kind: Kind::U, search: "かえる".to_string(), english: "go home".to_string() } }

	fn yomu() -> Verb { Verb { kind: Kind::U, search: "よむ".to_string(), english: "read".to_string() } }

	fn asobu() -> Verb { Verb { kind: Kind::U, search: "あそぶ".to_string(), english: "play".to_string() } }

	fn shinu() -> Verb { Verb { kind: Kind::U, search: "しぬ".to_string(), english: "die".to_string() } }

	fn kaku() -> Verb { Verb { kind: Kind::U, search: "かく".to_string(), english: "write".to_string() } }

	fn iku() -> Verb { Verb { kind: Kind::U, search: "行く".to_string(), english: "go".to_string() } }

	fn nugu() -> Verb { Verb { kind: Kind::U, search: "ぬぐ".to_string(), english: "disrobe".to_string() } }

	fn sagasu() -> Verb { Verb { kind: Kind::U, search: "さがす".to_string(), english: "search".to_string() } }
}

