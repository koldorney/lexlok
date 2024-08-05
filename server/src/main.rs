use actix_cors::Cors;
use actix_web::{App, get, HttpResponse, HttpServer, post, web};
use diesel::prelude::*;
use deadpool_diesel::postgres::Pool;
use crate::models::{NewUser};
use crate::pool::init_pool;
use crate::schema::users;

mod models;
mod schema;
mod pool;

#[post("/user")]
async fn add_user(pool : web::Data<Pool>, user : web::Json<NewUser>) -> HttpResponse {
    let pgconn = pool.get().await.unwrap();
    let newuser = user.into_inner();

    let result : NewUser = pgconn
        .interact(|conn| {
            diesel::insert_into(users::table)
                .values(newuser)
                .returning(NewUser::as_returning())
                .get_result(conn)
        }).await
        .unwrap()
        .unwrap();
    HttpResponse::Ok().json(result)
}

#[get("/")]
async fn landing() -> HttpResponse {
    println!("hello");
    HttpResponse::Ok().body("welcome page")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let pool = init_pool().expect("init pool failed");
    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(pool.clone()))
            .wrap(
                Cors::default()
                    .allow_any_origin()
                    .allow_any_header()
                    .allow_any_method()
            )
            .service(add_user)
            .service(landing)
    })
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}