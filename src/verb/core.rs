#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub enum Kind {
	U,
	Ru,
}

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub enum Tense {
	Present,
	Past,
}

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub enum Audience {
	Plain,
	Polite,
}

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub enum Mode {
	Immediate,
	Potential,
}
