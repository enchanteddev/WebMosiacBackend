use actix_web::{get, web, App, HttpResponse, HttpServer, Responder};
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
async fn posts() -> impl Responder {
    let posts = structs::Posts{
        posts: vec![
            structs::PostSmall {
                id: 0,
                name: String::from("Global Warming is Bad")
            },
            structs::PostSmall {
                id: 1,
                name: String::from("What tisadw wd aw")
            },
            structs::PostSmall {
                id: 2,
                name: String::from("af ja fj jej j ejj ej ")
            }
        ]
    };

    HttpResponse::Ok().json(posts)
}

async fn welcome() -> impl Responder {
    let apis: Vec<String> = vec!["/posts".into(), "/post".into(), "/about".into(), "/comments".into(), "/user".into()];
    let mut html_apis: String = "".into();
    for api in apis {
        html_apis += &("<a href='".to_owned() + &api + "'>" + &api + "</a><br>");
    }
    HttpResponse::Ok().body("<h1>Hello There.</h1>Here are the links to various API backends.<br>".to_owned() + &html_apis)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(post)
            .service(posts)
            .route("/", web::get().to(welcome))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}