use toasty::Model;

#[derive(Debug, Model)]
struct User {
    #[key]
    #[auto]
    id: uuid::Uuid,

    #[unique]
    #[column(type = varchar(24))]
    username: String,

    name: String,
    language: String,
    password_hash: String,
}
