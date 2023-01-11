use opentelemetry::Context;
use {
    crate::{error::Result, handlers::Response, state::AppState},
    axum::extract::{Path, State as StateExtractor},
    std::sync::Arc,
};

pub async fn handler(
    Path((tenant_id, id)): Path<(String, String)>,
    StateExtractor(state): StateExtractor<Arc<AppState>>,
) -> Result<Response> {
    state.client_store.delete_client(&tenant_id, &id).await?;

    if let Some(metrics) = &state.metrics {
        metrics.registered_clients.add(&Context::current(), -1, &[]);
    }

    Ok(Response::default())
}
