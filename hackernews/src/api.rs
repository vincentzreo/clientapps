#[allow(unused)]
use anyhow::Result;
use futures::future::join_all;

use crate::{Comment, StoryItem, StoryPageData};

const MAX_STORIES: usize = 50;

pub async fn get_top_stories(mut n: usize) -> Result<Vec<StoryItem>> {
    n = n.min(MAX_STORIES);
    let url = "https://hacker-news.firebaseio.com/v0/topstories.json";
    let ids: Vec<i64> = reqwest::get(url).await?.json().await?;
    let story_futures = ids.into_iter().take(n).map(get_story_item_by_id);

    let stories = join_all(story_futures)
        .await
        .into_iter()
        .filter_map(|item| item.ok())
        .collect();
    Ok(stories)
}

pub async fn get_story_item_by_id(id: i64) -> Result<StoryItem> {
    let url = format!("https://hacker-news.firebaseio.com/v0/item/{}.json", id);
    let item: StoryItem = reqwest::get(url).await?.json().await?;
    Ok(item)
}

pub async fn get_story_comments(item: StoryItem) -> Result<StoryPageData> {
    let comment_future = item.kids.iter().map(|id| get_comments_by_id(*id));
    let comments = join_all(comment_future)
        .await
        .into_iter()
        .filter_map(|comment| comment.ok())
        .collect();
    Ok(StoryPageData { item, comments })
}

pub async fn get_comments_by_id(id: i64) -> Result<Comment> {
    let url = format!("https://hacker-news.firebaseio.com/v0/item/{}.json", id);
    let comment: Comment = reqwest::get(url).await?.json().await?;
    Ok(comment)
}

// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[tokio::test]
//     async fn get_top_stories_should_work() {
//         let stories = get_top_stories(3).await.unwrap();
//         // stories <= MAX_STORIES
//         assert!(stories.len() <= 3);
//     }

//     #[tokio::test]
//     async fn get_comments_by_id_should_work() {
//         let story = get_top_stories(3).await.unwrap().pop().unwrap();
//         let id = story.kids[0];
//         let comment = get_comments_by_id(id).await.unwrap();
//         assert_eq!(comment.id, id);
//     }
// }
