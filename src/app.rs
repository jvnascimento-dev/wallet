use std::sync::Arc;

use axum::{Json, Router, extract::State, routing::get};
use tokio::{net::TcpListener, sync::Mutex};
use tracing::info;
use tracing_subscriber::{
    Layer, fmt::format::FmtSpan, layer::SubscriberExt, util::SubscriberInitExt,
};

use crate::models::{Asset, DtoAsset};
pub struct App;

#[derive(Clone)]
pub struct AppState {
    pub assets: Arc<Mutex<Vec<Asset>>>,
}
impl AppState {
    pub fn new() -> Self {
        Self {
            assets: Default::default(),
        }
    }
}

impl App {
    pub async fn start() -> color_eyre::Result<()> {
        let layer = tracing_subscriber::fmt::layer()
            .with_span_events(FmtSpan::NEW)
            .boxed();
        tracing_subscriber::registry().with(layer).init();

        let listener: TcpListener = TcpListener::bind("0.0.0.0:3000").await?;
        let router = Router::new()
            .route("/", get(list_assets).post(create_asset))
            .with_state(AppState::new());
        info!("Start Service");
        axum::serve(listener, router).await?;
        Ok(())
    }
}
#[tracing::instrument(skip_all)]
async fn list_assets(state: State<AppState>) -> Json<Vec<Asset>> {
    let assets = state.assets.lock().await;
    Json(assets.clone())
}

#[tracing::instrument(skip_all)]
async fn create_asset(state: State<AppState>, Json(request): Json<DtoAsset>) -> Json<Asset> {
    let mut assets = state.assets.lock().await;
    let id = assets
        .iter()
        .map(|asset| asset.id)
        .max()
        .unwrap_or_default()
        + 1;
    let new_asset = Asset {
        id,
        name: request.name,
        unit_value: request.unit_value,
    };
    assets.push(new_asset.clone());
    Json(new_asset)
}
