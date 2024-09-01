#![allow(non_snake_case)]
use comments::Comments;
use dioxus::prelude::*;
use stories::Stories;

use crate::StoryPageData;
mod comments;
mod stories;

#[derive(Debug, Clone)]
pub enum CommentsState {
    UnSet,
    Loading,
    Loaded(StoryPageData),
}

pub fn App() -> Element {
    use_context_provider(|| Signal::new(CommentsState::UnSet));
    rsx! {
        main { class: "flex w-full h-full shadow-lg rounded-3xl",
        section { class: "flex flex-col pt-3 w-4/12 bg-gray-50 h-full overflow-y-scroll",
            Stories {}
        }
        section { class: "w-8/12 px-4 flex flex-col bg-white rounded-r-3xl",
            Comments {}
        }
    }
    }
}
