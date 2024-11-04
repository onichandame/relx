use args::Args;
use clap::Parser;
use futures::{pin_mut, FutureExt};
use migration::{Migrator, MigratorTrait};
use sea_orm::Database;
use service::start_service;

mod args;
mod event;
mod service;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let args = Args::parse();
    let db = Database::connect(&args.database_url).await?;
    Migrator::up(&db, None).await?;
    let service = start_service(&db).fuse();
    pin_mut!(service);
    tokio::select! {
        _ = service => {}
    };
    Ok(())
}
