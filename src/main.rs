use std::collections::HashMap;
use actix_web::{get, web::{self, Data, Query}, App, HttpResponse, HttpServer, Responder, HttpRequest};
mod structs;


#[get("/post")]
async fn post(req: HttpRequest, appdata: web::Data<PreProcessedAppData>) -> impl Responder {
    let query_str = req.query_string();
    let query_result = Query::<HashMap<String, u32>>::from_query(query_str);

    let query = match query_result {
        Ok(q) => q,
        Err(_) => return HttpResponse::BadRequest().body("Bad query string")
    };

    let post_id = match query.get("id") {
        Some(id) => id,
        None => return HttpResponse::BadRequest().body("'id' parameter not found")
    };

    let post_map = &appdata.get_ref().posts;
    let post_result = post_map.get(post_id);
    let post = match post_result {
        Some(p) => p,
        None => return HttpResponse::NotFound().body("Incorrect Post Id")
    };
    HttpResponse::Ok().json(post)
}

#[get("/author")]
async fn author(req: HttpRequest, appdata: web::Data<PreProcessedAppData>) -> impl Responder {
    let query_str = req.query_string();
    let query_result = Query::<HashMap<String, u32>>::from_query(query_str);

    let query = match query_result {
        Ok(q) => q,
        Err(_) => return HttpResponse::BadRequest().body("Bad query string")
    };

    let author_id = match query.get("id") {
        Some(id) => id,
        None => return HttpResponse::BadRequest().body("'id' parameter not found")
    };

    let author_map = &appdata.get_ref().authors;
    let author_result = author_map.get(author_id);
    let author = match author_result {
        Some(p) => p,
        None => return HttpResponse::NotFound().body("Incorrect Author Id")
    };
    HttpResponse::Ok().json(author)
}

#[get("/comments")]
async fn comments(req: HttpRequest, appdata: web::Data<PreProcessedAppData>) -> impl Responder {
    let query_str = req.query_string();
    let query_result = Query::<HashMap<String, u32>>::from_query(query_str);

    let query = match query_result {
        Ok(q) => q,
        Err(_) => return HttpResponse::BadRequest().body("Bad query string")
    };

    let post_id = match query.get("post_id") {
        Some(id) => id,
        None => return HttpResponse::BadRequest().body("'post_id' parameter not found")
    };

    let comment_map = &appdata.get_ref().comments;
    let comment_result = comment_map.get(post_id);
    let comment = match comment_result {
        Some(p) => p,
        None => return HttpResponse::NotFound().body("No Comment found for this Post Id")
    };
    HttpResponse::Ok().json(comment)
}

#[get("/posts")]
async fn posts(appdata: web::Data<PreProcessedAppData>) -> impl Responder {
    let mut post_map: HashMap<String, Vec<structs::PostSmall>> = HashMap::new();
    let posts = appdata.get_ref().postsmalls.clone();
    post_map.insert(String::from("posts"), posts);

    HttpResponse::Ok().json(post_map)
}

async fn welcome() -> impl Responder {
    let apis: Vec<String> = vec!["/posts".into(), "/post".into(), "/about".into(), "/comments".into(), "/user".into()];
    let mut html_apis: String = "".into();
    for api in apis {
        html_apis += &("<a href='".to_owned() + &api + "'>" + &api + "</a><br>");
    }
    HttpResponse::Ok().body("<h1>Hello There.</h1>Here are the links to various API backends.<br>".to_owned() + &html_apis)
}

struct PreProcessedAppData{
    pub postsmalls: Vec<structs::PostSmall>,
    pub posts: HashMap<u32, structs::Post>,
    pub comments: HashMap<u32, Vec<structs::Comment>>,
    pub authors: HashMap<u32, structs::Author>
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let appdata = structs::load_data("./src/data.json");
    let postsmalls = structs::get_post_smalls(appdata.clone());
    HttpServer::new(move || {
        App::new()
            .app_data(Data::new(PreProcessedAppData{
                postsmalls: postsmalls.clone(),
                posts: appdata.posts.clone(),
                authors: appdata.authors.clone(),
                comments: appdata.comments.clone()
            }))
            .service(post)
            .service(posts)
            .service(comments)
            .route("/", web::get().to(welcome))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}