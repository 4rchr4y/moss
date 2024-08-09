use std::{path::PathBuf, sync::Arc};

use crate::common::workspace_configuration_model::WorkspaceConfiguration;
use anyhow::Context;
use configuration::common::{
    configuration_default::DefaultConfiguration,
    configuration_model::{ConfigurationModel, ConfigurationParser, UserConfiguration},
    configuration_policy::{ConfigurationPolicy, ConfigurationPolicyService},
    configuration_registry::ConfigurationRegistry,
};
use workspace::Workspace;

pub struct WorkspaceService {
    // workspace: Workspace,
    configuration: WorkspaceConfiguration,
    initialized: bool,
}

impl WorkspaceService {
    pub fn new(
        registry: Arc<ConfigurationRegistry>,
        policy_service: ConfigurationPolicyService,
    ) -> Self {
        let parser = ConfigurationParser::new(Arc::clone(&registry)); // TODO: platform ConfigurationParser?

        let default_configuration = DefaultConfiguration::new(Arc::clone(&registry)); // TODO: use WorkspaceDefaultConfiguration

        // TODO: use UserDataProfileService
        let config_file_path = &PathBuf::from("../../../.moss/settings.json");
        let user_configuration = UserConfiguration::new(config_file_path, Arc::new(parser));
        let user_configuration_model = user_configuration
            .load_configuration()
            .context("failed to load user configuration model")
            .unwrap();
        let default_configuration_model = default_configuration
            .get_configuration_model()
            .context("failed to get default configuration model".to_string())
            .context("default was not initialized correctly")
            .unwrap();

        let mut configuration_policy =
            ConfigurationPolicy::new(Arc::clone(&registry), policy_service);
        configuration_policy.initialize(&default_configuration);

        let policy_configuration_model = configuration_policy.get_model();

        Self {
            configuration: WorkspaceConfiguration::new(
                default_configuration_model,
                policy_configuration_model,
                Arc::new(user_configuration_model),
                Arc::new(ConfigurationModel::empty()),
                Arc::new(ConfigurationModel::empty()),
            ),
            initialized: false,
        }
    }
}
