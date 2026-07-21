use crate::models::{Asset, DtoAsset, UpdateDtoAsset};
use crate::repository::Repository;
use crate::{app::AppState, auth::admin::Admin, error::AppErr};
use axum::{Json, Router, routing::get};

pub fn router() -> Router<AppState> {
    Router::new().route(
        "/assets",
        get(list_assets).post(create_asset).patch(update_asset),
    )
}

#[tracing::instrument(skip_all)]
async fn list_assets(repository: Repository) -> Result<Json<Vec<Asset>>, AppErr> {
    let assets = repository.list_assets().await?;
    Ok(Json(assets))
}

#[tracing::instrument(skip_all)]
async fn create_asset(
    _: Admin,
    repository: Repository,
    Json(request): Json<DtoAsset>,
) -> Result<Json<Asset>, AppErr> {
    let new_asset = repository
        .create_asset(request.name, request.unit_value)
        .await?;
    Ok(Json(new_asset))
}

#[tracing::instrument(skip_all)]
async fn update_asset(
    _: Admin,
    repository: Repository,
    Json(request): Json<UpdateDtoAsset>,
) -> Result<Json<Asset>, AppErr> {
    match repository
        .update_asset(request.id, request.name, request.unit_value)
        .await?
    {
        Some(update_asset) => Ok(Json(update_asset)),
        None => Err(AppErr::AssetDoesNotExist),
    }
}
