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
            "class": "flex flex-col items-center space-y-4",
            h1 { "High-Five counter: {count}" }
            button {
                "class": "px-4 py-2 bg-amber-500 text-white border border-gray-300 hover:bg-amber-700 hover:border-amber-700",
                onclick: move |_| count += 1, "Up high!" }
            button {
                "class": "px-4 py-2 bg-sky-500 text-white border border-gray-300 hover:bg-sky-700 hover:border-sky-700",
                onclick: move |_| count -= 1, "Down low!" }
        }
    ))
}
