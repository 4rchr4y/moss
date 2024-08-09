use anyhow::Result;
use arc_swap::{ArcSwap, ArcSwapOption};
use hashbrown::{HashMap, HashSet};
use lazy_regex::{Lazy, Regex};
use moss_base::collection::Extend;
use radix_trie::{Trie, TrieCommon};
use serde_json::Value;
use std::{fs::File, io::Read, path::PathBuf, sync::Arc};

use super::configuration_registry::ConfigurationRegistry;

pub struct AttributeName {
    pub override_ident: Option<String>,
    pub name: Option<String>,
}

impl AttributeName {
    pub(super) fn format(name: impl std::fmt::Display) -> String {
        format!("$.{}", name)
    }

    pub(super) fn format_with_override(
        name: impl std::fmt::Display,
        override_ident: impl std::fmt::Display,
    ) -> String {
        format!("$.[{}].{}", override_ident, name)
    }

    pub fn must_get_name(&self) -> String {
        self.name
            .clone()
            .unwrap_or(String::from("<undefined_attribute_name>"))
    }

    pub fn is_with_override(&self) -> bool {
        self.override_ident.is_some()
    }
}

impl ToString for AttributeName {
    fn to_string(&self) -> String {
        let mut result = String::from("$.");

        let override_part = self
            .override_ident
            .as_ref()
            .map_or(String::new(), |ident| format!("[{}]", ident));

        let name_part = self.name.as_ref().map_or(String::new(), |name| {
            if self.override_ident.is_some() {
                format!(".{}", name)
            } else {
                name.clone()
            }
        });

        result.push_str(&override_part);
        result.push_str(&name_part);

        result
    }
}

/// A macro to create an `AttributeName` struct with optional override identifier or name.
///
/// # Examples
///
/// Basic usage with no overrides:
///
/// ```rust
/// let attribute_name = attribute_name!(editor.fontSize);
/// assert_eq!(attribute_name.override_ident, None);
/// assert_eq!(attribute_name.name, Some("editor.fontSize".to_string()));
/// ```
///
/// Usage with an override:
///
/// ```rust
/// let attribute_name = attribute_name!([rust].editor.fontSize);
/// assert_eq!(attribute_name.override_ident, Some("rust".to_string()));
/// assert_eq!(attribute_name.name, Some("editor.fontSize".to_string()));
/// ```
///
/// Usage with an override only:
///
/// ```rust
/// let attribute_name = attribute_name!([rust]);
/// assert_eq!(attribute_name.override_ident, Some("rust".to_string()));
/// assert_eq!(attribute_name.name, None);
/// ```
#[macro_export]
macro_rules! attribute_name {
    // Handle override with sub-identifiers
    ([$override:ident] . $ident:ident $(. $subident:ident)*) => {{
        let override_ident = Some(stringify!($override).to_string());
        let name = Some(concat!(stringify!($ident), $(concat!(".", stringify!($subident))),*).to_string());
        $crate::common::configuration_model::AttributeName {
            override_ident,
            name,
        }
    }};

    // Handle override without sub-identifiers
    ([$override:ident]) => {{
        let override_ident = Some(stringify!($override).to_string());
        $crate::common::configuration_model::AttributeName {
            override_ident,
            name: None,
        }
    }};

    // Handle no override with sub-identifiers
    ($ident:ident $(. $subident:ident)*) => {{
        let name = Some(concat!(stringify!($ident), $(concat!(".", stringify!($subident))),*).to_string());
        $crate::common::configuration_model::AttributeName {
            override_ident: None,
            name,
        }
    }};
}

// TODO:
// - Use a kernel/fs to work with the file system
// - Use a LogService.
// - Use a PolicyService
pub struct UserConfiguration {
    parser: Arc<ConfigurationParser>,
    resource: PathBuf,
}

impl UserConfiguration {
    pub fn new(file_path: &PathBuf, content_parser: Arc<ConfigurationParser>) -> Self {
        Self {
            parser: content_parser,
            resource: file_path.clone(),
        }
    }

    pub fn load_configuration(&self) -> Result<ConfigurationModel> {
        let mut file = File::open(&self.resource)?;
        let mut content = String::new();
        file.read_to_string(&mut content)?;

        if content.trim().is_empty() {
            content = String::from("{}")
        }

        Ok(self.parser.parse(&content)?)
    }
}

/// Enum representing the various configuration targets in Moss Compass.
/// These targets specify where the configuration settings should be applied.
#[derive(Debug)]
pub enum ConfigurationTarget {
    /// Platform specific configuration.
    /// These settings apply to the entire application and cannot be overridden in local, workspace, etc.
    Platform,

    /// User specific configuration.
    /// These settings apply globally to the user and can be overridden in workspace or others settings.
    User,

    /// Workspace specific configuration.
    /// These settings apply to the specific workspace and can override by`ConfigurationTarget::User` settings.
    Workspace,

    /// Default configuration.
    /// These settings represent the default values provided by Moss Compass and can be overridden by any other configuration target.
    Default,

    /// Memory specific configuration.
    /// These settings are stored in memory and are not persisted. They can be used for temporary configurations.
    Memory,
}

#[derive(Debug)]
pub struct ConfigurationModel {
    pub(super) content: Trie<String, Value>,
    pub(super) names: Vec<String>,
    pub(super) overrides: Vec<String>,
}

impl Clone for ConfigurationModel {
    fn clone(&self) -> Self {
        ConfigurationModel {
            content: self.content.clone(),
            names: self.names.clone(),
            overrides: self.overrides.clone(),
        }
    }
}

impl ConfigurationModel {
    pub fn new(content: Trie<String, Value>, names: Vec<String>, overrides: Vec<String>) -> Self {
        Self {
            content,
            names,
            overrides,
        }
    }

    pub fn empty() -> Self {
        Self {
            content: Trie::new(),
            names: Vec::new(),
            overrides: Vec::new(),
        }
    }

    pub fn is_empty(&self) -> bool {
        self.content.is_empty() && self.names.len() == 0 && self.overrides.len() == 0
    }

    pub fn get_attribute_names(&self) -> &Vec<String> {
        &self.names
    }

    pub fn get_value(&self, attribute_name: &AttributeName) -> Option<&Value> {
        self.content.get(&attribute_name.to_string())
    }

    pub fn set_value(&mut self, attribute_name: String, value: serde_json::Value) {
        self.content.insert(attribute_name.clone(), value);
        self.names.push(attribute_name);
    }

    // TODO:
    // fn get_override_identifier_content(&self, identifier: &str) -> Option<HashMap<String, Value>> {
    //     self.overrides
    //         .iter()
    //         .find(|override_data| override_data.identifier == identifier)
    //         .map(|override_data| override_data.content.clone())
    // }

    pub fn merge(&self, others: &[Arc<ConfigurationModel>]) -> Self {
        let mut result = self.clone();

        for other in others {
            result.content.extend(other.content.iter());
            result.names.extend(
                other
                    .names
                    .iter()
                    .filter(|key| !result.names.contains(key))
                    .cloned()
                    .collect::<Vec<String>>(),
            );
            result.overrides.extend(other.overrides.clone());
        }

        result
    }
}

static OVERRIDE_PROPERTY_REGEX: &'static Lazy<Regex> = regex!(r"^(\[.*\])+$");

pub struct ConfigurationParser {
    registry: Arc<ConfigurationRegistry>,
}

struct ConfigurationOverride {
    ident: String,
    attribute_names: Vec<String>,
    content: Trie<String, serde_json::Value>,
}

impl ConfigurationParser {
    pub fn new(registry: Arc<ConfigurationRegistry>) -> Self {
        Self { registry }
    }

    pub fn parse(&self, content: &str) -> Result<ConfigurationModel> {
        let raw_content: HashMap<String, Value> = serde_json::from_str(content)?;
        let mut model = ConfigurationModel::empty();

        for (attribute_name, value) in &raw_content {
            if OVERRIDE_PROPERTY_REGEX.is_match(attribute_name) {
                if let Some(override_definition) = self.process_override(attribute_name, value) {
                    model.overrides.push(override_definition.ident);
                    model.content.extend(override_definition.content.iter());
                }

                continue;
            }

            if self.inspect_attribute(attribute_name) {
                model.set_value(AttributeName::format(attribute_name), value.clone());
            }
        }

        Ok(model)
    }

    // TODO: return diags
    // TODO: use logs, not println
    fn inspect_attribute(&self, attribute_name: &str) -> bool {
        let configuration_properties = self.registry.get_configuration_properties();

        match configuration_properties.get(attribute_name) {
            Some(registered_property) => {
                if registered_property.is_protected() {
                    println!(
                        "Property `{}` is protected from contribution",
                        attribute_name
                    );
                    return false;
                }

                true
            }
            None => {
                println!("Unknown property `{}` was detected", attribute_name);
                return false;
            }
        }
    }

    fn process_override(
        &self,
        attribute_name: &str,
        value: &Value,
    ) -> Option<ConfigurationOverride> {
        let content = if let Value::Object(obj) = value {
            obj
        } else {
            // If the override is not an object, then we don't want to handle it in any way.
            return None;
        };

        let override_identifiers = self.registry.get_override_identifiers();
        let formatted_identifier = attribute_name.trim_matches(|c| c == '[' || c == ']');

        if override_identifiers.get(formatted_identifier).is_none() {
            println!(
                "Unknown override identifier `{}` was detected",
                formatted_identifier
            );
            return None;
        }

        let mut result = ConfigurationOverride {
            ident: formatted_identifier.to_string(),
            attribute_names: Vec::new(),
            content: Trie::new(),
        };

        for (attribute_name, value) in content {
            if self.inspect_attribute(attribute_name) {
                // let formatted_key = format!("$.[{}].{}", formatted_identifier, attribute_name);
                let formatted_key =
                    AttributeName::format_with_override(attribute_name, formatted_identifier);
                result.content.insert(formatted_key.clone(), value.clone());
                result.attribute_names.push(formatted_key);
            }
        }

        Some(result)
    }
}

#[derive(Debug)]
pub struct InspectedConfigurationValue {
    attribute_name: String,
    value: Option<serde_json::Value>,
    seen_in_overrides: Vec<String>,
    default_configuration: Arc<ConfigurationModel>,
    policy_configuration: Arc<ConfigurationModel>,
}

impl InspectedConfigurationValue {
    pub fn attribute_name(&self) -> &str {
        &self.attribute_name
    }

    pub fn value(&self) -> &Option<serde_json::Value> {
        &self.value
    }
    // TODO: Rewrite using override keys and getting sections
    pub fn get_default_value(&self, attribute_name: &AttributeName) -> Option<&serde_json::Value> {
        self.default_configuration.get_value(attribute_name)
    }

    // TODO: Rewrite using override keys and getting sections
    pub fn get_policy_value(&self, attribute_name: &AttributeName) -> Option<&serde_json::Value> {
        self.policy_configuration.get_value(attribute_name)
    }
}

#[derive(Debug)]
pub struct Configuration {
    default_configuration: Arc<ConfigurationModel>,
    policy_configuration: Arc<ConfigurationModel>,
    user_configuration: ArcSwap<ConfigurationModel>,
    workspace_configuration: Arc<ConfigurationModel>,
    inmem_configuration: Arc<ConfigurationModel>,
    consolidated_configuration: ArcSwapOption<ConfigurationModel>,
}

// TODO: add overrides
pub struct ConfigurationDifference {
    pub added: Vec<String>,
    pub modified: Vec<String>,
    pub removed: Vec<String>,
}

impl Configuration {
    pub fn new(
        default_model: Arc<ConfigurationModel>,
        policy_model: Arc<ConfigurationModel>,
        user_model: Arc<ConfigurationModel>,
        workspace_model: Arc<ConfigurationModel>,
        inmem_model: Arc<ConfigurationModel>,
    ) -> Self {
        Configuration {
            default_configuration: default_model,
            policy_configuration: policy_model,
            user_configuration: ArcSwap::new(user_model),
            workspace_configuration: workspace_model,
            inmem_configuration: inmem_model,
            consolidated_configuration: ArcSwapOption::from(None),
        }
    }

    // TODO: implement `section` functionality

    pub fn get_value(&self, attribute_name: &AttributeName) -> Option<Value> {
        let consolidated_model = self.get_consolidated_configuration();

        consolidated_model.get_value(attribute_name).cloned()
    }

    pub fn inspect(&self, attribute_name: &AttributeName) -> InspectedConfigurationValue {
        let consolidated_model = self.get_consolidated_configuration();

        let value = consolidated_model.get_value(attribute_name).cloned();

        let mut inspected_value = InspectedConfigurationValue {
            attribute_name: attribute_name.to_string(),
            value,
            seen_in_overrides: Vec::new(),
            default_configuration: Arc::clone(&self.default_configuration),
            policy_configuration: Arc::clone(&self.policy_configuration),
        };

        if !attribute_name.is_with_override() {
            for ident in consolidated_model.overrides.iter() {
                if consolidated_model.overrides.contains(&format!(
                    "$.[{}].{}",
                    ident,
                    attribute_name.must_get_name()
                )) {
                    inspected_value.seen_in_overrides.push(ident.to_string());
                }
            }
        }

        inspected_value
    }

    pub fn update_user_configuration(
        &self,
        new_model: Arc<ConfigurationModel>,
    ) -> ConfigurationDifference {
        let diff = Self::compare(self.user_configuration.load_full(), Arc::clone(&new_model));
        self.user_configuration.swap(new_model);
        self.consolidated_configuration.swap(None);

        diff
    }

    fn compare(
        old: Arc<ConfigurationModel>,
        new: Arc<ConfigurationModel>,
    ) -> ConfigurationDifference {
        let old_names: HashSet<_> = old.names.iter().cloned().collect();
        let new_names: HashSet<_> = new.names.iter().cloned().collect();

        ConfigurationDifference {
            added: new_names.difference(&old_names).cloned().collect(),
            removed: old_names.difference(&new_names).cloned().collect(),
            modified: old_names
                .intersection(&new_names)
                .filter(
                    |name| match (old.content.get(*name), new.content.get(*name)) {
                        (Some(old_value), Some(new_value)) => old_value != new_value,
                        _ => false,
                    },
                )
                .cloned()
                .collect(),
        }
    }

    pub fn get_consolidated_configuration(&self) -> Arc<ConfigurationModel> {
        if let Some(config) = self.consolidated_configuration.load_full().as_ref() {
            return Arc::clone(config);
        }

        let consolidated_model = self
            .default_configuration
            .merge(&[
                Arc::clone(&self.user_configuration.load_full()),
                Arc::clone(&self.workspace_configuration),
                Arc::clone(&self.inmem_configuration),
            ])
            .into();

        self.consolidated_configuration
            .store(Some(Arc::clone(&consolidated_model)));

        consolidated_model
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_no_override_with_identifiers() {
        let name = attribute_name!(editor.fontSize);
        assert_eq!(name.override_ident, None);
        assert_eq!(name.name, Some("editor.fontSize".to_string()));
    }

    #[test]
    fn test_no_override_with_section_only() {
        let name = attribute_name!(editor);
        assert_eq!(name.override_ident, None);
        assert_eq!(name.name, Some("editor".to_string()));
    }

    #[test]
    fn test_single_override_with_identifiers() {
        let name = attribute_name!([rust].editor.fontSize);
        assert_eq!(name.override_ident, Some("rust".to_string()));
        assert_eq!(name.name, Some("editor.fontSize".to_string()));
    }

    #[test]
    fn test_single_override_only() {
        let name = attribute_name!([rust]);
        assert_eq!(name.override_ident, Some("rust".to_string()));
        assert_eq!(name.name, None);
    }

    #[test]
    fn test_no_override_with_multiple_identifiers() {
        let name = attribute_name!(config.window.size);
        assert_eq!(name.override_ident, None);
        assert_eq!(name.name, Some("config.window.size".to_string()));
    }

    #[test]
    fn test_single_override_with_multiple_identifiers() {
        let name = attribute_name!([javascript].config.window.size);
        assert_eq!(name.override_ident, Some("javascript".to_string()));
        assert_eq!(name.name, Some("config.window.size".to_string()));
    }

    #[test]
    fn test_single_override_only_with_no_identifiers() {
        let name = attribute_name!([typescript]);
        assert_eq!(name.override_ident, Some("typescript".to_string()));
        assert_eq!(name.name, None);
    }
}
