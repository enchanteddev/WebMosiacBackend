use serde::Serialize;

#[derive(Serialize)]
pub struct Post {
    pub id: u32,
    pub name: String,
    pub body: String,
    pub likes: u32,
    pub dislikes: u32,
    pub author_id: u32
}

#[derive(Serialize)]
pub struct PostSmall{
    pub name: String,
    pub id: u32
}

#[derive(Serialize)]
pub struct Comment{
    pub id: u32,
    pub body: String,
    pub author_id: u32,
    pub post_id: u32
}


#[derive(Serialize)]
pub struct Author{
    pub name: String,
    pub id: u32
}


#[derive(Serialize)]
pub struct JsonRawData{
    pub posts: Vec<Post>,
    pub comments: Vec<Comment>,
    pub authors: Vec<Author>,
}