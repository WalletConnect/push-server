use std::sync::Arc;
use redis::Commands;
use warp::http;
use crate::State;
use serde::Deserialize;
use crate::handlers::{ErrorReason, new_error_response, new_success_response};

#[derive(Deserialize)]
pub struct RegisterBody {
    pub client_id: String,
    #[serde(rename = "type")]
    pub push_type: String,
    pub token: String
}

pub async fn handler(state: Arc<State>, body: RegisterBody) -> Result<impl warp::Reply, warp::Rejection> {
    let internal_server_error = new_error_response(vec![]);

    if !vec!["fcm", "apns"].contains(&&*body.push_type.to_lowercase()) {
        return Ok(warp::reply::with_status(
            warp::reply::json(
                &new_error_response(vec![
                    ErrorReason {
                        field: "type".to_string(),
                        description: "Invalid Push Service, must be one of: fcm, apns".to_string()
                    }
                ])
            ),
            http::StatusCode::BAD_REQUEST)
        );
    }

    if body.token.is_empty() {
        return Ok(warp::reply::with_status(
            warp::reply::json(
                &new_error_response(vec![
                    ErrorReason {
                        field: "token".to_string(),
                        description: "The `token` field must not be empty".to_string()
                    }
                ])
            ),
            http::StatusCode::BAD_REQUEST)
        );
    }

    let redis = state.get_redis_connection();
    if let Err(_) = redis {
        return Ok(warp::reply::with_status(
            warp::reply::json(&internal_server_error),
            http::StatusCode::INTERNAL_SERVER_ERROR
        ));
    }
    let mut redis_conn = redis.unwrap();

    let exists = redis_conn.exists::<&String, bool>(&body.client_id);
    if let Err(_) = exists {
        return Ok(warp::reply::with_status(
            warp::reply::json(&internal_server_error),
            http::StatusCode::INTERNAL_SERVER_ERROR
        ));
    }

    if exists.unwrap() == true {
        return Ok(warp::reply::with_status(
            warp::reply::json(
                &new_error_response(vec![
                    ErrorReason {
                        field: "client_id".to_string(),
                        description: "A client is already registered with this id".to_string()
                    }
                ])
            ),
            http::StatusCode::BAD_REQUEST)
        );
    }

    if let Err(_) = redis_conn.set::<&String, String, ()>(&body.client_id, format!("{}:{}", body.push_type, body.token)) {
        return Ok(warp::reply::with_status(
            warp::reply::json(&internal_server_error),
            http::StatusCode::INTERNAL_SERVER_ERROR
        ));
    }

    // TODO Register webhook with relay.

    if let Some(metrics) = &state.metrics {
        metrics.registered_webhooks.add(1, &[
            opentelemetry::KeyValue::new("client.id", body.client_id),
            opentelemetry::KeyValue::new("client.type", body.push_type)
        ]);
    }

    Ok(warp::reply::with_status(
        warp::reply::json(&new_success_response()),
        http::StatusCode::OK
    ))
}