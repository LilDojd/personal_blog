use actix_web::{
    get, patch, post,
    web::Json,
    web::{Data, Path},
    App, HttpResponse, HttpServer, Responder,
};
use validator::Validate;

use crate::db::Database;
use crate::models::post::{NewBlogPost, UpdateBlogPost};
mod db;
mod models;

#[get("/")]
async fn get_home() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

#[get("/blog")]
async fn get_blogs() -> impl Responder {
    HttpResponse::Ok().body("Blogs!")
}

#[post("/postblog")]
async fn post_blog(body: Json<NewBlogPost>) -> impl Responder {
    let is_valid = body.validate();
    match is_valid {
        Ok(_) => {
            let post_title = body.title.clone();
            HttpResponse::Ok().body(format!("Post entered is {post_title}"))
        }
        Err(_) => HttpResponse::Ok().body("Post name required"),
    }
}

#[patch("/updateblog/{uuid}")]
async fn update_blog(update_blogpost_url: Path<UpdateBlogPost>) -> impl Responder {
    let uuid = update_blogpost_url.into_inner().uuid;
    HttpResponse::Ok().body(format!("Updating blog with uuid {uuid}"))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let db = Database::init()
        .await
        .expect("Error conntecting to database");

    let db_data = Data::new(db);

    HttpServer::new(move || {
        App::new()
            .app_data(db_data.clone())
            .service(get_blogs)
            .service(post_blog)
            .service(update_blog)
    })
    .bind("127.0.0.1:7878")?
    .run()
    .await
}
