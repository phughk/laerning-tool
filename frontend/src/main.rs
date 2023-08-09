#![allow(non_snake_case)]

use dioxus::html::div;
// import the prelude to get access to the `rsx!` macro and the `Scope` and `Element` types
use dioxus::prelude::rsx;
use dioxus::prelude::Element;

use dioxus::prelude::dioxus_elements;
use dioxus::prelude::use_state;
use dioxus::prelude::Scope;

fn main() {
    // launch the web app
    dioxus_web::launch(App);
}

// create a component that renders a div with the text "Hello, world!"
fn App(cx: Scope) -> Element {
    let mut count = use_state(cx, || 0);

    cx.render(rsx!(
        div {
            h1 { "High-Five counter: {count}" }
            button { onclick: move |_| count += 1, "Up high!" }
            button { onclick: move |_| count -= 1, "Down low!" }
        }
    ))
}
