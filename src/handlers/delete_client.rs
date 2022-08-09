use std::sync::Arc;
use warp::http;
use crate::State;

pub async fn handler(id: String, _state: Arc<State>) -> Result<impl warp::Reply, warp::Rejection> {
    let response = warp::reply::with_status(
        format!("Delete Client {}", id),
        http::StatusCode::OK
    );

    Ok(response)
}