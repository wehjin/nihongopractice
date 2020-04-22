#[derive(Clone, Debug)]
pub struct Shadow {
	pub title: String,
	pub lines: Vec<Line>,
	pub audio_url: String,
}

#[derive(Clone, Debug)]
pub struct Line {
	pub speaker: String,
	pub description: String,
	pub start: f64,
	pub end: f64,
}
