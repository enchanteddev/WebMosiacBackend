use std::collections::HashMap;

use actix_web::{get, web::{self, Data}, App, HttpResponse, HttpServer, Responder};
mod structs;


#[get("/post")]
async fn post() -> impl Responder {
    let post = structs::Post{
        id: 0,
        name: String::from("Global Warming is Bad"), 
        body: String::from("lorem iiawdja wd jawd awd awd awd awd w awadw dawd w"), 
        likes: 12, 
        dislikes: 2, 
        author_id: 1 
    };

    HttpResponse::Ok().json(post)
}

#[get("/posts")]
async fn posts(appdata: web::Data<Dummy>) -> impl Responder {
    let mut post_map: HashMap<String, Vec<structs::PostSmall>> = HashMap::new();
    let posts = appdata.get_ref().data.clone();
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

struct Dummy{
    pub data: Vec<structs::PostSmall>
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let appdata = structs::load_data("./src/data.json");
    let postsmalls = structs::get_post_smalls(appdata);
    HttpServer::new(move || {
        App::new()
            .app_data(Data::new(Dummy{data: postsmalls.clone()}))
            .service(post)
            .service(posts)
            .route("/", web::get().to(welcome))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}