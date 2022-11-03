use crate::error::Result;
use crate::handlers::Response;
use crate::state::AppState;
use crate::stores::client::ClientStore;
use crate::stores::notification::NotificationStore;
use crate::stores::tenant::TenantStore;
use axum::extract::{Path, State};
use std::sync::Arc;

pub async fn handler(
    Path(tenant): Path<String>,
    Path(id): Path<String>,
    State(state): State<Arc<AppState<impl ClientStore, impl NotificationStore, impl TenantStore>>>,
) -> Result<Response> {
    state.client_store.delete_client(&id).await?;

    if let Some(metrics) = &state.metrics {
        metrics.registered_webhooks.add(-1, &[]);
    }

    Ok(Response::default())
}
