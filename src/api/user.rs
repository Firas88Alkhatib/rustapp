use crate::db::get_connection;
use crate::models::user::{NewUser, User};
use actix_web::{get, post, put, web::Json, web::Path};
use diesel::{PgConnection, QueryDsl, RunQueryDsl};

/*
* get asdas
*/
#[get("/users")]
pub async fn get_all_users() -> Json<Vec<User>> {
    use crate::schema::users::dsl::*;
    let mut db: PgConnection = get_connection();
    let results: Vec<User> = users.load::<User>(&mut db).expect("failed to load all users");
    return Json(results);
}
#[get("/users/{id}")]
pub async fn get_user(path: Path<i32>) -> Json<User> {
    use crate::schema::users::dsl::*;
    let user_id = path.into_inner();
    let mut db: PgConnection = get_connection();
    let result: User = users.find(user_id).first(&mut db).expect("failed to load single user");
    return Json(result);
}
#[post("/users")]
pub async fn create_user(user: Json<User>) -> Json<User> {
    use crate::schema::users::dsl::*;
    let mut db: PgConnection = get_connection();

    let new_user: NewUser = NewUser {
        id: &user.id,
        name: &user.name,
    };
    diesel::insert_into(users)
        .values(&new_user)
        .execute(&mut db)
        .expect("Failed to save user");
    return Json(user.into_inner());
}
#[put("/users/{id}")]
pub async fn update_user(path: Path<i32>, updated_user: Json<User>) -> Json<User> {
    use crate::schema::users::dsl::*;
    let mut connection: PgConnection = get_connection();
    let db_user = User {
        id: updated_user.id,
        name: updated_user.name.to_string(),
    };
    diesel::update(users.find(path.into_inner()))
        .set(&db_user)
        .execute(&mut connection)
        .expect("ُError updating product");
    return Json(db_user);
}
