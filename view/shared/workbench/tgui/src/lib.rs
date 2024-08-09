use anyhow::Result;
use configuration::{
    attribute_name,
    common::{configuration_service::ConfigurationService, AbstractConfigurationService},
};
use platform_formation_common::service_group::ServiceGroup;

pub struct Workbench {}

impl Workbench {
    pub fn new(service_group: ServiceGroup) -> Result<Self> {
        let config_service = service_group.get::<ConfigurationService>();

        let value = config_service
            .as_ref()
            .get_value(attribute_name!(editor.fontSize));
        println!("Value `editor.fontSize` form None: {:?}", value);

        Ok(Self {})
    }
}
