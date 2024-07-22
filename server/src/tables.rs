use diesel::table;
use deadpool_diesel::postgres::{Connection, Manager, Pool, BuildError};
use deadpool_diesel::Runtime;


table! {
    users (id) {
        id -> Integer,
        firstname -> Text,
        lastname -> Text,
        username -> Text,
        email -> Text,
    }
}