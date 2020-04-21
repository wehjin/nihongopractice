use yew::NodeRef;

pub mod view;

pub struct Model {
	pub audio_ref: NodeRef,
}

impl Default for Model {
	fn default() -> Self {
		Model {
			audio_ref: Default::default()
		}
	}
}
