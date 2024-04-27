// post.rs
use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Validate, Deserialize, Serialize)]
pub(crate) struct NewBlogPost {
    #[validate(length(min = 1, message = "Title is required"))]
    pub(crate) title: String,
}

pub(crate) struct BlogPost;

#[derive(Validate, Deserialize, Serialize)]
pub(crate) struct UpdateBlogPost {
    pub(crate) uuid: String,
}
