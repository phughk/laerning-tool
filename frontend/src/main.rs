// #![feature(unboxed_closures)]
#![allow(non_snake_case)]

// import the prelude to get access to the `rsx!` macro and the `Scope` and `Element` types
use crate::components::Button;
use dioxus::prelude::*;

mod components;

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
            Button {
                onclick: move |_| count += 1, "Up high!" }
            Button {
                onclick: move |_| count -= 1, "Down low!" }
        }
    ))
}
