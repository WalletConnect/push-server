use crate::error::Error;
use crate::handlers::{new_error_response, new_success_response, ErrorReason};
use crate::providers::{get_provider, PushProvider};
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
    state: Arc<State<impl crate::store::ClientStore>>,
    body: PushMessageBody,
) -> Result<impl warp::Reply, warp::Rejection> {
    let store = state.store.lock().unwrap();

    // TODO de-dup, and return accepted to already acknowledged notifications
    if body.id.as_str() == "0000-0000-0000-0000" {
        return Ok(warp::reply::with_status(
            warp::reply::json(&new_success_response()),
            http::StatusCode::ACCEPTED,
        ));
    }

    let client_result = store.get_client(&client_id);
    if let Err(_) = client_result {
        return Ok(warp::reply::with_status(
            warp::reply::json(&new_error_response(vec![])),
            http::StatusCode::INTERNAL_SERVER_ERROR,
        ));
    }
    let client = client_result.unwrap();
    if client.is_none() {
        return Ok(warp::reply::with_status(
            warp::reply::json(&new_error_response(vec![ErrorReason {
                field: "id".to_string(),
                description: "No client found with the provided id".to_string(),
            }])),
            http::StatusCode::NOT_FOUND,
        ));
    }

    let provider_name = &client.unwrap().push_type;
    let provider_result = get_provider(provider_name.clone());
    if let Err(err) = &provider_result {
        match err {
            Error::ProviderNotFound(_) => {
                // NOT POSSIBLE IN THEORY!
                return Ok(warp::reply::with_status(
                    warp::reply::json(&new_error_response(vec![ErrorReason {
                        field: "client.provider".to_string(),
                        description: "The client's registered provider cannot be found."
                            .to_string(),
                    }])),
                    http::StatusCode::NOT_FOUND,
                ));
            }
            Error::ProviderNotAvailable(_) => {
                return Ok(warp::reply::with_status(
                    warp::reply::json(&new_error_response(vec![ErrorReason {
                        field: "client.provider".to_string(),
                        description: "The client's registered provider is not available."
                            .to_string(),
                    }])),
                    http::StatusCode::NOT_FOUND,
                ));
            }
            // Cannot be any other error
            _ => {}
        }
    }
    let mut provider = provider_result.unwrap();

    match provider.send_notification(client.unwrap().token.clone(), body.payload.message) {
        Ok(_) => Ok(warp::reply::with_status(
            warp::reply::json(&new_success_response()),
            http::StatusCode::ACCEPTED,
        )),
        Err(_err) => Ok(warp::reply::with_status(
            warp::reply::json(&new_error_response(vec![])),
            http::StatusCode::INTERNAL_SERVER_ERROR,
        )),
    }
}
