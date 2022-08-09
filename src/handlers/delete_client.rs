use std::sync::Arc;
use redis::Commands;
use warp::http;
use crate::handlers::{ErrorReason, new_error_response, new_success_response};
use crate::State;

pub async fn handler(id: String, state: Arc<State>) -> Result<impl warp::Reply, warp::Rejection> {
    let internal_server_error = new_error_response(vec![]);

    let redis = state.get_redis_connection();
    if let Err(_) = redis {
        return Ok(warp::reply::with_status(
            warp::reply::json(&internal_server_error),
            http::StatusCode::INTERNAL_SERVER_ERROR
        ));
    }
    let mut redis_conn = redis.unwrap();

    let exists = redis_conn.exists::<&String, bool>(&id);
    if let Err(_) = exists {
        return Ok(warp::reply::with_status(
            warp::reply::json(&internal_server_error),
            http::StatusCode::INTERNAL_SERVER_ERROR
        ));
    }

    if exists.unwrap() == false {
        return Ok(warp::reply::with_status(
            warp::reply::json(
                &new_error_response(vec![
                    ErrorReason {
                        field: "client_id".to_string(),
                        description: "No client is registered with the supplied id".to_string()
                    }
                ])
            ),
            // TODO should be 404?
            http::StatusCode::BAD_REQUEST)
        );
    }

    if let Err(_) = redis_conn.del::<&String, ()>(&id) {
        return Ok(warp::reply::with_status(
            warp::reply::json(&internal_server_error),
            http::StatusCode::INTERNAL_SERVER_ERROR
        ));
    }

    let response = warp::reply::with_status(
        warp::reply::json(&new_success_response()),
        http::StatusCode::OK
    );

    Ok(response)
}