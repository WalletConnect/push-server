use {
    crate::state::{AppState, State},
    axum::{extract::State as ExtractState, http::StatusCode, response::IntoResponse},
    std::sync::Arc,
};

pub async fn handler(ExtractState(state): ExtractState<Arc<AppState>>) -> impl IntoResponse {
    (
        StatusCode::OK,
        format!(
            "OK, echo-server v{}\nmultitenant: {}\ntelemetry: {}\nfeatures: {}",
            state.build_info.crate_info.version,
            state.is_multitenant(),
            state.metrics.is_some(),
            state.build_info.crate_info.enabled_features.join(",")
        ),
    )
}
