use dioxus::html::footer;
use dioxus::prelude::*;
use dioxus_router::prelude::*;

use crate::components::question_component::Question;
use crate::components::Button;
use crate::{Routes, BASE_API};

pub fn AppLayout(cx: Scope) -> Element {
    render! {
        div {
            "class": "h-screen flex flex-row",
            div {
                "class" : "w-32",
                Sidebar {}
            }
            div {
                "class": "flex-row flex-grow right",
                div {
                    Outlet::<Routes> {}
                }
                Footer {}
            }
        }
    }
}

pub fn Sidebar(cx: Scope) -> Element {
    render! {
        div {
            "class": "bg-gray-800 text-white flex flex-col w-32 h-screen",
            span {
                "class": "p-4",
                "Laerning Tool"
            }
            span {
                "class": "p-4",
                Link {
                    to: Routes::About {},
                    "About"
                },
            },
            span {
                "class": "p-4",
                Link {
                    to: Routes::Quiz {},
                    "Quiz"
                }
            }
        }
    }
}

pub fn Footer(cx: Scope) -> Element {
    render! {
        div {
            "class": "relative h-5",
            footer {
                "class": "bottom",
                div {
                    "This is the footer"
                }
            }
        }
    }
}

pub fn Quiz(cx: Scope) -> Element {
    let mut count = use_state(cx, || 0);
    let url = BASE_API;
    render! {
        div {
            "class": "flex flex-col items-center space-y-4",
            "The base url is {url}"
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
