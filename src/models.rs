use super::schema::posts;
use std::time::SystemTime;

#[derive(Serialize, Deserialize)]
#[derive(Queryable)]
pub struct Post {
    pub id: i32,
    pub title: String,
    pub body: String,
    pub published: bool,
}

#[derive(Insertable)]
#[derive(Deserialize)]
#[table_name="posts"]
pub struct NewPost<'a> {
    pub title: &'a str,
    pub body: &'a str,
}

#[derive(Deserialize)]
pub struct AddPost {
    pub title: String,
    pub body: String,
}


// Nullable SQL types should be an Option struct
#[derive(Serialize, Deserialize)]
#[derive(Queryable)]
pub struct Video {
    pub id: i32,
    pub video_id: String,
    pub title: String,
    pub description: Option<String>,
    pub duration: String,
    pub played: bool,
    pub added_on: SystemTime,
    pub played_on: Option<SystemTime>,
}


// Start with the Youtube models

#[allow(non_snake_case)]
#[derive(Deserialize)]
pub struct YoutubeVideoId {
    pub kind: String,
    pub videoId: String,
}

#[derive(Deserialize)]
pub struct YoutubeVideoThumbnail {
    pub url: String,
    pub width: i16,
    pub height: i16,
}

#[derive(Deserialize)]
pub struct YoutubeVideoThumbnails {
    pub default: YoutubeVideoThumbnail,
    pub medium: YoutubeVideoThumbnail,
    pub high: YoutubeVideoThumbnail,
}

#[allow(non_snake_case)]
#[derive(Deserialize)]
pub struct YoutubeVideoSnippet {
    pub publishedAt: String,
    pub channelId: String,
    pub title: String,
    pub description: String,
    pub thumbnails: YoutubeVideoThumbnails,
    pub channelTitle: String,
    pub liveBroadcastContent: String,
}

#[derive(Deserialize)]
pub struct YoutubeVideo {
    pub kind: String,
    pub etag: String,
    pub id: YoutubeVideoId,
    pub snippet: YoutubeVideoSnippet,
}

