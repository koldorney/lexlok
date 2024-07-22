use actix_cors::Cors;
use actix_web::{App, HttpResponse, HttpServer, web};
use diesel::prelude::*;
use deadpool_diesel::postgres::Pool;
use dotenvy::dotenv;
use std::env;
use actix_web::web::Json;
use crate::models::{NewUser, User};
use crate::pool::{init_pool, PoolError};
use crate::tables::users;

mod models;
mod tables;
mod pool;

async fn add_user(pool : web::Data<Pool>, user : web::Json<NewUser>) -> HttpResponse {
    let pgconn = pool.get().await.map_err(PoolError::ConnError).unwrap();
    let newuser = user.into_inner();

    let result : NewUser = pgconn
        .interact(|conn| {
            diesel::insert_into(users::table)
                .values(&newuser)
                .returning(NewUser::as_returning())
                .get_result(conn)
        }).unwrap();
    HttpResponse::body(Json(result));
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let pool = init_pool().expect("init pool failed");
    HttpServer::new(|| {
        App::new()
            .app_data(web::Data::new(pool.clone()))
            .wrap(
                Cors::default()
                    .allow_any_origin()
                    .allow_any_header()
                    .allow_any_method()
            )
    })
        .bind(("127.0.0.1", 8080))
        .run()
        .await;

    Ok(())
}