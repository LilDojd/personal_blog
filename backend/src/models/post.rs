// post.rs
use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Validate, Deserialize, Serialize)]
pub(crate) struct NewBlogPost {
    #[validate(length(min = 1, message = "Title is required"))]
    pub(crate) title: String,
}

#[derive(Validate, Deserialize, Serialize, Debug)]
pub(crate) struct BlogPost {
    pub(crate) uuid: String,
    pub(crate) title: String,
}

impl BlogPost {
    pub fn new(uuid: String, title: String) -> BlogPost {
        BlogPost { uuid, title }
    }
}

#[derive(Validate, Deserialize, Serialize)]
pub(crate) struct UpdateBlogPost {
    pub(crate) uuid: String,
}
