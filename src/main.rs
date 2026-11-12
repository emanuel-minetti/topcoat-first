mod app;
mod components;

#[tokio::main]
async fn main() {
    let db = app::db::connect().await.unwrap();
    topcoat::start(app::router().with(db).build()).await.unwrap();
}
