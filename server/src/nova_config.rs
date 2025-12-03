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

/// Errors that can occur while validating a configuration.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum NovaConfigError {
    /// Manual mode was selected but no endpoint was provided.
    MissingManualEndpoint,
}

impl NovaDiscoveryConfig {
    /// Ensure the configuration is self-consistent.
    pub fn validate(&self) -> Result<(), NovaConfigError> {
        if matches!(self.method, NovaDiscoveryMethod::Manual) && self.manual_endpoint.is_none() {
            Err(NovaConfigError::MissingManualEndpoint)
        } else {
            Ok(())
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default_is_ssdp_with_timeouts() {
        let cfg = NovaDiscoveryConfig::default();
        assert_eq!(cfg.method, NovaDiscoveryMethod::Ssdp);
        assert_eq!(cfg.discovery_timeout_secs, 5);
        assert_eq!(cfg.reconnect_delay_secs, 3);
        assert!(cfg.manual_endpoint.is_none());
        assert_eq!(cfg.validate(), Ok(()));
    }

    #[test]
    fn manual_requires_endpoint() {
        let cfg = NovaDiscoveryConfig {
            method: NovaDiscoveryMethod::Manual,
            ..Default::default()
        };

        assert_eq!(
            cfg.validate(),
            Err(NovaConfigError::MissingManualEndpoint)
        );
    }

    #[test]
    fn manual_with_endpoint_is_valid() {
        let cfg = NovaDiscoveryConfig {
            method: NovaDiscoveryMethod::Manual,
            manual_endpoint: Some(NovaEndpoint {
                host: "127.0.0.1".to_string(),
                port: 2921,
            }),
            ..Default::default()
        };

        assert_eq!(cfg.validate(), Ok(()));
    }
}
