use toasty_cli::{Config, ToastyCli};
use topcoat_first::app::db;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let config = Config::load()?;

    let db = db::connect().await?;

    let cli = ToastyCli::with_config(db, config);
    cli.parse_and_run().await?;

    Ok(())
}
