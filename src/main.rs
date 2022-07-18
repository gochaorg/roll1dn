use std::sync::Mutex;

use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};
use std::collections::HashMap;

struct AppState {
    count: Mutex<u32>,
    user_id_seq: Mutex<u32>,
    users: Mutex<HashMap<String,u32>>
}

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

#[post("/user/{username}")]
async fn create_user(path: web::Path<(String,)>, state: web::Data<AppState>) -> impl Responder {
    let r = path.into_inner();
    let username = r.0;
    
    let mut usrs = state.users.lock().unwrap();
    if usrs.contains_key(&username) {
        println!("user {username} exists");
        HttpResponse::BadRequest().body( format!("user {username} already exists" ))
    }else{
        println!("users {username} not exists");
        let mut id_seq = state.user_id_seq.lock().unwrap();
        (*id_seq) += 1;
        let id = *id_seq;
        usrs.insert(username.clone(), id);
        HttpResponse::Ok().body( format!("create user {username} with id {id}" ))
    }
}

#[get("/user")]
async fn users_list( state: web::Data<AppState> ) -> impl Responder {
    let mut out = String::new();
    let usrs = state.users.lock().unwrap();
    for ( usr, id ) in usrs.iter() {
        out.push_str(usr);
        out.push_str(" ");
        let s = (*id).to_string();
        out.push_str(&s);
        out.push_str("\n\r");
    }
    HttpResponse::Ok().body(out)
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
        count: Mutex::new(0u32),
        user_id_seq: Mutex::new(0u32),
        users: Mutex::new(HashMap::new())
    });

    HttpServer::new(move || {
        App::new()
        .app_data(state.clone())
            .service(hello)
            .service(echo)
            .service(count)
            .service(create_user)
            .service(users_list)
            .route("/hey", web::get().to(manual_hello))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}