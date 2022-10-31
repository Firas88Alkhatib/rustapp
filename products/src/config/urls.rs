use std::env;

pub fn get_database_url() -> String {
    let database_url = env::var("DATABASE_URL").unwrap_or(String::from("postgres://postgres:postgres@0.0.0.0/rustappprod"));
    return database_url;
}
