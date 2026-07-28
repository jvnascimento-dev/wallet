use crate::{
    app::AppState,
    auth::user::{UnauthUser, User},
    error::AppErr,
    models::{Asset, OwnedAsset},
    repository::Repository,
};
use askama::Template;
use axum::{
    Form, Router,
    response::{Html, IntoResponse, Redirect, Response},
    routing::get,
};
use axum_extra::extract::{CookieJar, cookie::Cookie};
use serde::Deserialize;
use tokio::try_join;

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/", get(index))
        .route("/login", get(login_page).post(login))
        .route("/logout", get(logout))
}

#[derive(Template)]
#[template(path = "login.html")]
struct LoginPage;

async fn login_page() -> Result<Html<String>, AppErr> {
    let html = LoginPage.render()?;
    Ok(Html(html))
}

#[derive(Deserialize)]
struct LoginForm {
    username: String,
    password: String,
}

async fn login(
    repository: Repository,
    jar: CookieJar,
    Form(request): Form<LoginForm>,
) -> Result<impl IntoResponse, AppErr> {
    let unauth_user = UnauthUser::new(request.username, request.password);
    let user = match unauth_user.authenticate(&repository).await {
        Ok(user) => user,
        Err(AppErr::UserDoesNotExist) => unauth_user.register(&repository).await?,
        Err(other_err) => return Err(other_err),
    };
    let token = user.auth_token()?;
    let cookie = Cookie::build(("token", token)).http_only(true);

    Ok((jar.add(cookie), Redirect::to("/")))
}

async fn index(maybe_user: Option<User>) -> Result<Response, AppErr> {
    match maybe_user {
        Some(user) => Ok(Html(format!("Hello {}", user.username())).into_response()),
        None => Ok(Redirect::to("/login").into_response()),
    }
}
async fn logout(jar: CookieJar) -> impl IntoResponse {
    (jar.remove("token"), Redirect::to("/login"))
}

#[derive(Template)]
#[template(path = "assets.html")]
pub struct AssetsPage {
    owned_assets: Vec<OwnedAsset>,
    available_assets: Vec<Asset>,
    user: User,
}

async fn assets(repository: Repository, user: User) -> Result<Html<String>, AppErr> {
    let (owned_assets, available_assets) = try_join!(
        repository.list_owned_assets(user.id()),
        repository.list_assets()
    )?;

    let html = AssetsPage {
        owned_assets,
        available_assets,
        user,
    }
    .render()?;
    Ok(Html(html))
}
