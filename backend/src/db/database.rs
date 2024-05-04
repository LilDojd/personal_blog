use surrealdb::engine::remote::ws::{Client, Ws};
use surrealdb::opt::auth::Root;
use surrealdb::{Error, Surreal};

use crate::models::BlogPost;

#[derive(Clone)]
pub struct Database {
    pub client: Surreal<Client>,
    pub name_space: String,
    pub db_name: String,
}

impl Database {
    pub async fn init() -> Result<Self, Error> {
        let client = Surreal::new::<Ws>("127.0.0.1:8081").await?;
        client
            .signin(Root {
                username: "root",
                password: "changeme123",
            })
            .await?;
        client.use_ns("surreal").use_db("back").await.unwrap();
        Ok(Database {
            client,
            name_space: String::from("surreal"),
            db_name: String::from("back"),
        })
    }

    pub async fn get_all_blogposts(&self) -> Option<Vec<BlogPost>> {
        let result = self.client.select("back").await;
        match result {
            Ok(all_blogposts) => Some(all_blogposts),
            Err(_) => None,
        }
    }

    pub async fn add_blogpost(&self, new_blogpost: BlogPost) -> Option<BlogPost> {
        let created_blogpost = self
            .client
            .create(("back", new_blogpost.uuid.clone()))
            .content(new_blogpost)
            .await;

        match created_blogpost {
            Ok(created) => created,
            Err(_) => None,
        }
    }
}
