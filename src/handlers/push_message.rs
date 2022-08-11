use crate::State;
use serde::Deserialize;
use std::sync::Arc;
use warp::http;

#[derive(Deserialize)]
pub struct MessagePayload {
    message: String,
}

#[derive(Deserialize)]
pub struct PushMessageBody {
    id: String,
    payload: MessagePayload,
}

pub async fn handler(
    client_id: String,
    _state: Arc<State<impl crate::store::ClientStore>>,
    _body: PushMessageBody,
) -> Result<impl warp::Reply, warp::Rejection> {
    let response = warp::reply::with_status(
        format!("Push message to {}", client_id),
        http::StatusCode::OK,
    );

    Ok(response)
}
