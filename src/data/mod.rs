use verb::{Kind, Verb};

pub fn verbs() -> Vec<Verb> {
	include!("verbs.in")
}