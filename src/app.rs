use crate::models::{Asset, DtoAsset};
use ::axum::Router;
use std::{collections::HashMap, sync::Arc};
use tokio::{net::TcpListener, sync::Mutex};
use tracing::info;
use tracing_subscriber::{
    Layer, fmt::format::FmtSpan, layer::SubscriberExt, util::SubscriberInitExt,
};

use crate::routes;

pub struct App;

#[derive(Clone)]
pub struct AppState {
    pub assets: Arc<Mutex<HashMap<i64, Asset>>>,
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
            .nest("/api", routes::api::router())
            .with_state(AppState::new());
        info!("Start Service");
        axum::serve(listener, router).await?;
        Ok(())
    }
}
