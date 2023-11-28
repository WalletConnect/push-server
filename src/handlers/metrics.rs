use {
    crate::{error::Result, log::prelude::*, state::AppState},
    axum::{extract::State, http::StatusCode},
    std::sync::Arc,
    tracing::instrument,
};

#[instrument(skip_all, name = "metrics_handler")]
pub async fn handler(State(state): State<Arc<AppState>>) -> Result<(StatusCode, String)> {
    if let Some(metrics) = &state.metrics {
        let exported = metrics.export()?;
        Ok((StatusCode::OK, exported))
    } else {
        // No Metrics!
        warn!("request for metrics while they are disabled");
        Ok((StatusCode::BAD_REQUEST, "Metrics not enabled.".to_string()))
    }
}
