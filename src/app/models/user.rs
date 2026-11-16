use crate::app::models::language::Language;
use crate::db::db;
use argon2::{Argon2, PasswordHash};
use password_hash::PasswordVerifier;
use toasty::Model;
use topcoat::Result;
use topcoat::context::Cx;
use topcoat::router::error::internal_server_error;

#[derive(Debug, Clone, Model)]
pub struct User {
    #[key]
    #[auto]
    pub(crate) id: uuid::Uuid,

    #[unique]
    #[column(type = varchar(24))]
    username: String,

    pub(crate) name: String,
    language: Language,
    password_hash: String,
}

impl User {
    /// Checks the credentials and returns the user, or `None` if they do not match.
    pub async fn authenticate(cx: &Cx, username: &str, password: String) -> Result<Option<User>> {
        let user = User::filter_by_username(username)
            .first()
            .exec(&mut db(cx))
            .await?;

        let Some(user) = user else { return Ok(None) };

        let password_hash = user.password_hash.clone();
        let matches =
            tokio::task::spawn_blocking(move || verify_password(&password, &password_hash))
                .await
                .map_err(internal_server_error)?;

        Ok(matches.then_some(user))
    }
}

fn verify_password(password: &str, password_hash: &str) -> bool {
    PasswordHash::new(password_hash).is_ok_and(|hash| {
        Argon2::default()
            .verify_password(password.as_bytes(), &hash)
            .is_ok()
    })
}
