use axum::{http::StatusCode, routing::post, Json, Router};
use models::{CreateUser, User};

use crate::app::AppState;

pub fn create_route<'a>() -> Router<AppState> {
    Router::new().route("/users", post(create_user))
}

async fn create_user(
    // this argument tells axum to parse the request body
    // as JSON into a `CreateUser` type
    Json(payload): Json<CreateUser>,
) -> (StatusCode, Json<User>) {
    // insert your application logic here
    let user = User {
        id: 1337,
        username: payload.username,
    };

    // this will be converted into a JSON response
    // with a status code of `201 Created`
    (StatusCode::CREATED, Json(user))
}
