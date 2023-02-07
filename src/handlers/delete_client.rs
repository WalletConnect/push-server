use {
    crate::{
        error::Result,
        handlers::{Response, DECENTRALIZED_IDENTIFIER_PREFIX},
        log::prelude::*,
        state::AppState,
    },
    axum::extract::{Path, State as StateExtractor},
    opentelemetry::Context,
    std::sync::Arc,
};

pub async fn handler(
    Path((tenant_id, id)): Path<(String, String)>,
    StateExtractor(state): StateExtractor<Arc<AppState>>,
) -> Result<Response> {
    let id = id
        .trim_start_matches(DECENTRALIZED_IDENTIFIER_PREFIX)
        .to_string();

    state.client_store.delete_client(&tenant_id, &id).await?;
    info!("client ({}) deleted for tenant ({})", id, tenant_id);

    if let Some(metrics) = &state.metrics {
        metrics.registered_clients.add(&Context::current(), -1, &[]);
        debug!("decremented `registered_clients` counter")
    }

    Ok(Response::default())
}
