use std::{
    env,
    net::SocketAddr,
    str::FromStr,
    sync::{Arc, Mutex},
};

use axum_askama_template::{routes, state::AppState};
use sqlx::{migrate::Migrator, sqlite::{SqliteConnectOptions, SqlitePoolOptions}};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt, EnvFilter};

static MIGRATOR: Migrator = sqlx::migrate!();

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenvy::dotenv().ok();
    // Initialize the tracing subscriber.
    // This collects and formats logs, and can be configured with the `RUST_LOG` environment variable.
    tracing_subscriber::registry()
        .with(EnvFilter::try_from_default_env().unwrap_or_else(|_| {
            "axum_askama_template=debug,tower_http=debug".into()
        }))
        .with(tracing_subscriber::fmt::layer())
        .init();

    let addr_str = env::var("LISTEN_ADDR").unwrap_or_else(|_| "127.0.0.1:8000".to_string());
    let addr = SocketAddr::from_str(&addr_str)?;
    tracing::info!("Server is listening on {}", addr);

    let listener = tokio::net::TcpListener::bind(addr).await?;

    let db_url = env::var("DATABASE_URL").unwrap_or_else(|_| "sqlite:database/users.db".to_string());
    let db_connect_options = SqliteConnectOptions::from_str(&db_url)?
        .create_if_missing(true);
    
    let db_pool = SqlitePoolOptions::new()
        .max_connections(5)
        .connect_with(db_connect_options)
        .await?;

    MIGRATOR.run(&db_pool).await?;
    let app_state = AppState {
        db_pool,
        view_count: Arc::new(Mutex::new(0)),
    };

    let app = routes::router(app_state);

    axum::serve(listener, app).await?;
    Ok(())
}
