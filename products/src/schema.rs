// @generated automatically by Diesel CLI.

diesel::table! {
    categories (id) {
        id -> Int4,
        name -> Varchar,
    }
}

diesel::table! {
    products (id) {
        id -> Int4,
        name -> Varchar,
        description -> Varchar,
    }
}

diesel::table! {
    productscategories (id) {
        id -> Int4,
        product_id -> Int4,
        category_id -> Int4,
    }
}

diesel::joinable!(productscategories -> categories (category_id));
diesel::joinable!(productscategories -> products (product_id));

diesel::allow_tables_to_appear_in_same_query!(
    categories,
    products,
    productscategories,
);
