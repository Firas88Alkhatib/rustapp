// @generated automatically by Diesel CLI.

diesel::table! {
    users (id) {
        id -> Int4,
        username -> Varchar,
        password -> Varchar,
        roles -> Array<Text>,
        first_name -> Varchar,
        last_name -> Varchar,
    }
}
