use std::sync::Arc;
use warp::http;
use crate::State;

pub async fn handler(state: Arc<State>) -> Result<impl warp::Reply, warp::Rejection> {
    let response = warp::reply::with_status(
        format!("OK, echo-server v{}", state.build_info.crate_info.version),
        http::StatusCode::OK
    );

    Ok(response)
}