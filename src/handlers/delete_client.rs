use crate::handlers::{new_error_response, new_success_response, ErrorReason};
use crate::State;
use std::sync::Arc;
use warp::http;

pub async fn handler(
    id: String,
    state: Arc<State<impl crate::store::ClientStore>>,
) -> Result<impl warp::Reply, warp::Rejection> {
    let mut store = state.store.lock().unwrap();

    let exists = store.get_client(id.clone());
    if let Err(_) = exists {
        return Ok(warp::reply::with_status(
            warp::reply::json(&new_error_response(vec![])),
            http::StatusCode::INTERNAL_SERVER_ERROR,
        ));
    }

    if exists.unwrap().is_none() {
        return Ok(warp::reply::with_status(
            warp::reply::json(&new_error_response(vec![ErrorReason {
                field: "client_id".to_string(),
                description: "No client is registered with the supplied id".to_string(),
            }])),
            http::StatusCode::BAD_REQUEST,
        ));
    }

    if let Err(_) = store.delete_client(id.clone()) {
        return Ok(warp::reply::with_status(
            warp::reply::json(&new_error_response(vec![])),
            http::StatusCode::INTERNAL_SERVER_ERROR,
        ));
    }

    if let Some(metrics) = &state.metrics {
        metrics
            .registered_webhooks
            .add(-1, &[opentelemetry::KeyValue::new("client.id", id)]);
    }

    let response = warp::reply::with_status(
        warp::reply::json(&new_success_response()),
        http::StatusCode::OK,
    );

    Ok(response)
}
