use std::future::Future;

use axum::{
    body,
    extract::State,
    http::{Response, StatusCode},
    response::IntoResponse,
    routing::get,
    Router,
};
use bytes::Bytes;
use tokio::net::TcpListener;

use crate::server::ZserveError;

pub fn setup_http_listener(
    file: Bytes,
    port: u16,
) -> Result<
    (
        Router,
        impl Future<Output = Result<TcpListener, std::io::Error>>,
    ),
    ZserveError,
> {
    let app: Router = Router::new().route("/", get(serve_file)).with_state(file);

    let listener = tokio::net::TcpListener::bind(("0.0.0.0", port));
    Ok((app, listener))
}

async fn serve_file(State(state): State<Bytes>) -> impl IntoResponse {
    Response::builder()
        .status(StatusCode::OK)
        .header("content-type", "application/octet-stream")
        .body(body::Body::from(state))
        .unwrap()
}
