use std::sync::Mutex;

use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};

struct AppState {
    count: Mutex<u32>,
}

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

#[post("/echo")]
async fn echo(req_body: String) -> impl Responder {
    HttpResponse::Ok().body(req_body)
}

async fn manual_hello() -> impl Responder {
    HttpResponse::Ok().body("Hey there!")
}

#[get("/count")]
async fn count(data: web::Data<AppState>) -> impl Responder {
    let mut str = String::new();
    str.push_str("count ");
    
    let mut cnt = data.count.lock().unwrap();
    *cnt += 1;

    str.push_str( &(*cnt).to_string() );

    HttpResponse::Ok().body(str)
    // HttpResponse::Ok().body("Hey there!")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let state = web::Data::new( AppState {
        count: Mutex::new(0u32)
    });

    HttpServer::new(move || {
        App::new()
        .app_data(state.clone())
            .service(hello)
            .service(echo)
            .service(count)
            .route("/hey", web::get().to(manual_hello))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}