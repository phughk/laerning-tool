use dioxus::prelude::*;

use crate::components::button_component::Button;

#[derive(Props, PartialEq)]
pub struct CreateGameProps {}

pub fn CreateGame(cx: Scope<CreateGameProps>) -> Element {
    let datasets = use_future(cx, (), |_| crate::api::get_datasets());
    let dataset_render = match datasets.value() {
        None => {
            rsx!(
                p {
                    "Loading datasets"
                }
            )
        }
        Some(Ok(d)) => {
            rsx!(
                p {
                    "Datasets"
                }
                p {
                    format!("{:?}", d)
                }
            )
        }
        Some(Err(e)) => {
            rsx!(
                p {
                    "Failed to load datasets {e:?}"
                }
            )
        }
    };
    cx.render(rsx!(
        div {
            "class": "w-full bg-white rounded-lg shadow-lg p-6",
            p {
                "Configure the game that you want to create"
            }
            Button {
                onclick: |_| {},
                "Start Game"
            }
            dataset_render
        }
    ))
}
