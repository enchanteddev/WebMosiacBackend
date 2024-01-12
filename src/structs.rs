use std::{collections::HashMap, fs};

use serde::{Serialize, Deserialize};
use serde_json;

#[derive(Serialize, Deserialize, Clone)]
pub struct Post {
    pub id: u32,
    pub name: String,
    pub body: String,
    pub likes: u32,
    pub dislikes: u32,
    pub author_id: u32
}

#[derive(Serialize, Clone)]
pub struct PostSmall{
    pub name: String,
    pub id: u32
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Comment{
    pub id: u32,
    pub body: String,
    pub author_id: u32,
    pub post_id: u32
}


#[derive(Serialize, Deserialize, Clone)]
pub struct Author{
    pub name: String,
    pub id: u32
}


#[derive(Serialize, Clone)]
pub struct AppData{
    pub posts: HashMap<u32, Post>,
    pub comments: HashMap<u32, Comment>,
    pub authors: HashMap<u32, Author>
}

#[derive(Serialize, Deserialize)]
pub struct RawJSON{
    pub posts: Vec<Post>,
    pub comments: Vec<Comment>,
    pub authors: Vec<Author>
}

pub fn load_data(path: &str) -> AppData {
    let data = fs::read_to_string(path)
        .expect(&format!("Unable to read file at: {}", path));
    let json: RawJSON = serde_json::from_str(&data)
        .expect("Incorrect JSON File or JSON structure. Could not Parse.");
    
    let post_map: HashMap<u32, Post> = json.posts.iter()
        .map(|x| {(x.id, x.clone())}).collect();
    
    let author_map: HashMap<u32, Author> = json.authors.iter()
        .map(|x| {(x.id, x.clone())}).collect();
    
    let comment_map: HashMap<u32, Comment> = json.comments.iter()
        .map(|x| {(x.id, x.clone())}).collect();

    AppData{
        posts: post_map,
        authors: author_map,
        comments: comment_map
    }
}

pub fn get_post_smalls(appdata: AppData) -> Vec<PostSmall> {
    appdata.posts.iter().map(|(_, x)| {PostSmall{
        id: x.id,
        name: x.name.clone()
    }}).collect()
}