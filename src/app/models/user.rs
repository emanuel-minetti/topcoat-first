use toasty::Model;
use crate::app::models::language::Language;

#[derive(Debug, Model)]
pub struct User {
    #[key]
    #[auto]
    id: uuid::Uuid,

    #[unique]
    #[column(type = varchar(24))]
    username: String,

    name: String,
    language: Language,
    password_hash: String,
}
