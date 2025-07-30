use diesel::{QueryDsl, ExpressionMethods, SelectableHelper, OptionalExtension};
use argon2::{Argon2, PasswordHash, PasswordVerifier};
use serde::Deserialize;
use crate::models::User;
use diesel_async::{AsyncPgConnection, RunQueryDsl};
use crate::schema::users::{e_mail, username};
use crate::schema::users::dsl::users;

#[derive(Deserialize)]
pub struct UserLoginInput {
    pub login: String,
    pub password: String,
    pub device_id: String
}


impl UserLoginInput {

    fn is_username(&self) -> bool {
        !self.login.contains(['.', '@'])
    }

    pub async fn find_user(&self, conn: &mut AsyncPgConnection) -> Result<Option<User>, diesel::result::Error> {

        if self.is_username() {
            users
                .filter(username.eq(&self.login))
                .select(User::as_select())
                .first::<User>(conn)
                .await
                .optional()
        } else {
            users
                .filter(e_mail.eq(&self.login))
                .select(User::as_select())
                .first::<User>(conn)
                .await
                .optional()
        }

    }

    pub fn verify_password(&self, hashed_password: &str, argon2: Argon2) -> Result<bool, argon2::password_hash::Error> {
        let parsed_hash = PasswordHash::new(&hashed_password)?;
        Ok(argon2.verify_password(&self.password.as_bytes(), &parsed_hash).is_ok())
    }

    

}
