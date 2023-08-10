use dioxus::prelude::*;

mod button_component;

#[derive(Props)]
pub struct ButtonProps<'a> {
    pub onclick: EventHandler<'a, MouseEvent>,
    pub children: Element<'a>,
}

pub fn Button<'a>(cx: Scope<'a, ButtonProps<'a>>) -> Element {
    cx.render(rsx!(
        button {
            "class": "px-6 py-3 text-white bg-blue-500 hover:bg-blue-700 rounded-md shadow-md",
            onclick:  move |event| cx.props.onclick.call(event),
            &cx.props.children
        }
    ))
}
