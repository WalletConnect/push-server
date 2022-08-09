use std::sync::Arc;
use warp::http;
use crate::State;
use serde::Deserialize;

#[derive(Deserialize)]
pub struct MessagePayload {
    message: String
}

#[derive(Deserialize)]
pub struct PushMessageBody {
    id: String,
    payload: MessagePayload
}

pub async fn handler(client_id: String, _state: Arc<State>, _body: PushMessageBody) -> Result<impl warp::Reply, warp::Rejection> {
    let response = warp::reply::with_status(
        format!("Push message to {}", client_id),
        http::StatusCode::OK
    );

    Ok(response)
}