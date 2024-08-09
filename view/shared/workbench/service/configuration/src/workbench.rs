use std::{path::PathBuf, sync::Arc};

use anyhow::Result;
use configuration::common::{
    configuration_policy::ConfigurationPolicyService,
    configuration_registry::ConfigurationRegistry, configuration_service::ConfigurationService,
};

// TODO: ServiceIdentifier
pub struct ServiceCollection {
    // TODO: map
}

pub struct Workbench {}

impl Workbench {
    pub fn new(
        registry: ConfigurationRegistry,
        policy_service: ConfigurationPolicyService,
    ) -> Result<Self> {
        let config_service = ConfigurationService::new(
            Arc::new(registry),
            policy_service,
            &PathBuf::from("../../../.moss/settings.json"),
        )
        .unwrap();

        Ok(Self {})
    }
}
