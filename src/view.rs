use yew::{ComponentLink, html, Html};

use crate::app::{Model, Msg};

pub fn answer_button(link: &ComponentLink<Model>) -> Html {
	html! {
	<button onclick=link.callback(|_| Msg::ShowAnswer)>{ "Show Answer" }</button>
	}
}