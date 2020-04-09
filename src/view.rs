use yew::{ComponentLink, html, Html};

use crate::app::{Model, Msg};

pub fn answer_button(link: &ComponentLink<Model>) -> Html {
	html! {
	<button onclick=link.callback(|_| Msg::ShowAnswer)>{ "Show Answer" }</button>
	}
}

pub fn flat_button(label: &str, msg: Msg, link: &ComponentLink<Model>) -> Html {
	html! {
        <button class="mdl-button mdl-js-button" onclick=link.callback(move |_| msg)>{ label }</button>
	}
}

fn page(link: &ComponentLink<Model>, content: impl Fn(&ComponentLink<Model>) -> Html) -> Html {
	let title = "Verb Trainer";
	let link_text = "Link";
	html! {
<div class="mdl-layout mdl-js-layout">
  <header class="mdl-layout__header  mdl-layout__header--scroll">
    <div class="mdl-layout__header-row">
      <span class="mdl-layout-title">{title}</span>
      <div class="mdl-layout-spacer"></div>
      <nav class="mdl-navigation">
        <a class="mdl-navigation__link" href="">{link_text}</a>
      </nav>
    </div>
  </header>
  <div class="mdl-layout__drawer">
    <span class="mdl-layout-title">{title}</span>
    <nav class="mdl-navigation">
      <a class="mdl-navigation__link" href="">{link_text}</a>
    </nav>
  </div>
  <main class="mdl-layout__content">{content(link)}</main>
</div>
	}
}

pub fn idle(link: &ComponentLink<Model>) -> Html {
	page(link, idle_content)
}

fn idle_content(link: &ComponentLink<Model>) -> Html {
	html! {
<ol>
	<li>
	{ flat_button( "Recognition Game", Msg::Recognition, link)}
    </li>
</ol>
    }
}
