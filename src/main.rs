mod model;
mod model_a;

use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};
use model::AppState;

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

#[post("/user/{username}")]
async fn create_user(path: web::Path<(String,)>, state: web::Data<AppState>) -> impl Responder {
    let (username,) = path.into_inner();

    let mut usrs = state.users.lock().unwrap();
    match usrs.create(username) {
        Ok(id) => HttpResponse::Ok().body( format!("created user with id={id}") ),
        Err(msg) => HttpResponse::BadRequest().body( msg )
    }
}

#[get("/user")]
async fn users_list( state: web::Data<AppState> ) -> impl Responder {
    let mut out = String::new();
    let usrs = state.users.lock().unwrap();
    for ( usr, id ) in usrs.users.iter() {
        out.push_str(usr);
        out.push_str(" ");
        let s = (*id).to_string();
        out.push_str(&s);
        out.push_str("\n\r");
    }
    HttpResponse::Ok().body(out)
}

#[test]
fn test_user_list() {
    use serde::{Deserialize, Serialize};
    use serde_json::json;

    let values = vec![ "+7 950 190 66 77", "8(343)3451234" ];

    let john = json!({
        "name": "John Doe",
        "age": 43,
        "phones": values
    });

    println!("first phone number: {}", john["phones"][0]);
    println!("{}", john.to_string());

    println!("--------------");

    let combine = json!({
        "root": "Root a/b",
        "inner": john
    });
    println!("{}",combine.to_string());
}

#[post("/echo")]
async fn echo(req_body: String) -> impl Responder {
    HttpResponse::Ok().body(req_body)
}

async fn manual_hello() -> impl Responder {
    HttpResponse::Ok().body("Hey there!")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {    
    let state = web::Data::new( AppState::new() );

    HttpServer::new(move || {
        App::new()
        .app_data(state.clone())
            .service(hello)
            .service(echo)
            .service(create_user)
            .service(users_list)
            .route("/hey", web::get().to(manual_hello))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}