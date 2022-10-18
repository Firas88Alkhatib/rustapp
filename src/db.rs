use diesel::prelude::*;

pub fn get_connection() -> PgConnection {
    let database_url: &str = "postgres://postgres:postgres@0.0.0.0/rustapp";
    PgConnection::establish(&database_url).expect(&format!("Erro connecting to {}", database_url))
}
