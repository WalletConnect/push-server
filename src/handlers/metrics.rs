use opentelemetry::sdk::export;
use {
    crate::state::AppState,
    axum::{extract::State, http::StatusCode, response::IntoResponse},
    std::sync::Arc,
};

pub async fn handler(State(state): State<Arc<AppState>>) -> impl IntoResponse {
   if let Some(metrics) = &state.metrics {
        let exported = metrics.export()?;

       (
           StatusCode::OK,
           exported,
       )
   } else {
       // No Metrics!
       (
           StatusCode::BAD_REQUEST,
           "Metrics not enabled.",
       )
   }
}