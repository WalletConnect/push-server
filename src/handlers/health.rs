use crate::State;
use std::sync::Arc;
use warp::http;

pub async fn handler(
    state: Arc<State<impl crate::store::ClientStore>>,
) -> Result<impl warp::Reply, warp::Rejection> {
    let response = warp::reply::with_status(
        format!("OK, echo-server v{}", state.build_info.crate_info.version),
        http::StatusCode::OK,
    );

    Ok(response)
}
