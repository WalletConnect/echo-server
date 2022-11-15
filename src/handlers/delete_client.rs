use crate::error::Error::IncludedTenantIdWhenNotNeeded;
use crate::error::Result;
use crate::handlers::Response;
use crate::state::{AppState, State};
use axum::extract::{Path, State as StateExtractor};
use std::sync::Arc;

pub async fn handler(
    Path((tenant_id, id)): Path<(String, String)>,
    StateExtractor(state): StateExtractor<Arc<AppState>>,
) -> Result<Response> {
    if state.config.default_tenant_id != tenant_id && !state.is_multitenant() {
        return Err(IncludedTenantIdWhenNotNeeded);
    }

    state.client_store.delete_client(&tenant_id, &id).await?;

    if let Some(metrics) = &state.metrics {
        metrics.registered_webhooks.add(-1, &[]);
    }

    Ok(Response::default())
}
