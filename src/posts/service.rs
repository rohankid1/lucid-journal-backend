use super::models::*;
use crate::validation::{validate_edit, validate_post, ResponseType};
use crate::AppState;
use crate::DB;
use actix_web::{
    delete, get, post, put,
    web::{self, post},
    HttpResponse, Responder,
};
use chrono::Local;

const AUTH_KEY: &str = env!("TOKEN");

#[get("/posts")]
async fn get_posts(data: web::Data<AppState>) -> impl Responder {
    HttpResponse::Ok().json(data.posts.lock().to_vec())
}

#[get("/posts/{id}")]
async fn get_post(_data: web::Data<AppState>, id: web::Path<i64>) -> impl Responder {
    let db = &mut DB.lock();
    let posts = db.deserialize().unwrap();
    let id = id.into_inner();

    for i in 0..posts.len() {
        if posts[i].id == id {
            return HttpResponse::Ok().json(posts.get(i).unwrap());
        }
    }

    HttpResponse::NotFound().body(format!("Post with supplied id of {id} could not be found"))
}

#[post("/posts/entries")]
async fn create_post(data: web::Data<AppState>, obj: web::Json<CreatePost>) -> impl Responder {
    println!("about to create post... first validating it.");
    
    match validate_post(&obj).await {
        ResponseType::BadRequest(msg) => {
            return HttpResponse::Unauthorized().body(msg.unwrap_or("Input error"))
        }
        ResponseType::Good => {}
    }

    if obj.auth_key != AUTH_KEY {
        return HttpResponse::Unauthorized().body("Incorrect authorisation key");
    }

    let date = Local::now();
    let date = date.format("%d/%m/%Y").to_string();

    let entries = &mut *data.posts.lock();
    let new_id = entries.len() + 1;
    // for i in 0..entries.len() {
    //     if entries[i].id > highest_id {
    //         highest_id = entries[i].id;
    //         break;
    //     }
    // }

    let mut description = obj.description.clone();
    description = description
        .replace("{name}", &obj.author)
        .replace("{postLen}", entries.len().to_string().as_str())
        .replace(
            "{myPostsLen}",
            entries
                .iter()
                .filter(|post| &post.author == &obj.author)
                .count()
                .to_string()
                .as_str(),
        );

    entries.push(Post {
        id: new_id as i64,
        date,
        author: obj.author.clone(),
        description,
        title: obj.title.clone(),
    });

    let db = &mut DB.lock();
    db.write_back(&entries).await.unwrap();

    HttpResponse::Ok().json(entries)
}

#[put("/posts/entries/{id}")]
async fn update_post(
    data: web::Data<AppState>,
    req: web::Path<i64>,
    with: web::Json<UpdatePost>,
) -> crate::Res<impl Responder> {
    if with.auth_key.as_str() != AUTH_KEY {
        return Ok(HttpResponse::Unauthorized());
    }

    match validate_edit(&with).await {
        ResponseType::Good => {}
        ResponseType::BadRequest(_) => {
            return Ok(HttpResponse::BadRequest());
        }
    }

    let db = &mut *DB.lock();
    db.read().await?;

    let mut entries = db.deserialize()?;
    let id = if let Some(post) = entries.iter().find(|post| post.id == *req) {
        post.id
    } else {
        return Ok(HttpResponse::NotFound());
    };

    for i in 0..entries.len() {
        if entries[i].id == id {
            if entries[i].author != with.author {
                return Ok(HttpResponse::MethodNotAllowed());
            }
            let date = Local::now();
            let date = date.format("%d/%m/%Y").to_string();
            entries[i].date = date;
            entries[i].title = with.title.clone();
            entries[i].description = with.description.clone();

            break;
        }
    }

    db.write_back_sync(&entries).await?;

    Ok(HttpResponse::Ok())
}

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(get_post)
        .service(get_posts)
        .service(create_post);
}
