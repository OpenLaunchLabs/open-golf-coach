/// Minimal types for configuring Nova discovery without performing any I/O.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NovaDiscoveryMethod {
    /// SSDP should be attempted first.
    Ssdp,
    /// mDNS discovery is preferred.
    Mdns,
    /// Use a manually provided endpoint instead of discovery.
    Manual,
}

/// Host and port for a Nova OpenAPI endpoint.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct NovaEndpoint {
    pub host: String,
    pub port: u16,
}

/// Configuration for Nova discovery.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct NovaDiscoveryConfig {
    pub method: NovaDiscoveryMethod,
    pub discovery_timeout_secs: u64,
    pub reconnect_delay_secs: u64,
    pub manual_endpoint: Option<NovaEndpoint>,
}

impl Default for NovaDiscoveryConfig {
    fn default() -> Self {
        Self {
            method: NovaDiscoveryMethod::Ssdp,
            discovery_timeout_secs: 5,
            reconnect_delay_secs: 3,
            manual_endpoint: None,
        }
    }
}
