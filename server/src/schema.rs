// @generated automatically by Diesel CLI.

diesel::table! {
    users (id) {
        id -> Int4,
        firstname -> Varchar,
        lastname -> Varchar,
        username -> Varchar,
        email -> Varchar,
        password -> Varchar,
        groups -> Nullable<Array<Nullable<Int4>>>,
    }
}
