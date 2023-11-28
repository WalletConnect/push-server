use {
    crate::state::AppState,
    axum::{extract::State as ExtractState, http::StatusCode, response::IntoResponse},
    std::sync::Arc,
};

pub async fn handler(ExtractState(state): ExtractState<Arc<AppState>>) -> impl IntoResponse {
    let build_commit = match state.build_info.version_control.clone() {
        Some(v) => v.git().unwrap().commit_short_id.clone(),
        None => String::new(),
    };
    (
        StatusCode::OK,
        format!(
            "OK v{}, commit hash: {}, features: {:?}, instance id: {}, uptime: {} seconds",
            state.build_info.crate_info.version,
            build_commit,
            state.build_info.crate_info.enabled_features,
            state.instance_id,
            state.uptime.elapsed().as_secs(),
        ),
    )
}
