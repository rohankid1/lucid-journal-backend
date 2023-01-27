use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Post {
    pub id: i64,
    pub date: String,
    pub author: String,
    pub title: String,
    pub description: String,
}

#[derive(Deserialize, Clone)]
pub struct CreatePost {
    #[serde(rename = "authKey")]
    pub auth_key: String,
    #[serde(alias = "authorName")]
    pub author: String,
    pub title: String,
    pub description: String,
}

#[derive(Deserialize, Clone)]
pub struct UpdatePost {
    pub id: i64,
    // the author is not for updating,
    // but rather for checking if the post
    // had been created by the same user.
    pub author: String,
    pub auth_key: String,
    pub title: String,
    pub description: String,
}
