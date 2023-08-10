use dioxus::prelude::*;

#[derive(Props)]
pub struct QuestionProps<'a> {
    pub children: Element<'a>,
}

pub fn Question<'a>(cx: Scope<'a, QuestionProps<'a>>) -> Element {
    cx.render(rsx!(
        div {
            "class": "w-full bg-white rounded-lg shadow-lg p-6",
            &cx.props.children
        }
    ))
}
