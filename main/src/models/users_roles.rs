use crate::models::role::Role;
use crate::models::user::User;
use crate::schema::users_roles;
use serde::{Deserialize, Serialize};

// #[diesel(belongs_to(Product))]
// #[diesel(belongs_to(Category))]
#[derive(Identifiable, Deserialize, Serialize, Debug, Queryable, AsChangeset, Associations)]
#[diesel(belongs_to(User))]
#[diesel(belongs_to(Role))]
#[diesel(table_name = users_roles)]
pub struct UserRole {
    pub id: i32,
    pub user_id: i32,
    pub role_id: i32,
}

#[derive(Insertable)]
#[diesel(table_name = users_roles)]
pub struct NewUserRole<'a> {
    pub user_id: &'a i32,
    pub role_id: &'a i32,
}
