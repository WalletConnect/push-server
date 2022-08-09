use std::sync::Arc;
use warp::http;
use crate::State;
use serde::Deserialize;

#[derive(Deserialize)]
pub struct RegisterBody {
    pub client_id: String,
    #[serde(rename = "type")]
    pub push_type: String,
    pub token: String
}

pub async fn handler(_state: Arc<State>, mut _body: RegisterBody) -> Result<impl warp::Reply, warp::Rejection> {
    let response = warp::reply::with_status(
        "Register Client",
        http::StatusCode::OK
    );

    Ok(response)
}