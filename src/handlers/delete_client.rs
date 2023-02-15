use {
    crate::{
        decrement_counter,
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

    decrement_counter!(state.metrics, registered_clients);

    Ok(Response::default())
}
