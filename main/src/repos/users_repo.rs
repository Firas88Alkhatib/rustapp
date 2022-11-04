use crate::db::{DbConnection, DbPool};
use crate::error_handle::{RepositoryError, RepositoryErrorType};
use crate::models::role::Role;
use crate::models::user::{NewUser, User, UserDto, UserOutDto};
use crate::models::users_roles::{NewUserRole, UserRole};
use crate::schema::roles::dsl::roles as DbRoles;
use crate::schema::users::dsl::*;
use crate::schema::users_roles::dsl::users_roles;
use crate::services::authentication_service::hash_password;
use diesel::{BelongingToDsl, ExpressionMethods, QueryDsl, RunQueryDsl, Table};

pub struct UsersRepo {
    connection_pool: DbPool,
}

impl UsersRepo {
    pub fn new(connection_pool: DbPool) -> Self {
        return Self { connection_pool };
    }
    fn get_connection(&self) -> DbConnection {
        let connection = self
            .connection_pool
            .get()
            .expect("Failed to get connection from the connection pool");
        return connection;
    }
    pub async fn create_user(&self, user_dto: UserDto) -> Result<User, RepositoryError> {
        let mut connection = self.get_connection();

        let hashed_password = hash_password(user_dto.password).expect("unable to hash password");

        let new_user: NewUser = NewUser {
            username: &user_dto.username,
            password: &hashed_password,
            first_name: &user_dto.first_name,
            last_name: &user_dto.last_name,
        };

        let inserted_user = diesel::insert_into(users).values(&new_user).get_result::<User>(&mut connection)?;
        let roles: Vec<NewUserRole> = user_dto
            .roles
            .iter()
            .map(|role| NewUserRole {
                role_id: &role.id,
                user_id: &inserted_user.id,
            })
            .collect();
        let affected_rows = diesel::insert_into(users_roles).values(&roles).execute(&mut connection)?;

        return Ok(inserted_user);
    }
    pub async fn get_all_users(&self) -> Result<Vec<User>, RepositoryError> {
        let mut connection = self.get_connection();
        let results = users.load::<User>(&mut connection)?;
        return Ok(results);
    }
    pub async fn get_user(&self, user_id: i32) -> Result<User, RepositoryError> {
        let mut connection = self.get_connection();
        let user = users.find(user_id).first(&mut connection)?;
        return Ok(user);
    }
    pub async fn get_user_by_username(&self, user_name: String) -> Result<UserOutDto, RepositoryError> {
        let mut connection = self.get_connection();
        let user: User = users.filter(username.eq(user_name)).first(&mut connection)?;
        let user_roles = UserRole::belonging_to(&user)
            .inner_join(DbRoles)
            .select(DbRoles::all_columns())
            .load::<Role>(&mut connection)?;

        let user = UserOutDto {
            id: user.id,
            first_name: user.first_name,
            last_name: user.last_name,
            username: user.username,
            password: user.password,
            roles: user_roles,
        };
        return Ok(user);
    }

    pub async fn update_user(&self, user_id: i32, user: User) -> Result<User, RepositoryError> {
        let mut connection = self.get_connection();
        let updated_user = diesel::update(users.find(user_id)).set(&user).get_result::<User>(&mut connection)?;

        return Ok(updated_user);
    }

    pub async fn delete_user(&self, user_id: i32) -> Result<usize, RepositoryError> {
        let mut connection = self.get_connection();
        let result = diesel::delete(users.find(user_id)).execute(&mut connection)?;
        return match result {
            0 => Err(RepositoryError {
                error_type: RepositoryErrorType::NotFound,
            }),
            _ => Ok(result),
        };
    }
}
