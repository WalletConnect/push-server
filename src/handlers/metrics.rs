use {
    crate::{error::Result, log::prelude::*, request_id::get_req_id, state::AppState},
    axum::{
        extract::State,
        http::{HeaderMap, StatusCode},
    },
    std::sync::Arc,
    tracing::debug,
};

pub async fn handler(
    State(state): State<Arc<AppState>>,
    headers: HeaderMap,
) -> Result<(StatusCode, String)> {
    let req_id = get_req_id(&headers);

    if let Some(metrics) = &state.metrics {
        let exported = metrics.export()?;

        debug!(request_id = req_id, "exported metrics");

        Ok((StatusCode::OK, exported))
    } else {
        // No Metrics!
        warn!(
            request_id = req_id,
            "request for metrics while they are disabled"
        );

        Ok((StatusCode::BAD_REQUEST, "Metrics not enabled.".to_string()))
    }
}
