use diesel::prelude::*;
use std::env;

pub fn get_connection() -> PgConnection {
    let database_url = env::var("DATABASE_URL").unwrap_or("postgres://postgres:postgres@0.0.0.0/rustappprod".to_string());
    PgConnection::establish(&database_url).expect(&format!("Erro connecting to {}", database_url))
}
