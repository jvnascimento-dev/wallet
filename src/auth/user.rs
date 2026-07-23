use password_auth::VerifyError;

use crate::{error::AppErr, repository::Repository};

pub struct User {
    id: i64,
    username: String,
}

impl User {
    pub fn new(id: i64, username: String) -> Self {
        Self { id, username }
    }

    pub const fn username(&self) -> &String {
        &self.username
    }

    pub const fn id(&self) -> i64 {
        self.id
    }
}

pub struct UnauthUser {
    username: String,
    password: String,
}

impl UnauthUser {
    pub fn new(username: String, password: String) -> Self {
        Self { username, password }
    }

    pub async fn authenticate(&self, repository: &Repository) -> Result<User, AppErr> {
        let user_record = match repository.get_user_by_username(&self.username).await? {
            Some(user_record) => user_record,
            None => return Err(AppErr::InvalidCredentials),
        };
        match password_auth::verify_password(&self.password, &user_record.password_hash) {
            Ok(()) => Ok(User::new(user_record.id, user_record.username)),
            Err(VerifyError::PasswordInvalid) => Err(AppErr::InvalidCredentials),
            Err(VerifyError::Parse(err)) => panic!("Hashing algorithm failed: {err}"),
        }
    }

    pub async fn register(self, repository: &Repository) -> Result<User, AppErr> {
        let password_hash = password_auth::generate_hash(self.password);
        let user_record = match repository.add_user(&self.username, &password_hash).await {
            Ok(user_record) => user_record,
            Err(sqlx::Error::Database(db_err)) if db_err.is_unique_violation() => {
                return Err(AppErr::UsernameTaken);
            }
            Err(err) => return Err(AppErr::Database(err)),
        };
        Ok(User::new(user_record.id, user_record.username))
    }
}
