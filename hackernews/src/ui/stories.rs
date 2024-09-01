#![allow(non_snake_case)]
use crate::{
    api::{get_story_comments, get_top_stories},
    ui::CommentsState,
    StoryItem, StoryPageData,
};
use dioxus::prelude::*;
use dioxus_logger::tracing::info;

#[component]
pub fn Stories() -> Element {
    let stories = use_resource(move || get_top_stories(20));
    match &*stories.read_unchecked() {
        Some(Ok(stories)) => rsx! {
            ul { class: "mt-6",
                for item in stories {
                    StoryItem { item: item.clone() }
                }}
        },
        Some(Err(e)) => rsx! {
            div {
                class: "mt-6 text-red-500",
                p { "Failed to fetch stories" }
                p{"Error: {e}"}
            }
        },
        None => rsx! {
            div { class: "mt-6", p {"Loading stories..."} }
        },
    }
}

#[component]
pub fn StoryItem(item: StoryItem) -> Element {
    let state = use_context::<Signal<CommentsState>>();
    // cache of the already loaded comments: Option<StoryPageData>
    let full_story = use_signal(|| None);
    rsx! {
        li { class: "py-5 border-b px-3 transition hover:bg-indigo-100",
        a { href: "#", class: "flex justify-between items-center",
            h3 { class: "text-lg font-semibold", "{item.title}" }
            p { class: "text-md text-gray-400" }
        }
        div { class: "text-md italic text-gray-400",
            span {"{item.score} points by {item.by}  {item.time} | "}
            a { href: "#",
            prevent_default: "onclick",
            onclick: move |_| {
                info!("Clicked on story: {}", item.title);
                load_comments(state, full_story, item.clone())
            },
            "{item.kids.len()} comments" }
        }
        }
    }
}

async fn load_comments(
    mut state: Signal<CommentsState>,
    mut full_story: Signal<Option<StoryPageData>>,
    item: StoryItem,
) {
    // if already loaded, return
    if let Some(story_data) = full_story.as_ref() {
        *state.write() = CommentsState::Loaded(story_data.clone());
        return;
    }
    // if not loaded, set state to loading
    *state.write() = CommentsState::Loading;

    if let Ok(story_data) = get_story_comments(item).await {
        *state.write() = CommentsState::Loaded(story_data.clone());
        *full_story.write() = Some(story_data);
    }
}
