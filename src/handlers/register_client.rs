use crate::handlers::{new_error_response, new_success_response, ErrorReason};
use crate::store::Client;
use crate::State;
use serde::Deserialize;
use std::sync::Arc;
use warp::http;

#[derive(Deserialize)]
pub struct RegisterBody {
    pub client_id: String,
    #[serde(rename = "type")]
    pub push_type: String,
    pub token: String,
}

pub async fn handler(
    state: Arc<State<impl crate::store::ClientStore>>,
    body: RegisterBody,
) -> Result<impl warp::Reply, warp::Rejection> {
    let internal_server_error = new_error_response(vec![]);

    if !vec!["fcm", "apns"].contains(&&*body.push_type.to_lowercase()) {
        return Ok(warp::reply::with_status(
            warp::reply::json(&new_error_response(vec![ErrorReason {
                field: "type".to_string(),
                description: "Invalid Push Service, must be one of: fcm, apns".to_string(),
            }])),
            http::StatusCode::BAD_REQUEST,
        ));
    }

    if body.token.is_empty() {
        return Ok(warp::reply::with_status(
            warp::reply::json(&new_error_response(vec![ErrorReason {
                field: "token".to_string(),
                description: "The `token` field must not be empty".to_string(),
            }])),
            http::StatusCode::BAD_REQUEST,
        ));
    }

    let mut store = state.store.lock().unwrap();
    let exists = store.get_client(&body.client_id);
    if let Err(_) = exists {
        return Ok(warp::reply::with_status(
            warp::reply::json(&internal_server_error),
            http::StatusCode::INTERNAL_SERVER_ERROR,
        ));
    }

    if exists.unwrap().is_some() {
        return Ok(warp::reply::with_status(
            warp::reply::json(&new_error_response(vec![ErrorReason {
                field: "client_id".to_string(),
                description: "A client is already registered with this id".to_string(),
            }])),
            http::StatusCode::BAD_REQUEST,
        ));
    }

    if let Err(_) = store.create_client(
        body.client_id.clone(),
        Client {
            push_type: body.push_type.clone(),
            token: body.token,
        },
    ) {
        return Ok(warp::reply::with_status(
            warp::reply::json(&internal_server_error),
            http::StatusCode::INTERNAL_SERVER_ERROR,
        ));
    }

    // TODO Register webhook with relay.

    if let Some(metrics) = &state.metrics {
        metrics.registered_webhooks.add(
            1,
            &[
                opentelemetry::KeyValue::new("client.id", body.client_id),
                opentelemetry::KeyValue::new("client.type", body.push_type),
            ],
        );
    }

    Ok(warp::reply::with_status(
        warp::reply::json(&new_success_response()),
        http::StatusCode::OK,
    ))
}
