use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct PostMetadata {
    pub title: String,
    pub date: String,
    pub author: String,
    pub tags: Vec<String>,
    pub summary: String,
}
