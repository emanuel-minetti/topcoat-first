mod app;

#[tokio::main]
async fn main() {
    let db = app::db::connect().await.unwrap();
    topcoat::start(app::router().app_context(db).build()).await.unwrap();
}
