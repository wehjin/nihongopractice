use verb::{Kind, Verb};

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