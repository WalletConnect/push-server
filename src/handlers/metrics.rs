use {
    crate::{error::Result, request_id::get_req_id, state::AppState},
    axum::{extract::State, http::StatusCode},
    hyper::{Body, Request},
    std::sync::Arc,
    tracing::debug,
};

pub async fn handler(
    State(state): State<Arc<AppState>>,
    req: Request<Body>,
) -> Result<(StatusCode, String)> {
    let req_id = get_req_id(&req);

    if let Some(metrics) = &state.metrics {
        let exported = metrics.export()?;

        debug!(request_id = req_id, "exported metrics");

        Ok((StatusCode::OK, exported))
    } else {
        // No Metrics!
        warning!(
            request_id = req_id,
            "request for metrics while they are disabled"
        );

        Ok((StatusCode::BAD_REQUEST, "Metrics not enabled.".to_string()))
    }
}
