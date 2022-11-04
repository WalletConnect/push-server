use crate::error::Result;
use crate::handlers::Response;
use crate::state::AppState;
use axum::extract::{Path, State};
use std::sync::Arc;

pub async fn handler(
    Path((_tenant, id)): Path<(String, String)>,
    State(state): State<Arc<AppState>>,
) -> Result<Response> {
    state.client_store.delete_client(&id).await?;

    if let Some(metrics) = &state.metrics {
        metrics.registered_webhooks.add(-1, &[]);
    }

    Ok(Response::default())
}
