use yew::{ComponentLink, Html, html, NodeRef};

use crate::{app, mdc, shadow, upgradeDom, utils};

use super::*;

pub fn page(model: &shadow::Model, link: &ComponentLink<app::Model>) -> Html {
	mdc::page("Shadow 17.1", Some("Back".to_uppercase()), link, model, content)
}

fn content(link: &ComponentLink<app::Model>, model: &shadow::Model) -> Html {
	match model {
		Model::Authorizing => {
			utils::request_animation_frame(move || upgradeDom());
			authorizing(link)
		}
		Model::Authorized {
			audio_ref,
			shadow
		} => authorized(audio_ref, shadow, link),
	}
}

fn authorized(audio_ref: &NodeRef, shadow: &Shadow, link: &ComponentLink<app::Model>) -> Html {
	let audio = mdc::audio::visible("shadow-player", &shadow.audio_url, audio_ref);
	let play_link = link.clone();
	let play = mdc::button::flat("Play", move || play_link.send_message(app::Msg::Shadow(Msg::Play(0))));
	html! {
	<>
	<div class="mdl-grid">
		<div class="mdl-cell mdl-cell--4-col-phone mdl-cell--8-desktop mdl-cell--2-offset-desktop mdl-cell--6-col-tablet mdl-cell--1-offset-tablet">
		<table class="mdl-data-table mdl-js-data-table" style="width:100%;">
			<thead>
				<tr class="mdl-color--blue-grey-100">
					<th></th>
					<th class="mdl-data-table__cell--non-numeric">{&shadow.title}</th>
					<th class="mdl-data-table__cell--non-numeric"></th>
				</tr>
			</thead>
			<tbody>
				{shadow.lines.iter().enumerate().map(|(index, line)|row(index,line, link)).collect::<Html>()}
			</tbody>
		</table>
		</div>
	</div>
	{ audio }
	{play}
	</>
	}
}

fn row(index: usize, line: &Line, link: &ComponentLink<app::Model>) -> Html {
	let callback = link.callback(move |_| app::Msg::Shadow(Msg::Play(index)));
	html! {
	<tr onclick=callback>
		<td class="mdl-data-table__cell--non-numeric">{index+1}</td>
		<td class="mdl-data-table__cell--non-numeric mdl-color-text--accent">{&line.description}</td>
		<td class="mdl-data-table__cell--non-numeric">{&line.speaker}</td>
	</tr>
	}
}

fn authorizing(link: &ComponentLink<app::Model>) -> Html {
	let success_link = link.clone();
	let succeeded = mdc::button::flat("Continue", move || success_link.send_message(app::Msg::Shadow(Msg::Authorized)));
	let failure_link = link.clone();
	let failed = mdc::button::flat("Cancel", move || failure_link.send_message(app::Msg::Quit));
	html! {
		< div class = "mdl-grid" >
			< div class ="mdl-card mdl-shadow--2dp mdl-cell mdl-cell--4-col" >
				< div class = "mdl-card__title mdl-color--primary" style = "color:#fff">
				< h2 class = "mdl-card__title-text" >{"Authorizing"}< / h2 >
			< / div >
			< div class = "mdl-card__title mdl-color--primary" style = "color:#fff" >
				< div class= "mdl-card__media" >
					< iframe
					id = "shadow-frame" height ="120" scrolling = "no" src = AUTHORIZATION_URL
					onload = "document.getElementById('auth-progress').style.display = 'none'"
					/ >
				< / div >
			< / div >
			< div id = "auth-progress" class = "mdl-progress mdl-js-progress mdl-progress__indeterminate mdl-card__border" > </ div >
				< div class = "mdl-card__actions mdl-card--border" >
					<div class = "mdl-card__supporting-text" >
						{"Continue when the pink section turns white. Otherwise cancel."}
					< / div >
				< / div >
				< div class = "mdl-card__actions mdl-card--border" >
					{succeeded}
					{failed}
				< / div >
			< / div >
		< /div >
	}
}

const AUTHORIZATION_URL: &str = "https://www.ccsf.edu/Departments/Language_Center/oll/japanese/ganbaroo_2/review.html";
