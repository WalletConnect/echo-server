use std::net::SocketAddr;

use {
    crate::{
        servers::RelayServer,
        storage::{MockMailboxStorage, MockStorage},
        wsclient::{self, WsClient},
    },
    async_trait::async_trait,
    relay::{
        message_hub::{mailbox::MailboxMessage, routing_table::RoutingTableEntry},
        ProjectDataResponse,
        Storages,
    },
    std::sync::Arc,
    test_context::AsyncTestContext,
};

pub struct SingleNodeContext {
    pub relay: RelayServer,
    pub client_1: WsClient,
    pub client_2: WsClient,
}

#[async_trait]
impl AsyncTestContext for SingleNodeContext {
    async fn setup() -> Self {
        let relay = EchoServer::start(storages, None).await;
    }
}

pub struct EchoServer {
    pub public_addr: SocketAddr,
    pub private_addr: SocketAddr,
    shutdown_signal: tokio::sync::broadcast::Sender<()>,
    is_shutdown: bool,
}

impl EchoServer {
    pub async fn start(storages: Storages, websocket_disconnect_timeout_ms: Option<u64>) -> Self {
        echo_server::bootstap(state, "multi_tenant").await.unwrap();
    }
}
