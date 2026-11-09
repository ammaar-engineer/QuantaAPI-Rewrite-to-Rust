use std::env;

use axum::Router;
use sea_orm::{ConnectOptions, Database, DatabaseConnection};
use std::time::Duration;

#[tokio::main]
async fn main() {
    dotenvy::dotenv().ok();

    // Koneksi ke PostgreSQL (docker-compose: instantapi-db)
    let db_url = env::var("DB_STRING").expect("DB_STRING harus di-set di .env");

    let mut opt = ConnectOptions::new(db_url);
    opt.max_connections(5)
        .min_connections(1)
        .connect_timeout(Duration::from_secs(10))
        .acquire_timeout(Duration::from_secs(10))
        .sqlx_logging(true);

    let db: DatabaseConnection = Database::connect(opt)
        .await
        .expect("Gagal konek ke PostgreSQL");

    println!("Berhasil terhubung ke PostgreSQL");

    let app = Router::new().with_state(db);

    let port = env::var("PORT").unwrap_or_else(|_| "3000".to_string());
    let listener = tokio::net::TcpListener::bind(format!("0.0.0.0:{port}"))
        .await
        .unwrap();
    println!("Server berjalan di http://0.0.0.0:{port}");
    axum::serve(listener, app).await.unwrap();
}
