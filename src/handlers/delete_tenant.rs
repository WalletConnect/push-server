use {
    crate::{error::Error, state::AppState},
    axum::{
        extract::{Path, State},
        Json,
    },
    serde::Serialize,
    std::sync::Arc,
};

#[derive(Serialize)]
pub struct DeleteTenantResponse {
    success: bool,
}

pub async fn handler(
    State(state): State<Arc<AppState>>,
    Path(id): Path<String>,
) -> Result<Json<DeleteTenantResponse>, Error> {
    state.tenant_store.delete_tenant(&id).await?;

    Ok(Json(DeleteTenantResponse { success: true }))
}