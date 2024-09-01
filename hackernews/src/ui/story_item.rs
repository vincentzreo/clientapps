#![allow(non_snake_case)]
use crate::StoryItem;
use dioxus::prelude::*;

#[component]
pub fn StoryItem(item: StoryItem) -> Element {
    rsx! {
        li { class: "py-5 border-b px-3 transition hover:bg-indigo-100",
        a { href: "#", class: "flex justify-between items-center",
            h3 { class: "text-lg font-semibold", "{item.title}" }
            p { class: "text-md text-gray-400" }
        }
        div { class: "text-md italic text-gray-400",
            span {"{item.score} points by {item.by}  {item.time} | "}
            a { href: "#", "{item.kids.len()} comments" }
        }
        }
    }
}
