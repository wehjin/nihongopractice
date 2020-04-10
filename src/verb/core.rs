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

pub fn random_tense() -> Tense { random_pick(vec![Tense::Present, Tense::Past]) }

fn random_pick<T: Clone>(options: Vec<T>) -> T {
	let index: usize = random_u8() as usize % options.len();
	options[index].clone()
}

fn random_u8() -> u8 {
	let mut buffer = [0u8];
	getrandom::getrandom(&mut buffer).expect("Random number is required");
	buffer[0]
}

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub enum Audience {
	Plain,
	Polite,
}

pub fn random_audience() -> Audience { random_pick(vec![Audience::Plain, Audience::Polite]) }

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub enum Mode {
	Immediate,
	Potential,
}

pub fn random_mode() -> Mode { random_pick(vec![Mode::Immediate, Mode::Potential]) }


