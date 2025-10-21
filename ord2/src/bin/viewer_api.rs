use std::{net::SocketAddr, sync::Arc};

use axum::{extract::Path, routing::get, Json, Router};
use ord2::{mirror_bridge::default_db_path, MirrorBridge, MirrorRecord};
use tokio::signal;
use tracing::{error, info};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

#[derive(Clone)]
struct AppState {
  bridge: Arc<MirrorBridge>,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
  setup_tracing();

  let db_path = default_db_path();
  let bridge = Arc::new(MirrorBridge::new(db_path)?);
  let state = AppState { bridge };

  let app = Router::new()
    .route("/mirror", get(list_mirrors))
    .route("/mirror/:commitment", get(get_mirror))
    .route("/health", get(health_check))
    .with_state(state);

  let addr: SocketAddr = "127.0.0.1:8787".parse()?;
  info!("starting viewer api", %addr);

  axum::Server::bind(&addr)
    .serve(app.into_make_service())
    .with_graceful_shutdown(shutdown_signal())
    .await?;

  Ok(())
}

async fn get_mirror(
  state: axum::extract::State<AppState>,
  Path(commitment): Path<String>,
) -> Result<Json<MirrorRecord>, axum::http::StatusCode> {
  state
    .bridge
    .get(&commitment)
    .map_err(|err| {
      error!(?err, "failed to fetch mirror record");
      axum::http::StatusCode::INTERNAL_SERVER_ERROR
    })?
    .map(Json)
    .ok_or(axum::http::StatusCode::NOT_FOUND)
}

async fn list_mirrors(
  state: axum::extract::State<AppState>,
) -> Result<Json<Vec<MirrorRecord>>, axum::http::StatusCode> {
  let records = state.bridge.list().map_err(|err| {
    error!(?err, "failed to list mirror records");
    axum::http::StatusCode::INTERNAL_SERVER_ERROR
  })?;

  Ok(Json(records))
}

async fn health_check() -> &'static str {
  "ok"
}

fn setup_tracing() {
  let subscriber = tracing_subscriber::registry()
    .with(tracing_subscriber::EnvFilter::from_default_env())
    .with(tracing_subscriber::fmt::layer());
  subscriber.init();
}

async fn shutdown_signal() {
  let ctrl_c = async {
    signal::ctrl_c()
      .await
      .expect("failed to install Ctrl+C handler");
  };

  #[cfg(unix)]
  let terminate = async {
    signal::unix::signal(signal::unix::SignalKind::terminate())
      .expect("failed to install signal handler")
      .recv()
      .await;
  };

  #[cfg(not(unix))]
  let terminate = std::future::pending::<()>();

  tokio::select! {
      _ = ctrl_c => {},
      _ = terminate => {},
  }
}
