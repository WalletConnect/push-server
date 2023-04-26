use {
    crate::state::AppState,
    axum::{extract::State as ExtractState, http::StatusCode, response::IntoResponse},
    std::sync::Arc,
};

pub async fn handler(ExtractState(state): ExtractState<Arc<AppState>>) -> impl IntoResponse {
    (
        StatusCode::OK,
        format!("OK, echo-server v{}", state.build_info.crate_info.version,),
    )
}
