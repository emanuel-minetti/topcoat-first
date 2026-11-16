use crate::app::models::user::User;
use toasty::{Deferred, Model};

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
