use dioxus::prelude::*;
use dioxus_router::prelude::*;

use crate::components::question_component::Question;
use crate::components::Button;
use crate::Routes;

pub fn AppLayout(cx: Scope) -> Element {
    render! {
        Sidebar {}
        Outlet::<Routes> {}
        Footer {}
    }
}

pub fn Sidebar(cx: Scope) -> Element {
    render! {
        div {
            "This is the sidebar"
            Link {
                to: Routes::About {},
                "About"
            }
            Link {
                to: Routes::Quiz {},
                "Quiz"
            }
        }
    }
}

pub fn Footer(cx: Scope) -> Element {
    render! {
        div {
            "This is the footer"
        }
    }
}

pub fn Quiz(cx: Scope) -> Element {
    let mut count = use_state(cx, || 0);
    render! {
        div {
            "class": "flex flex-col items-center space-y-4",
            Question {
                "High-Five counter: {count}"
            }
            Button {
                onclick: move |_| count += 1, "Up high!" }
            Button {
                onclick: move |_| count -= 1, "Down low!" }
        }
    }
}

pub fn About(cx: Scope) -> Element {
    render! {
        div {
            p {
                "This is the about page."
            }
            p {
                "It contains info."
            }
        }
    }
}
