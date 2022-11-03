use crate::state::AppState;
use crate::stores::client::ClientStore;
use crate::stores::notification::NotificationStore;
use axum::extract::State;
use axum::http::StatusCode;
use axum::response::IntoResponse;
use std::sync::Arc;
use crate::stores::tenant::TenantStore;

pub async fn handler(
    State(state): State<Arc<AppState<impl ClientStore, impl NotificationStore, impl TenantStore>>>,
) -> impl IntoResponse {
    (
        StatusCode::OK,
        format!("OK, echo-server v{}", state.build_info.crate_info.version),
    )
}
