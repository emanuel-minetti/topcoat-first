use toasty::{Embed, Model};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Embed)]
pub enum Language {
    De,
    En
}

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
