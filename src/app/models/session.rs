use jiff::Timestamp;
use crate::app::models::user::User;
use toasty::{Deferred, Model};
use topcoat::context::Cx;
use topcoat::Result;
use topcoat::session::TokenHash;
use crate::db::db;

#[derive(Debug, Model)]
pub struct Session {
    #[key]
    #[auto]
    id: uuid::Uuid,

    #[unique]
    token_hash: Vec<u8>,

    #[index]
    pub user_id: uuid::Uuid,

    #[belongs_to(key = user_id, references = id)]
    user: Deferred<User>,

    #[auto]
    created_at: jiff::Timestamp,

    #[auto]
    updated_at: jiff::Timestamp,

    pub expires_at: jiff::Timestamp,

    #[default(None)]
    pub revoked_at: Option<jiff::Timestamp>,
}

impl Session {
    pub async fn update_session_expiry(cx: &Cx, token_hash: TokenHash, expires_at: Timestamp) -> Result<()> {
        Session::update_by_token_hash(token_hash.to_vec())
            .expires_at(expires_at)
            .exec(&mut db(cx))
            .await?;

        Ok(())
    }
}
