use dioxus::prelude::*;

#[derive(Props)]
pub struct QuestionProps<'a> {
    pub children: Element<'a>,
}

pub fn Question<'a>(cx: Scope<'a, QuestionProps<'a>>) -> Element {
    /*
    <div >
      <!-- Content of the card goes here -->
      <h2 class="text-lg font-semibold mb-4">Card Title</h2>
      <p class="text-gray-600">Lorem ipsum dolor sit amet, consectetur adipiscing elit...</p>
      <button class="mt-4 px-4 py-2 bg-blue-500 hover:bg-blue-700 text-white rounded-md shadow-md">
        Read More
      </button>
    </div>
     */
    cx.render(rsx!(
        div {
            "class": "w-full h-screen/3 bg-white rounded-lg shadow-lg p-6",
            &cx.props.children
        }
    ))
}
