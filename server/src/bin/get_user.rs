use std::{env, io};
use diesel::prelude::*;
use server::schema::users::dsl::users;
use server::models::User;

fn main () {
    let mut input = String::new();

    println!("Enter user's id:");
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read user's id.");

    let id = input.trim().parse::<i32>().expect("oopsie bad id");


    let dburl = env::var("DATABASE_URL").expect("No DATABASE_URL enviroment variable");
    let connection = &mut PgConnection::establish(&dburl).unwrap();

    let user = users
        .find(id)
        .select(User::as_select())
        .first(connection)
        .optional();

    match user {
        Ok(Some(User)) =>
            println!("User id {}: {} {} email: {}", User.id, User.firstname, User.lastname, User.email),
        Ok(none) => println!("user {} not fount", id),
        Err(_) => println!("did not work")
    }
}