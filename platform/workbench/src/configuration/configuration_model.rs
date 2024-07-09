use anyhow::Result;
use arc_swap::{ArcSwap, ArcSwapOption};
use hashbrown::HashMap;
use serde_json::Value;
use std::{fs::File, io::Read, sync::Arc};

#[derive(Debug, Clone)]
pub struct Override {
    identifier: String,
    _keys: Vec<String>,
    contents: HashMap<String, Value>,
}

#[derive(Debug, Clone)]
pub struct ConfigurationLayer {
    content: HashMap<String, Value>,
    keys: Vec<String>,
    overrides: Vec<Override>,
    overridden_configurations: Arc<ArcSwap<HashMap<String, Arc<ConfigurationLayer>>>>,
}

impl ConfigurationLayer {
    pub fn new(
        contents: HashMap<String, Value>,
        keys: Vec<String>,
        overrides: Vec<Override>,
    ) -> Self {
        Self {
            content: contents,
            keys,
            overrides,
            overridden_configurations: Arc::new(ArcSwap::new(Arc::new(HashMap::new()))),
        }
    }

    pub fn empty() -> Self {
        Self {
            content: HashMap::new(),
            keys: Vec::new(),
            overrides: Vec::new(),
            overridden_configurations: Arc::new(ArcSwap::new(Arc::new(HashMap::new()))),
        }
    }

    pub fn get_value(&self, key: &str) -> Option<&Value> {
        self.content.get(key)
    }

    fn override_configuration(&self, identifier: &str) -> Arc<Self> {
        let current_overrides = self.overridden_configurations.load_full();

        if let Some(override_model) = current_overrides.get(identifier) {
            return Arc::clone(override_model);
        }

        let new_override = Arc::new(self.create_overridden_configuration(identifier));

        let mut new_overrides = HashMap::clone(&*current_overrides);
        new_overrides.insert(identifier.to_string(), Arc::clone(&new_override));

        self.overridden_configurations
            .store(Arc::new(new_overrides));

        new_override
    }

    fn create_overridden_configuration(&self, identifier: &str) -> Self {
        if let Some(override_content) = self.get_override_identifier_content(identifier) {
            let mut content = self.content.clone();
            content.extend(override_content);

            ConfigurationLayer::new(content, self.keys.clone(), self.overrides.clone())
        } else {
            self.clone()
        }
    }

    fn get_override_identifier_content(&self, identifier: &str) -> Option<HashMap<String, Value>> {
        self.overrides
            .iter()
            .find(|override_data| override_data.identifier == identifier)
            .map(|override_data| override_data.contents.clone())
    }

    fn merge(&self, others: Vec<ConfigurationLayer>) -> Self {
        let mut merged = self.clone();
        for other in others {
            merged.content.extend(other.content.clone());

            let new_keys: Vec<String> = other
                .keys
                .into_iter()
                .filter(|key| !merged.keys.contains(key))
                .collect();

            merged.keys.extend(new_keys);
            merged.overrides.extend(other.overrides);
        }

        merged
    }
}
// TODO: Use kernel/fs to work with the file system
pub struct Parser;

impl Parser {
    pub fn new() -> Self {
        Self {}
    }

    pub fn parse_file(&self, file_path: &str) -> Result<ConfigurationLayer> {
        let re_override_property = regex!(r#"^\[.*\]$"#);

        let mut file = File::open(file_path)?;
        let mut content = String::new();
        file.read_to_string(&mut content)?;

        let root_map: HashMap<String, Value> = serde_json::from_str(&content)?;
        let mut root_overrides: Vec<Override> = Vec::new();
        let mut root_contents: HashMap<String, Value> = HashMap::new();
        let mut root_keys: Vec<String> = Vec::new();

        for (k, v) in &root_map {
            if re_override_property.is_match(k) {
                Self::collect_overrides(k, v, &mut root_overrides, None);
            } else {
                root_contents.insert(k.clone(), v.clone());
                root_keys.push(k.clone());
            }
        }

        let result = ConfigurationLayer::new(root_contents, root_keys, root_overrides);

        dbg!(&result);

        Ok(result)
    }

    fn collect_overrides(
        key: &str,
        value: &Value,
        overrides: &mut Vec<Override>,
        parent_identifier: Option<&str>,
    ) {
        if let Value::Object(inner_map) = value {
            let current_identifier = Self::format_identifier(parent_identifier, key);

            let (override_content, override_keys) = Self::extract_override_content_and_keys(
                inner_map,
                overrides,
                Some(&current_identifier),
            );

            overrides.push(Override {
                identifier: current_identifier.to_string(),
                _keys: override_keys,
                contents: override_content,
            });
        }
    }

    fn extract_override_content_and_keys(
        inner_map: &serde_json::Map<std::string::String, Value>,
        overrides: &mut Vec<Override>,
        current_identifier: Option<&str>,
    ) -> (HashMap<String, Value>, Vec<String>) {
        let mut override_content = HashMap::new();
        let mut override_keys = Vec::new();

        for (inner_key, inner_value) in inner_map {
            if inner_key.starts_with('[') && inner_key.ends_with(']') {
                Self::collect_overrides(inner_key, inner_value, overrides, current_identifier);
            } else {
                override_content.insert(inner_key.clone(), inner_value.clone());
                override_keys.push(inner_key.clone());
            }
        }

        (override_content, override_keys)
    }

    fn format_identifier(parent_identifier: Option<&str>, key: &str) -> String {
        let trimmed_key = key.trim_matches(|c| c == '[' || c == ']');

        parent_identifier.map_or_else(
            || trimmed_key.to_string(),
            |parent_id| {
                let mut result = String::from(parent_id);
                result.push_str("/");
                result.push_str(trimmed_key);

                result
            },
        )
    }
}

#[derive(Debug)]
pub struct Configuration {
    default_configuration: ConfigurationLayer,
    user_configuration: ConfigurationLayer,
    workspace_configuration: ConfigurationLayer,
    inmem_configuration: ConfigurationLayer,
    consolidated_configuration: ArcSwapOption<ConfigurationLayer>,
}

impl Configuration {
    pub fn new(
        default_conf: ConfigurationLayer,
        user_conf: ConfigurationLayer,
        workspace_conf: ConfigurationLayer,
        inmem_conf: ConfigurationLayer,
    ) -> Self {
        Configuration {
            default_configuration: default_conf,
            user_configuration: user_conf,
            workspace_configuration: workspace_conf,
            inmem_configuration: inmem_conf,
            consolidated_configuration: ArcSwapOption::from(None),
        }
    }

    pub fn get_value(&self, key: &str, overrider_identifier: Option<&str>) -> Option<Value> {
        let consolidated_conf = self.get_consolidated_configuration(overrider_identifier);
        consolidated_conf.get_value(key).cloned()
    }

    pub fn get_consolidated_configuration(
        &self,
        overrider_identifier: Option<&str>,
    ) -> Arc<ConfigurationLayer> {
        if let Some(config) = self.consolidated_configuration.load_full().as_ref() {
            if let Some(identifier) = overrider_identifier {
                return config.override_configuration(identifier.trim_start_matches('/'));
            }

            return Arc::clone(config);
        }

        let new_configuration = {
            let merged_configuration = self.default_configuration.merge(vec![
                self.user_configuration.clone(),
                self.workspace_configuration.clone(),
                self.inmem_configuration.clone(),
            ]);

            Arc::new(merged_configuration)
        };

        self.consolidated_configuration
            .store(Some(Arc::clone(&new_configuration)));

        if let Some(identifier) = overrider_identifier {
            return new_configuration.override_configuration(identifier);
        }

        new_configuration
    }
}
