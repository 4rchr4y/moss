use hashbrown::{HashMap, HashSet};
use lazy_regex::{Lazy, Regex};
use serde_json::Value;

pub static OVERRIDE_PROPERTY_REGEX: &'static Lazy<Regex> = regex!(r"^(\[.*\])+$");
static OVERRIDE_IDENTIFIER_REGEX: &'static Lazy<Regex> = regex!(r"\[([^\]]+)\]");

/// Enumeration representing the scope of a configuration setting.
/// This enum defines the different levels at which a configuration setting can be applied.
#[derive(Debug, Clone)]
pub enum ConfigurationScope {
    /// Platform-specific configuration that applies globally across the entire platform.
    Platform,
    /// Machine-specific configuration that applies to the entire machine.
    Machine,
    /// Window-specific configuration that applies to a single window within the application.
    Window,
    /// Resource-specific configuration that applies to individual resources, such as files or projects.
    Resource,
}

impl Default for ConfigurationScope {
    fn default() -> Self {
        Self::Window
    }
}

/// Enumeration representing the type of a configuration setting.
#[derive(Debug, Clone, PartialEq)]
pub enum ConfigurationNodeType {
    Null,
    String,
    Bool,
    Number,
    Array,
    Object,
}

impl Default for ConfigurationNodeType {
    fn default() -> Self {
        Self::Null
    }
}

impl ConfigurationNodeType {
    fn is_object(&self) -> bool {
        if self == &ConfigurationNodeType::Object {
            return true;
        } else {
            return false;
        }
    }

    fn default_value(r#type: &Self) -> serde_json::Value {
        match r#type {
            ConfigurationNodeType::Null => Value::Null,
            ConfigurationNodeType::String => Value::String(String::new()),
            ConfigurationNodeType::Bool => Value::Bool(false),
            ConfigurationNodeType::Number => Value::Number(serde_json::Number::from(0)),
            ConfigurationNodeType::Array => Value::Array(vec![]),
            ConfigurationNodeType::Object => Value::Object(serde_json::Map::new()),
        }
    }
}

#[derive(Debug, Clone)]
pub struct SourceInfo {
    pub id: String,
    pub display_name: Option<String>,
}

/// Struct representing a schema for a configuration property.
/// This struct defines the metadata and constraints for a configuration setting.
#[derive(Debug, Clone)]
pub struct ConfigurationPropertySchema {
    /// The scope of the configuration property, indicating the level at which it applies.
    pub scope: Option<ConfigurationScope>,
    /// The type of the configuration property, specifying the kind of value it holds.
    pub r#type: ConfigurationNodeType,
    /// The order in which the configuration property appears in the settings UI.
    pub order: Option<usize>,
    /// The default value of the configuration property, if any.
    pub default: Option<Value>,
    /// A description of the configuration property, providing context and usage information.
    pub description: Option<String>,
    /// Indicates if the configuration property is protected from contributions by extensions.
    /// If true, the property cannot be overridden by contributions.
    pub protected_from_contribution: Option<bool>,
    /// Specifies if the configuration property is allowed only for restricted sources.
    /// If true, the property can only be set by trusted sources.
    pub allow_for_only_restricted_source: Option<bool>,
    /// Indicates if the configuration property is included in the registry.
    /// If false, the property is excluded from the configuration registry.
    pub schemable: Option<bool>,

    pub source: Option<SourceInfo>,
}

/// Struct representing a configuration node.
#[derive(Debug, Clone)]
pub struct ConfigurationNode {
    /// The ID of the configuration node.
    pub id: Option<String>,
    /// The scope of the configuration property, indicating the level at which it applies.
    pub scope: Option<ConfigurationScope>,
    /// The order in which the configuration node appears.
    pub order: Option<usize>,
    /// The type of the configuration node.
    pub r#type: Option<ConfigurationNodeType>,
    /// The title of the configuration node.
    pub title: Option<String>,
    /// The description of the configuration node.
    pub description: Option<String>,
    /// The properties of the configuration node.
    pub properties: Option<HashMap<String, ConfigurationPropertySchema>>,
    /// Sub-nodes of the configuration node.
    pub parent_of: Option<Vec<ConfigurationNode>>,

    pub source: Option<SourceInfo>,
}

/// Struct representing default configurations.
#[derive(Debug, Clone)]
pub struct ConfigurationDefaults {
    /// The default values for configuration properties.
    /// This field contains a map where the keys are configuration property names and the values are their default values.
    /// These defaults can override the initial values specified in the configuration schema.
    pub overrides: HashMap<String, Value>,
    /// The source of the default configurations.
    /// This optional field indicates the origin of these default configurations, such as an extension or a specific configuration context.
    /// It provides context for the default values and helps track their origin.
    pub source: Option<SourceInfo>,
}

#[derive(Debug, Clone)]
pub struct RegisteredConfigurationPropertySchema {
    pub schema: ConfigurationPropertySchema,
    pub source: Option<SourceInfo>,
}

impl RegisteredConfigurationPropertySchema {
    pub fn is_protected(&self) -> bool {
        self.schema.protected_from_contribution.unwrap_or(false)
    }
}

impl RegisteredConfigurationPropertySchema {
    fn new(property: &ConfigurationPropertySchema, source: Option<SourceInfo>) -> Self {
        let registered_property = Self {
            schema: property.clone(),
            source,
        };

        registered_property
    }
}

/// Struct representing an override value for a default configuration.
#[derive(Debug, Clone)]
pub struct ConfigurationDefaultOverrideValue {
    /// The value of the override.
    /// This field stores the new default value for the configuration property.
    /// It replaces the original default value defined in the configuration schema.
    pub value: Value,
    /// The source of the override.
    /// This optional field indicates the origin of the override, such as an extension or user-defined configuration.
    /// It helps track where the override came from and provides context for the overridden value.
    pub source: Option<SourceInfo>,
}

/// Struct to store schema information for configuration settings.
#[derive(Debug)]
pub struct ConfigurationSchemaStorage {
    /// Schema for all settings.
    all_settings_schema: HashMap<String, ConfigurationPropertySchema>,
    /// Schema for platform-specific settings.
    platform_settings_schema: HashMap<String, ConfigurationPropertySchema>,
    /// Schema for machine-specific settings.
    machine_settings_schema: HashMap<String, ConfigurationPropertySchema>,
    /// Schema for window-specific settings.
    window_settings_schema: HashMap<String, ConfigurationPropertySchema>,
    /// Schema for resource-specific settings.
    resource_settings_schema: HashMap<String, ConfigurationPropertySchema>,
}

impl ConfigurationSchemaStorage {
    fn empty() -> Self {
        Self {
            all_settings_schema: HashMap::new(),
            platform_settings_schema: HashMap::new(),
            machine_settings_schema: HashMap::new(),
            window_settings_schema: HashMap::new(),
            resource_settings_schema: HashMap::new(),
        }
    }

    fn update_schema(&mut self, key: &str, property: &ConfigurationPropertySchema) {
        self.all_settings_schema
            .insert(key.to_string(), property.clone());

        match property
            .scope
            .as_ref()
            .unwrap_or(&ConfigurationScope::Window)
        {
            ConfigurationScope::Platform => {
                self.platform_settings_schema
                    .insert(key.to_string(), property.clone());
            }
            ConfigurationScope::Machine => {
                self.machine_settings_schema
                    .insert(key.to_string(), property.clone());
            }
            ConfigurationScope::Window => {
                self.window_settings_schema
                    .insert(key.to_string(), property.clone());
            }
            ConfigurationScope::Resource => {
                self.resource_settings_schema
                    .insert(key.to_string(), property.clone());
            }
        }
    }
}

/// Registry to manage configurations and their schemas.
#[derive(Debug)]
pub struct ConfigurationRegistry {
    #[allow(unused)] // Designed for future expansion
    /// List of registered default configurations.
    /// These configurations define standard default values for various settings that can be
    /// overridden by users or other configurations.
    registered_configuration_defaults: Vec<ConfigurationDefaults>,

    #[allow(unused)] // Designed for future expansion
    /// Map of configuration default overrides.
    /// This hashmap stores overridden default values for configuration properties, indexed by their keys.
    /// Overrides can come from different sources and can change the default values defined in `registered_configuration_defaults`.
    configuration_defaults_overrides: HashMap<String, ConfigurationDefaultOverrideValue>,

    /// Map of configuration properties.
    /// This hashmap stores the properties of configurations, indexed by their keys.
    /// Each property includes metadata such as type, scope, default values, and descriptions.
    configuration_properties: HashMap<String, RegisteredConfigurationPropertySchema>,

    /// List of configuration nodes contributed.
    /// This vector contains all configuration nodes that have been registered to the registry.
    /// Configuration nodes can include multiple properties and sub-nodes.
    configuration_contributors: Vec<ConfigurationNode>,

    /// Set of override identifiers.
    /// This set contains identifiers that are used to specify configurations that can override default values.
    /// Override identifiers are used to create specialized settings for different scopes or contexts.
    override_identifiers: HashSet<String>,

    /// Storage for configuration schemas.
    /// This structure stores the schema definitions for all settings, organized by their scope (e.g., platform, machine, window, resource).
    /// It is used to generate and manage the JSON schema for configuration properties.
    configuration_schema_storage: ConfigurationSchemaStorage,

    /// Map of excluded configuration properties.
    /// This hashmap stores properties that are explicitly excluded from the configuration registry.
    /// These properties are not included in the configuration schema and are not available for users to configure.
    excluded_configuration_properties: HashMap<String, RegisteredConfigurationPropertySchema>,
}

impl ConfigurationRegistry {
    pub fn new() -> Self {
        Self {
            registered_configuration_defaults: Vec::new(),
            configuration_properties: HashMap::new(),
            configuration_contributors: Vec::new(),
            configuration_defaults_overrides: HashMap::new(),
            override_identifiers: HashSet::new(),
            configuration_schema_storage: ConfigurationSchemaStorage::empty(),
            excluded_configuration_properties: HashMap::new(),
        }
    }

    pub fn get_configuration_properties(
        &self,
    ) -> &HashMap<String, RegisteredConfigurationPropertySchema> {
        &self.configuration_properties
    }

    pub fn get_excluded_configuration_properties(
        &self,
    ) -> &HashMap<String, RegisteredConfigurationPropertySchema> {
        &self.excluded_configuration_properties
    }

    pub fn get_override_identifiers(&self) -> &HashSet<String> {
        &self.override_identifiers
    }

    pub fn register_configuration(&mut self, configuration: ConfigurationNode) {
        let _properties = self.do_configuration_registration(configuration, false);

        // TODO: Emit schema change events
    }

    fn do_configuration_registration(
        &mut self,
        configuration: ConfigurationNode,
        validate: bool,
    ) -> HashMap<String, ConfigurationPropertySchema> {
        let configuration_scope_or_default = configuration.scope.unwrap_or_default();
        let mut configuration_properties = configuration.properties.unwrap_or_default();

        for (key, property) in configuration_properties.clone() {
            if validate && !self.validate_property(&property) {
                continue;
            }

            let mut registered_property =
                RegisteredConfigurationPropertySchema::new(&property, configuration.source.clone());

            if OVERRIDE_PROPERTY_REGEX.is_match(&key) {
                // Assigning a specific scope is redundant since this property already implies a particular context.
                registered_property.schema.scope = None;

                self.handle_override_identifier(&key, &property);
                continue;
            } else {
                registered_property.schema.scope = Some(configuration_scope_or_default.clone());
                registered_property.schema.allow_for_only_restricted_source =
                    Some(property.allow_for_only_restricted_source.unwrap_or(false));
            }

            if property.schemable.unwrap_or(true) {
                self.configuration_properties
                    .insert(key.clone(), registered_property);
            } else {
                self.excluded_configuration_properties
                    .insert(key.clone(), registered_property);
            }
        }

        if let Some(sub_nodes) = configuration.parent_of {
            for node in sub_nodes {
                let sub_properties = self.do_configuration_registration(node, false);
                configuration_properties.extend(sub_properties);
            }
        }

        configuration_properties
    }

    fn validate_property(&self, _property: &ConfigurationPropertySchema) -> bool {
        unimplemented!()
    }

    fn handle_override_identifier(&mut self, key: &str, property: &ConfigurationPropertySchema) {
        let trimmed_override_identifier = key.trim_matches(|c| c == '[' || c == ']').to_string();
        self.override_identifiers
            .insert(trimmed_override_identifier);

        if property.r#type.is_object() {
            // TODO:
            // 1. Validation
        }
    }

    // fn register_json_configuration(&mut self, configuration: &ConfigurationNode) {
    //     let properties = configuration.properties.unwrap_or_default();

    //     for (key, property) in &properties {
    //         // Check if the property is included in the configuration registry
    //         if property.included.unwrap_or(true) {
    //             self.configuration_schema_storage
    //                 .update_schema(key, property);
    //         }
    //     }

    //     for sub_node in configuration.all_of.as_ref().unwrap_or(&vec![]) {
    //         self.register_json_configuration(sub_node);
    //     }
    // }

    pub fn register_default_configurations(
        &mut self,
        default_configurations: Vec<ConfigurationDefaults>,
    ) {
        let _properties = self.do_register_default_configuration(default_configurations);

        // TODO: Emit schema change events
        unimplemented!()
    }

    fn do_register_default_configuration(
        &mut self,
        _default_configurations: Vec<ConfigurationDefaults>,
    ) -> HashSet<String> {
        unimplemented!()
    }
}
