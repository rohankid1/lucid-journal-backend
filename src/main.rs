mod db;
mod posts;
mod validation;

use actix_web::{web, App, HttpResponse, HttpServer};
use actix_cors::Cors;
use db::Db;
use parking_lot::Mutex;
use posts::service::config;
use serde::{Deserialize, Serialize};
use std::sync::Arc;

use posts::Post;

type Res<T> = std::io::Result<T>;

lazy_static::lazy_static! {
    pub static ref DB: Arc<Mutex<Db>> = {
        Arc::new(
            Mutex::new(
                Db::new("db.json")
            )
        )
    };
}

struct AppState {
    posts: Mutex<Vec<Post>>,
}

async fn health_check() -> HttpResponse {
    if *&DB.lock().read().await.is_ok() {
        HttpResponse::Ok().body("health status is OKAY")
    } else {
        HttpResponse::InternalServerError().body("JSON store file is not available at the moment.")
    }
}

#[actix_web::main]
async fn main() -> Res<()> {
    {
        DB.lock().read().await?;
    }
    
    let posts = &DB.lock().deserialize()?;

    let app_state = web::Data::new(AppState {
        posts: Mutex::new(posts.clone()),
    });

    HttpServer::new(move || {
        App::new()
            .wrap(
                Cors::default()
                    .allowed_methods(vec!["GET", "POST"])
                    .allow_any_origin()
                    .allow_any_header()
                    .max_age(3600)
            )
            .app_data(app_state.clone())
            .configure(config)
            .route("/health", web::get().to(health_check))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
