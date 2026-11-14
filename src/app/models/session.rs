use toasty::{Deferred, Model};
use crate::app::models::user::User;

#[derive(Debug, Model)]
pub struct Session {
    #[key]
    id: String,

    #[index]
    user_id: uuid::Uuid,

    #[belongs_to(key = user_id, references = id)]
    user: Deferred<User>,

    #[auto]
    created_at: jiff::Timestamp,

    #[auto]
    updated_at: jiff::Timestamp,

    expires_at: jiff::Timestamp,

    reevoked_at: Option<jiff::Timestamp>,
}