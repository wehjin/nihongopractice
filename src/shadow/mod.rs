use yew::NodeRef;

pub mod view;

#[derive(Clone, Debug)]
pub enum Model {
	Loading,
	Loaded { audio_ref: NodeRef },
}

impl Model {
	pub fn start() -> Self { Model::Loading }

	pub fn load(&self) -> Self {
		Model::Loaded { audio_ref: Default::default() }
	}
}

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub enum Msg {
	Start,
	FinishedLoading,
}
