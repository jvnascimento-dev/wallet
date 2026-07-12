use crate::app::App;

mod app;
mod models;

#[tokio::main]
async fn main() -> color_eyre::Result<()> {
    App::start().await
}
