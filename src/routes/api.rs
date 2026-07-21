use std::collections::HashMap;

use axum::{Json, Router, extract::State, routing::get};
use serde::Deserialize;

use crate::models::{Asset, DtoAsset, UpdateDtoAsset};
use crate::{app::AppState, auth::admin::Admin, error::AppErr};

pub fn router() -> Router<AppState> {
    Router::new().route(
        "/assets",
        get(list_assets).post(create_asset).patch(update_asset),
    )
}

#[tracing::instrument(skip_all)]
async fn list_assets(state: State<AppState>) -> Json<HashMap<i64, Asset>> {
    let assets = state.assets.lock().await;
    Json(assets.clone())
}

#[tracing::instrument(skip_all)]
async fn create_asset(
    _: Admin,
    state: State<AppState>,
    Json(request): Json<DtoAsset>,
) -> Json<Asset> {
    let mut assets = state.assets.lock().await;
    let id = assets
        .values()
        .map(|asset| asset.id)
        .max()
        .unwrap_or_default()
        + 1;
    let new_asset = Asset {
        id,
        name: request.name,
        unit_value: request.unit_value,
    };
    assets.insert(id, new_asset.clone());
    Json(new_asset)
}

#[tracing::instrument(skip_all)]
async fn update_asset(
    _: Admin,
    state: State<AppState>,
    Json(request): Json<UpdateDtoAsset>,
) -> Result<Json<Asset>, AppErr> {
    let mut assets = state.assets.lock().await;
    let Some(existing_asset) = assets.get_mut(&request.id) else {
        return Err(AppErr::AssetDoesNotExist);
    };

    if let Some(new_name) = request.name {
        existing_asset.name = new_name;
    }
    if let Some(new_unit_value) = request.unit_value {
        existing_asset.unit_value = new_unit_value;
    }

    Ok(Json(existing_asset.clone()))
}
