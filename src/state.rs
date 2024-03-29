#[cfg(feature = "cloud")]
use cerberus::registry::RegistryHttpClient;
use {
    crate::{
        config::Config,
        metrics::Metrics,
        networking,
        relay::RelayClient,
        stores::{client::ClientStore, notification::NotificationStore, tenant::TenantStore},
    },
    build_info::BuildInfo,
    std::{net::IpAddr, sync::Arc},
    wc::geoip::{block::middleware::GeoBlockLayer, MaxMindResolver},
};

#[cfg(feature = "analytics")]
use crate::analytics::PushAnalytics;
#[cfg(feature = "multitenant")]
use crate::supabase::GoTrueClient;

pub type ClientStoreArc = Arc<dyn ClientStore + Send + Sync + 'static>;
pub type NotificationStoreArc = Arc<dyn NotificationStore + Send + Sync + 'static>;
pub type TenantStoreArc = Arc<dyn TenantStore + Send + Sync + 'static>;

pub trait State {
    fn config(&self) -> Config;
    fn build_info(&self) -> BuildInfo;
    fn client_store(&self) -> ClientStoreArc;
    fn notification_store(&self) -> NotificationStoreArc;
    fn tenant_store(&self) -> TenantStoreArc;
    fn relay_client(&self) -> RelayClient;
    #[cfg(feature = "cloud")]
    fn registry_client(&self) -> RegistryHttpClient;
    fn is_multitenant(&self) -> bool;
    fn validate_signatures(&self) -> bool;
}

#[derive(Clone)]
pub struct AppState {
    pub config: Config,
    pub build_info: BuildInfo,
    pub metrics: Option<Metrics>,
    #[cfg(feature = "analytics")]
    pub analytics: Option<PushAnalytics>,
    pub client_store: ClientStoreArc,
    pub notification_store: NotificationStoreArc,
    pub tenant_store: TenantStoreArc,
    pub relay_client: RelayClient,
    #[cfg(feature = "cloud")]
    pub registry_client: RegistryHttpClient,
    #[cfg(feature = "multitenant")]
    pub gotrue_client: GoTrueClient,
    pub public_ip: Option<IpAddr>,
    is_multitenant: bool,
    pub geoblock: Option<GeoBlockLayer<Arc<MaxMindResolver>>>,
    /// Service instance identifier
    pub instance_id: uuid::Uuid,
    /// Service instance uptime measurement
    pub uptime: std::time::Instant,
}

build_info::build_info!(fn build_info);

pub fn new_state(
    config: Config,
    client_store: ClientStoreArc,
    notification_store: NotificationStoreArc,
    tenant_store: TenantStoreArc,
) -> crate::error::Result<AppState> {
    let build_info: &BuildInfo = build_info();

    #[cfg(feature = "multitenant")]
    let is_multitenant = true;

    #[cfg(not(feature = "multitenant"))]
    let is_multitenant = false;

    #[cfg(feature = "cloud")]
    let (cloud_url, cloud_api_key) = (config.cloud_api_url.clone(), config.cloud_api_key.clone());

    #[cfg(feature = "multitenant")]
    let jwt_secret = config.jwt_secret.clone();

    let public_ip = match networking::find_public_ip_addr() {
        Ok(ip) => Some(ip),
        // Note: Should we pass this error back up?
        Err(_e) => None,
    };

    Ok(AppState {
        config: config.clone(),
        build_info: build_info.clone(),
        metrics: None,
        #[cfg(feature = "analytics")]
        analytics: None,
        client_store,
        notification_store,
        tenant_store,
        relay_client: RelayClient::new(config.relay_public_key)?,
        #[cfg(feature = "cloud")]
        registry_client: RegistryHttpClient::new(cloud_url, cloud_api_key.as_str())?,
        #[cfg(feature = "multitenant")]
        gotrue_client: GoTrueClient::new(jwt_secret),
        public_ip,
        is_multitenant,
        geoblock: None,
        instance_id: uuid::Uuid::new_v4(),
        uptime: std::time::Instant::now(),
    })
}

impl AppState {
    pub fn set_metrics(&mut self, metrics: Metrics) {
        self.metrics = Some(metrics);
    }
}

impl State for Arc<AppState> {
    fn config(&self) -> Config {
        self.config.clone()
    }

    fn build_info(&self) -> BuildInfo {
        self.build_info.clone()
    }

    fn client_store(&self) -> ClientStoreArc {
        self.client_store.clone()
    }

    fn notification_store(&self) -> NotificationStoreArc {
        self.notification_store.clone()
    }

    fn tenant_store(&self) -> TenantStoreArc {
        self.tenant_store.clone()
    }

    fn relay_client(&self) -> RelayClient {
        self.relay_client.clone()
    }

    #[cfg(feature = "cloud")]
    fn registry_client(&self) -> RegistryHttpClient {
        self.registry_client.clone()
    }

    fn is_multitenant(&self) -> bool {
        self.is_multitenant
    }

    fn validate_signatures(&self) -> bool {
        self.config.validate_signatures
    }
}
