mod app;
pub mod db;


#[tokio::main]
async fn main() {
    let db = db::connect().await.unwrap();
    topcoat::start(app::router().with(db).build())
        .await
        .unwrap();
}
