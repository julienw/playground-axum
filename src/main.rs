mod db;
mod handlers;

use axum::{Router, routing::get};
use db::connect;
use migration::{Migrator, MigratorTrait};
use sea_orm::DatabaseConnection;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let db: DatabaseConnection = connect().await;
    Migrator::up(&db, None).await?;
    let app = Router::new().route(
        "/",
        get({
            let db = db.clone();
            move || handlers::list_users(db)
        }),
    );

    // run our app with hyper, listening globally on port 3000
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    println!("Listening on http://{}", listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap();

    Ok(())
}
