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

#[cfg(test)]
mod test {
    use sqlx::PgPool;

    use super::*;

    #[sqlx::test]
    async fn test_create_asset(db: PgPool) {
        let request = DtoAsset {
            name: "Bitcoin".to_string(),
            unit_value: 10.0,
        };
        let Json(new_asset) = create_asset(Admin, db.into(), Json(request))
            .await
            .expect("sucess");

        assert_eq!(new_asset.id, 1);
        assert_eq!(new_asset.name, "Bitcoin");
        assert_eq!(new_asset.unit_value, 10.0);
        insta::assert_json_snapshot!(new_asset);
    }

    #[sqlx::test(fixtures("bitcoin_asset"))]
    async fn test_list_assets(db: PgPool) {
        let Json(assets) = list_assets(db.into()).await.expect("sucess");
        assert_eq!(assets.len(), 1);
        assert_eq!(assets[0].name, "Bitcoin");
        insta::assert_json_snapshot!(assets);
    }

    #[sqlx::test(fixtures("bitcoin_asset"))]
    async fn test_update_assets(db: PgPool) {
        let request = UpdateDtoAsset {
            id: 1,
            name: Some("DogCoin".to_string()),
            unit_value: Some(50.0),
        };
        let Json(update_asset) = update_asset(Admin, db.into(), Json(request))
            .await
            .expect("sucess");

        assert_eq!(update_asset.id, 1);
        assert_eq!(update_asset.name, "DogCoin");
        assert_eq!(update_asset.unit_value, 50.0);
        insta::assert_json_snapshot!(update_asset);
    }
}
