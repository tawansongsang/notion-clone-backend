use axum::{routing::get, Router};

use crate::{config, routes};

#[derive(Clone)]
pub struct AppState {
    pub config: config::Config,
}

impl AppState {
    pub fn build(config: config::Config) -> AppState {
        AppState { config }
    }
}

// pub fn create_app(eservice_url: &String, token: &String) -> Router {
//     let app_state = AppState::create_app_state(eservice_url, token);
pub fn create_app(config: config::Config) -> Router {
    async fn root() -> &'static str {
        "Hello, World!"
    }

    let app_state = AppState::build(config);

    Router::new()
        .merge(Router::new().route("/", get(root)))
        .merge(
            Router::new()
                .nest(
                    "/api",
                    Router::new()
                        .merge(routes::health_check::create_route())
                        .merge(routes::auth::create_route()),
                )
                .with_state(app_state),
        )
}
