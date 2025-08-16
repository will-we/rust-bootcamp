mod chat;
mod messages;
mod auth;

use axum::response::IntoResponse;

pub(crate) use chat::*;
pub(crate) use messages::*;
pub(crate) use auth::*;

pub(crate) async fn index_handler() -> impl IntoResponse {
    "index"
}