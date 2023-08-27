// #![feature(unboxed_closures)]
#![allow(non_snake_case)]

// import the prelude to get access to the `rsx!` macro and the `Scope` and `Element` types
use crate::components::question_component::Question;
use crate::components::Button;
use crate::layout::About;
use crate::layout::AppLayout;
use crate::layout::Quiz;
use dioxus::prelude::*;
use dioxus_router::prelude::*;
use serde::{Deserialize, Serialize};

mod components;
mod layout;

#[cfg(debug_assertions)]
const BASE_API: String = "http://localhost:3000/";
#[cfg(not(debug_assertions))]
const BASE_API: String = "/api";

#[derive(Clone, Routable, PartialEq, Eq, Serialize, Deserialize, Debug)]
enum Routes {
    #[layout(AppLayout)]
    #[route("/")]
    #[redirect("/about", || Routes::About {})]
    About {},

    #[route("/quiz")]
    Quiz {},
}

fn main() {
    // launch the web app
    dioxus_web::launch(App);
}

// create a component that renders a div with the text "Hello, world!"
fn App(cx: Scope) -> Element {
    cx.render(rsx!(Router::<Routes> {}))
}
