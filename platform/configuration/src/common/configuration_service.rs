use anyhow::{Context as AnyhowContext, Result};
use base::queue::{thread_backend::ThreadBackend, Processor, Queue};
use lazy_regex::Lazy;
use serde_json::Value;
use std::{
    fs::OpenOptions,
    io::{Read, Seek, SeekFrom, Write},
    path::PathBuf,
    sync::Arc,
};

use super::{
    configuration_default::DefaultConfiguration,
    configuration_model::{
        AttributeName, Configuration, ConfigurationModel, ConfigurationParser, UserConfiguration,
    },
    configuration_policy::{ConfigurationPolicy, ConfigurationPolicyService},
    configuration_registry::ConfigurationRegistry,
    AbstractConfigurationService,
};

pub struct ConfigurationService {
    default_configuration: DefaultConfiguration,
    user_configuration: UserConfiguration,
    configuration: Configuration,
    configuration_editing: ConfigurationEditingService,
    configuration_policy: ConfigurationPolicy,
}

impl ConfigurationService {
    pub fn new(
        registry: Arc<ConfigurationRegistry>,
        policy_service: ConfigurationPolicyService,
        config_file_path: &PathBuf,
    ) -> Result<Self> {
        let parser = ConfigurationParser::new(Arc::clone(&registry));
        let user_configuration = UserConfiguration::new(config_file_path, Arc::new(parser));

        let default_configuration = DefaultConfiguration::new(Arc::clone(&registry));
        default_configuration.initialize();

        let user_configuration_model = user_configuration
            .load_configuration()
            .context("failed to load user configuration model")?;
        let default_configuration_model = default_configuration
            .get_configuration_model()
            .context("failed to get default configuration model".to_string())
            .context("default was not initialized correctly")?;

        let mut configuration_policy =
            ConfigurationPolicy::new(Arc::clone(&registry), policy_service);
        configuration_policy.initialize(&default_configuration);

        let policy_configuration_model = configuration_policy.get_model();

        let configuration = Configuration::new(
            default_configuration_model,
            policy_configuration_model,
            Arc::new(user_configuration_model),
            Arc::new(ConfigurationModel::empty()),
            Arc::new(ConfigurationModel::empty()),
        );

        let configuration_editing = ConfigurationEditingService::new(config_file_path.clone());

        Ok(Self {
            default_configuration,
            user_configuration,
            configuration,
            configuration_editing,
            configuration_policy,
        })
    }

    fn reload_configuration(&self) -> Result<()> {
        let user_configuration_model = self
            .user_configuration
            .load_configuration()
            .context("failed to load user configuration model")?;

        // TODO: use the resulting difference to identify and notify those parts of the application whose configurations have changed
        let _diff = self
            .configuration
            .update_user_configuration(Arc::new(user_configuration_model));

        Ok(())
    }

    async fn do_update_value(&self, attribute_name: &AttributeName, value: &Value) -> Result<()> {
        let inspected_value = self.configuration.inspect(attribute_name);
        if inspected_value.get_policy_value(attribute_name).is_some() {
            return Err(anyhow!(
                "value `{}` is protected by policy and cannot be overwritten.",
                attribute_name.to_string()
            ));
        }

        if inspected_value
            .get_default_value(&attribute_name)
            .map_or(false, |default_value| default_value == value)
        {
            self.configuration_editing
                .write(attribute_name.to_string(), None)
                .await;
        } else {
            self.configuration_editing
                .write(attribute_name.to_string(), Some(value.clone()))
                .await;
        }

        Ok(())
    }
}

#[async_trait]
impl AbstractConfigurationService for ConfigurationService {
    fn get_value(&self, attribute_name: AttributeName) -> Option<Value> {
        self.configuration.get_value(&attribute_name)
    }

    /// NOTE: The function only works to update non-object values ​​at the root level
    async fn update_value(&self, attribute_name: AttributeName, value: &Value) -> Result<()> {
        // TODO:
        // - Use pointer instead of key
        // - Check if the setting being changed is a USER level setting

        self.do_update_value(&attribute_name, value).await?;
        Ok(self.reload_configuration()?)
    }
}

#[derive(Debug)]
pub struct ConfigurationEditingService {
    edited_resource: PathBuf,
    queue: Queue<ThreadBackend, ConfigurationWriteJob>,
}

#[derive(Debug)]
struct ConfigurationWriteJob {
    key: String, // JSON Path
    value: Option<serde_json::Value>,
    resource: PathBuf,
}

#[derive(Debug)]
struct ConfigurationWriteJobProcessor;

#[async_trait]
impl Processor<ConfigurationWriteJob> for ConfigurationWriteJobProcessor {
    async fn process(&self, job: ConfigurationWriteJob) {
        // TODO:
        // - Implement error handling when the event module is implemented
        // - Implement update logic for overriding and nested objects

        if let Ok(mut file) = OpenOptions::new().read(true).write(true).open(job.resource) {
            let mut content = String::new();
            file.read_to_string(&mut content).expect("read file");

            let mut json: Value = serde_json::from_str(&content).expect("read content");

            if let Some(obj) = json.as_object_mut() {
                if let Some(v) = job.value {
                    obj.insert(job.key, v);
                } else {
                    obj.remove(&job.key);
                }
            }

            file.seek(SeekFrom::Start(0)).expect("seek to start");
            file.set_len(0).expect("truncate file");

            let json_string = serde_json::to_string_pretty(&json).expect("write content");
            file.write_all(json_string.as_bytes()).expect("write");
        };
    }
}

impl ConfigurationEditingService {
    fn new(edited_resource: PathBuf) -> Self {
        Self {
            edited_resource,
            queue: Queue::new(
                Lazy::new(|| ThreadBackend::new()),
                ConfigurationWriteJobProcessor {},
            ),
        }
    }

    async fn write(&self, key: String, value: Option<serde_json::Value>) {
        self.queue
            .enqueue(ConfigurationWriteJob {
                key,
                value,
                resource: self.edited_resource.clone(),
            })
            .await
    }
}
