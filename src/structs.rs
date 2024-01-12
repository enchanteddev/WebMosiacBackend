use std::collections::HashMap;

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
pub struct AppData{
    pub posts: HashMap<u32, Post>,
    pub comments: HashMap<u32, Comment>,
    pub authors: HashMap<u32, Author>,
}