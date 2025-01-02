use axum::{
    body,
    extract::State,
    http::{Response, StatusCode},
    response::IntoResponse,
    routing::get,
    Router,
};
use bytes::Bytes;

use crate::server::ZserveError;

pub fn setup_app(file: Bytes) -> Result<Router, ZserveError> {
    Ok(Router::new().route("/", get(serve_file)).with_state(file))
}

async fn serve_file(State(state): State<Bytes>) -> impl IntoResponse {
    Response::builder()
        .status(StatusCode::OK)
        .header("content-type", "application/octet-stream")
        .body(body::Body::from(state))
        .unwrap()
}
