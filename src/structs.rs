use serde::Serialize;

#[derive(Serialize)]
pub struct Post {
    pub id: i32,
    pub name: String,
    pub body: String,
    pub likes: i32,
    pub dislikes: i32,
    pub author_id: i32
}

#[derive(Serialize)]
pub struct PostSmall{
    pub name: String,
    pub id: i32
}

#[derive(Serialize)]
pub struct Posts{
    pub posts: Vec<PostSmall>
}