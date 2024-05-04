use actix_web::{
    get, patch, post,
    web::Json,
    web::{Data, Path},
    App, HttpResponse, HttpServer, Responder,
};
use uuid::Uuid;
use validator::Validate;

use crate::models::post::{NewBlogPost, UpdateBlogPost};
use crate::{db::Database, models::BlogPost};
mod db;
mod models;

#[get("/")]
async fn get_home() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

#[get("/blog")]
async fn get_blogs(db: Data<Database>) -> impl Responder {
    let blogposts = db.get_all_blogposts().await;
    match blogposts {
        Some(found_blogs) => HttpResponse::Ok().body(format!("{:?}", found_blogs)),
        None => HttpResponse::Ok().body("Error!"),
    }
}

#[post("/postblog")]
async fn post_blog(body: Json<NewBlogPost>, db: Data<Database>) -> impl Responder {
    let is_valid = body.validate();
    match is_valid {
        Ok(_) => {
            let post_title = body.title.clone();
            let mut buffer = Uuid::encode_buffer();
            let new_uuid = Uuid::new_v4().simple().encode_lower(&mut buffer);

            let new_blogpost = db
                .add_blogpost(BlogPost::new(String::from(new_uuid), post_title))
                .await;

            match new_blogpost {
                Some(created) => {
                    HttpResponse::Ok().body(format!("Created new blogpost: {:?}", created))
                }
                None => HttpResponse::Ok().body("Error posting blogpost"),
            }
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
        .expect("Error connecting to database");

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
