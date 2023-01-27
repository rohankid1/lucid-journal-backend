use crate::{
    posts::{CreatePost, UpdatePost},
    Res,
};
use actix_web::HttpResponse;
use validator::validate_range;

pub enum ResponseType {
    BadRequest(Option<&'static str>),
    Good,
}

pub async fn validate_post(obj: &CreatePost) -> ResponseType {
    let CreatePost {
        author,
        auth_key: _,
        description,
        title,
    } = obj;

    let (title, desc, user) = (
        validate_range(title.len(), Some(5), Some(40)),
        validate_range(description.len(), Some(10), None),
        validate_range(author.len(), Some(3), Some(20)),
    );

    if !title {
        return ResponseType::BadRequest(Some(
            "The title must have at least 5 characters, with a maximum of 40",
        ));
    }

    if !desc {
        return ResponseType::BadRequest(Some("The description must have at least 10 characters"));
    }

    if !user {
        return ResponseType::BadRequest(Some(
            "The author's name must be between 3 and 20 characters long",
        ));
    }

    ResponseType::Good
}

pub async fn validate_edit(obj: &UpdatePost) -> ResponseType {
    let (title, desc, user) = (
        validate_range(obj.title.len(), Some(5), Some(40)),
        validate_range(obj.description.len(), Some(10), None),
        validate_range(obj.author.len(), Some(3), Some(20)),
    );

    if !title {
        return ResponseType::BadRequest(Some(
            "The title must have at least 5 characters, with a maximum of 40",
        ));
    }

    if !desc {
        return ResponseType::BadRequest(Some("The description must have at least 10 characters"));
    }

    if !user {
        return ResponseType::BadRequest(Some(
            "The author's name must be between 3 and 20 characters long",
        ));
    }

    ResponseType::Good
}