use surrealdb::opt::auth::Root;

use super::schema::init_schema;
use crate::db::DB;

pub async fn init_database(database_url: &str) {
    DB.connect(database_url).await.unwrap_or_else(|e| {
        eprintln!("Failed to connect to database: {}", e);
        eprintln!("\nThe DATABASE_URL format should be one of:");
        eprintln!("  mem://                           (in-memory)");
        eprintln!("  surrealkv://path/to/data         (local file)");
        eprintln!("  ws://localhost:8000              (WebSocket)");
        eprintln!("  wss://cloud.surrealdb.com        (WebSocket Secure)");
        eprintln!("\nPlease check your .env file.");
        std::process::exit(1);
    });

    if database_url.starts_with("ws://") || database_url.starts_with("wss://") {
        let username =
            dotenvy::var("SURREAL_USER").unwrap_or_else(|_| "root".to_string());
        let password =
            dotenvy::var("SURREAL_PASS").unwrap_or_else(|_| "root".to_string());

        DB.signin(Root {
            username: &username,
            password: &password,
        })
        .await
        .unwrap_or_else(|e| {
            eprintln!("Failed to authenticate with SurrealDB: {}", e);
            eprintln!("\nTroubleshooting:");
            eprintln!("  1. Check SURREAL_USER and SURREAL_PASS in .env");
            eprintln!("  2. Verify the SurrealDB server is running");
            std::process::exit(1);
        });
    }

    let db_name = dotenvy::var("SURREAL_DB").unwrap_or_else(|_| "app".to_string());
    let ns_name = dotenvy::var("SURREAL_NS").unwrap_or_else(|_| "app".to_string());

    DB.use_ns(&ns_name).use_db(&db_name).await.unwrap_or_else(|e| {
        eprintln!("Failed to select namespace/database: {}", e);
        std::process::exit(1);
    });

    init_schema().await;
}
