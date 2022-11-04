// @generated automatically by Diesel CLI.

diesel::table! {
    roles (id) {
        id -> Int4,
        name -> Varchar,
    }
}

diesel::table! {
    users (id) {
        id -> Int4,
        username -> Varchar,
        password -> Varchar,
        first_name -> Varchar,
        last_name -> Varchar,
    }
}

diesel::table! {
    users_roles (id) {
        id -> Int4,
        user_id -> Int4,
        role_id -> Int4,
    }
}

diesel::joinable!(users_roles -> roles (role_id));
diesel::joinable!(users_roles -> users (user_id));

diesel::allow_tables_to_appear_in_same_query!(
    roles,
    users,
    users_roles,
);
