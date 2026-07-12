use axum::{Router, routing::get};
use tokio::net::TcpListener;
use tracing::info;
use tracing_subscriber::{
    Layer, fmt::format::FmtSpan, layer::SubscriberExt, util::SubscriberInitExt,
};

#[tokio::main]
async fn main() -> color_eyre::Result<()> {
    let layer = tracing_subscriber::fmt::layer()
        .with_span_events(FmtSpan::NEW)
        .boxed();
    tracing_subscriber::registry().with(layer).init();

    let listener: TcpListener = TcpListener::bind("0.0.0.0:3000").await?;
    let router = Router::new().route("/", get(test));
    info!("Start Service");
    axum::serve(listener, router).await?;
    Ok(())
}

#[tracing::instrument]
async fn test() -> &'static str {
    "Hello Word"
}
