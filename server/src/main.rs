use actix_cors::Cors;
use actix_web::{App, get, HttpResponse, HttpServer, post, web};
use deadpool_diesel::postgres::Pool;
use diesel::prelude::*;

use crate::models::{NewUser, User};
use crate::pool::init_pool;
use crate::schema::users;

mod models;
mod schema;
mod pool;

#[get("/user")]
async fn get_users(pool : web::Data<Pool>) -> HttpResponse {
    let pgconn = pool.get().await.unwrap();
    let result: Vec<User> = pgconn
        .interact(|conn| {
            users::table.load::<User>(conn).expect("unable to get users")
        })
        .await
        .expect("Database interaction failed");
    HttpResponse::Ok().json(result)
}
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

#[cfg(test)]
mod tests {
    use actix_web::{App, test, web};
    use actix_web::http::StatusCode;
    use deadpool_diesel::Status;
    use crate::get_users;

    #[actix_web::test]
    async fn test_user_get() {
        let pool = init_pool().expect("init pool failed");
        let app = test::init_service(
            App::new()
                .app_data(web::Data::new(pool.clone()))
                .service(get_users))
            .await;
        let req = test::TestRequest::get().uri("/user").to_request();
        let resp = test::call_service(&app, req).await;
        assert_eq!(resp.status(), StatusCode::OK);
    }
}
