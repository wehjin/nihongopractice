use yew::{ComponentLink, Html, html, NodeRef};

use crate::{app, mdc, shadow, upgradeDom, utils};

use super::*;

pub fn page(model: &shadow::Model, link: &ComponentLink<app::Model>) -> Html {
	mdc::page("17.1 花見", Some("Back".to_uppercase()), link, model, content)
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
	let audio = mdc::audio::hidden("shadow-player", &shadow.audio_url, audio_ref);
	html! {
	<>
		{ audio }
		<div class="mdl-list">
			{shadow.lines.iter().enumerate().map(|(index, line)|row(index,line, link)).collect::<Html>()}
		</div>
	</>
	}
}

fn row(index: usize, line: &Line, link: &ComponentLink<app::Model>) -> Html {
	let button_link = link.to_owned();
	let button_label = format!("► {}", line.description);
	let button = mdc::button::raised(&button_label, move || button_link.send_message(app::Msg::Shadow(Msg::Play(index))));
	let speaker_classes = vec![
		"mdl-cell",
		"mdl-cell--1-col",
		"mdl-cell--hide-phone",
		"mdl-cell--middle",
		"mdc-typography-subtitle1"
	].join(" ");
	let description_classes = vec![
		"mdl-cell",
		"mdl-cell--4-col",
		"mdl-cell--7-col-tablet",
		"mdl-cell--11-col-desktop",
		"mdl-cell--middle"
	].join(" ");
	let row_classes = format!("mdl-list__item");
	html! {
		<div class=row_classes>
			<div class="mdl-list__item-primary-content mdl-grid">
				<div class=speaker_classes> {&line.speaker}</div>
				<div class=description_classes>{button}</div>
			</div>
		</div>
	}
}

fn authorizing(link: &ComponentLink<app::Model>) -> Html {
	let success_link = link.clone();
	let succeeded = mdc::button::flat_primary("Continue", move || success_link.send_message(app::Msg::Shadow(Msg::Authorized)));
	let failure_link = link.clone();
	let failed = mdc::button::flat_primary("Cancel", move || failure_link.send_message(app::Msg::Quit));
	html! {
		< div class = "mdl-grid" >
			< div class ="mdl-card mdl-shadow--2dp mdl-cell mdl-cell--4-col" >
				< div class = "mdl-card__title mdl-color--primary" style = "color:#fff">
				< h2 class = "mdl-card__title-text" >{"Authorizing"}< / h2 >
			< / div >
			< div class = "mdl-card__title mdl-color--primary" style = "color:#fff" >
				< div class= "mdl-card__media" >
					< iframe id = "shadow-frame" height ="120" scrolling = "no" src = AUTHORIZATION_URL
					    onload = "document.getElementById('auth-progress').style.display = 'none'"
					    onerror = "alert(event.type)"
					/>
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
