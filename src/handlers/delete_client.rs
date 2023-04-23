use {
    crate::{
        decrement_counter,
        error::{Error::InvalidAuthentication, Result},
        handlers::{authenticate_client, Response, DECENTRALIZED_IDENTIFIER_PREFIX},
        log::prelude::*,
        request_id::get_req_id,
        state::AppState,
    },
    axum::{
        extract::{Path, State as StateExtractor},
        http::HeaderMap,
    },
    relay_rpc::domain::ClientId,
    std::sync::Arc,
};

pub async fn handler(
    Path((tenant_id, id)): Path<(String, String)>,
    StateExtractor(state): StateExtractor<Arc<AppState>>,
    headers: HeaderMap,
) -> Result<Response> {
    let request_id = get_req_id(&headers);

    let id = id
        .trim_start_matches(DECENTRALIZED_IDENTIFIER_PREFIX)
        .to_string();

    let client_to_be_deleted = ClientId::new(id.clone().into());
    if !authenticate_client(headers, &state.config.public_url, |client_id| {
        if let Some(client_id) = client_id {
            debug!(
                %request_id,
                %tenant_id,
                requested_client_id = %client_to_be_deleted,
                token_client_id = %client_id,
                "client_id authentication checking"
            );
            client_id == client_to_be_deleted
        } else {
            debug!(
                %request_id,
                %tenant_id,
                requested_client_id = %client_to_be_deleted,
                token_client_id = "unknown",
                "client_id verification failed: missing client_id"
            );
            false
        }
    })? {
        debug!(
            %request_id,
            %tenant_id,
            requested_client_id = %client_to_be_deleted,
            token_client_id = "unknown",
            "client_id verification failed: invalid client_id"
        );
        return Err(InvalidAuthentication);
    }

    state.client_store.delete_client(&tenant_id, &id).await?;
    info!("client ({}) deleted for tenant ({})", id, tenant_id);

    info!(
        %request_id,
        %tenant_id,
        client_id = %client_to_be_deleted,
        "deleted client"
    );

    decrement_counter!(state.metrics, registered_clients);

    Ok(Response::default())
}
