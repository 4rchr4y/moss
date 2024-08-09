mod mem;

pub mod menu;
pub mod service;

use anyhow::Result;
use app::context_compact::AppContextCompact;
use configuration::common::{
    configuration_registry::{
        ConfigurationNode, ConfigurationPropertySchema, ConfigurationRegistry,
    },
    configuration_service::ConfigurationService,
    AbstractConfigurationService,
};
use configuration::{
    attribute_name,
    common::{
        configuration_policy::ConfigurationPolicyService,
        configuration_registry::{
            ConfigurationNodeType, ConfigurationScope, ConfigurationSource, PropertyMap,
            PropertyPolicy,
        },
    },
    property_key,
};
use hashbrown::HashMap;
use platform_formation_common::service_group::ServiceGroup;
use platform_window_tgui::window::NativeWindowConfiguration;
use service::project_service::ProjectService;
use service::session_service::SessionService;
use std::path::PathBuf;
use std::sync::Arc;
use surrealdb::{engine::remote::ws::Ws, Surreal};
use surrealdb::{
    method::Query,
    opt::{IntoEndpoint, IntoQuery},
    Connection,
};
use tauri::{App, AppHandle, Emitter, Manager, State};
use tauri_specta::{collect_commands, collect_events};
use workbench_tgui::Workbench;

use crate::service::{
    project_service::{CreateProjectInput, ProjectDTO},
    session_service::SessionInfoDTO,
};

#[macro_use]
extern crate serde;

#[macro_use]
extern crate tracing;

#[tauri::command(async)]
#[specta::specta]
async fn app_ready(app_handle: AppHandle) {
    let window = app_handle.get_webview_window("main").unwrap();
    window.show().unwrap();
}

#[tauri::command(async)]
#[specta::specta]
async fn create_project(
    state: State<'_, AppState>,
    input: CreateProjectInput,
) -> Result<Option<ProjectDTO>, String> {
    match state.project_service.create_project(&input).await {
        Ok(Some(project)) => return Ok(Some(project.into())),
        Ok(None) => return Ok(None),
        Err(e) => {
            let err = format!("An error occurred while creating the project: {e}");
            error!(err);
            return Err(err);
        }
    }
}

#[tauri::command(async)]
#[specta::specta]
async fn restore_session(
    state: State<'_, AppState>,
    project_source: Option<String>,
) -> Result<Option<SessionInfoDTO>, String> {
    match state.session_service.restore_session(project_source).await {
        Ok(Some(session_info)) => return Ok(Some(session_info.into())),
        Ok(None) => return Ok(None),
        Err(e) => {
            let err = format!("An error occurred while restoring the session: {e}");
            error!(err);
            return Err(err);
        }
    }
}

pub struct DesktopMain {
    native_window_configuration: NativeWindowConfiguration,
}

impl DesktopMain {
    pub fn new(configuration: NativeWindowConfiguration) -> Self {
        Self {
            native_window_configuration: configuration,
        }
    }

    pub fn open(&self, ctx: &mut AppContextCompact) -> Result<()> {
        // ------ Example stream
        let (tx, mut rx) = tokio::sync::broadcast::channel(16);

        ctx.detach(|_| async move {
            let mut interval = tokio::time::interval(tokio::time::Duration::from_secs(1));
            let mut count = 0;
            loop {
                interval.tick().await;
                count += 1;
                if tx.send(count).is_err() {
                    break;
                }
            }
        });
        // ------

        // TODO: move to StorageService
        let db = ctx.block_on(|ctx| async {
            let db = Surreal::new::<Ws>("127.0.0.1:8000")
                .await
                .expect("failed to connect to db");
            // let db = Surreal::new::<File>("../rocksdb").await.unwrap();
            db.use_ns("moss").use_db("compass").await.unwrap();

            // let schema = include_str!("../schema.surql");
            // db.query(schema).await.unwrap();

            Arc::new(db)
        });

        let mut registry = ConfigurationRegistry::new();

        let editor_configuration = ConfigurationNode {
            id: "editor".to_string(),
            title: Some("Editor".to_string()),
            order: Some(1),
            r#type: Default::default(),
            scope: Default::default(),
            source: Some(ConfigurationSource {
                id: "moss.core".to_string(),
                display_name: Some("Moss Core".to_string()),
            }),
            properties: {
                let mut properties = PropertyMap::new();
                properties.insert(
                    property_key!(editor.fontSize),
                    ConfigurationPropertySchema {
                        scope: Some(ConfigurationScope::Resource),
                        typ: Some(ConfigurationNodeType::Number),
                        order: Some(1),
                        default: Some(serde_json::Value::Number(serde_json::Number::from(12))),
                        description: Some("Controls the font size in pixels.".to_string()),
                        ..Default::default()
                    },
                );
                properties.insert(
                    property_key!(editor.lineHeight),
                    ConfigurationPropertySchema {
                        scope: Some(ConfigurationScope::Resource),
                        typ: Some(ConfigurationNodeType::Number),
                        order: Some(2),
                        default: Some(serde_json::Value::Number(serde_json::Number::from(20))),
                        description: Some("Controls the line height.".to_string()),
                        policy: Some(PropertyPolicy {
                            name: "editorLineHeightPolicy".to_string(),
                        }),
                        ..Default::default()
                    },
                );

                Some(properties)
            },
            description: None,
            parent_of: Some(vec![ConfigurationNode {
                id: "mossql".to_string(),
                title: Some("MossQL".to_string()),
                order: Some(1),
                r#type: Default::default(),
                scope: Default::default(),
                source: Some(ConfigurationSource {
                    id: "moss.core".to_string(),
                    display_name: Some("Moss Core".to_string()),
                }),

                properties: {
                    let mut properties = PropertyMap::new();

                    properties.insert(
                        property_key!([mossql].editor.fontSize),
                        ConfigurationPropertySchema {
                            scope: Some(ConfigurationScope::Resource),
                            typ: Some(ConfigurationNodeType::Number),
                            order: Some(1),
                            default: Some(serde_json::Value::Number(serde_json::Number::from(12))),
                            description: Some("Controls the font size in pixels.".to_string()),
                            protected_from_contribution: Some(false),
                            allow_for_only_restricted_source: Some(false),
                            schemable: Some(true),
                            ..Default::default()
                        },
                    );
                    properties.insert(
                        property_key!([mossql].editor.lineHeight),
                        ConfigurationPropertySchema {
                            scope: Some(ConfigurationScope::Resource),
                            typ: Some(ConfigurationNodeType::Number),
                            order: Some(2),
                            default: Some(serde_json::Value::Number(serde_json::Number::from(30))),
                            description: Some("Controls the line height.".to_string()),
                            ..Default::default()
                        },
                    );

                    Some(properties)
                },
                description: None,
                parent_of: None,
            }]),
        };

        registry.register_configuration(editor_configuration);

        let policy_service = ConfigurationPolicyService {
            definitions: {
                use configuration::common::policy::PolicyDefinitionType;

                let mut this = HashMap::new();

                this.insert(
                    "editorLineHeightPolicy".to_string(),
                    PolicyDefinitionType::Number,
                );

                this
            },
            policies: {
                let mut this = HashMap::new();
                this.insert(
                    "editorLineHeightPolicy".to_string(),
                    serde_json::Value::Number(serde_json::Number::from(45)),
                );

                this
            },
        };

        // ctx.block_on(|_| async {
        //     let config_service = ConfigurationService::new(
        //         Arc::new(registry),
        //         policy_service,
        //         &PathBuf::from("../../../.moss/settings.json"),
        //     )
        //     .unwrap();

        //     let value = config_service.get_value(attribute_name!(editor.fontSize));
        //     println!("Value `editor.fontSize` form None: {:?}", value);

        //     let value = config_service.get_value(attribute_name!(editor.lineHeight));
        //     println!("Value `editor.lineHeight` form None: {:?}", value);

        //     let value = config_service.get_value(attribute_name!([mossql].editor.fontSize));
        //     println!("Value `editor.fontSize` form `mossql`: {:?}", value);

        //     config_service
        //         .update_value(
        //             "editor.fontSize",
        //             serde_json::Value::Number(serde_json::Number::from(15)),
        //         )
        //         .await
        //         .unwrap();

        //     let value = config_service.get_value("editor.fontSize", None);
        //     println!("Value `editor.fontSize` form None (after): {:?}", value);
        // });

        let service_group = ServiceGroup::new();

        let config_service = ConfigurationService::new(
            Arc::new(registry),
            policy_service,
            &PathBuf::from("../../../.moss/settings.json"),
        )
        .unwrap();

        service_group.set(config_service);

        let builder = tauri_specta::Builder::<tauri::Wry>::new()
            .events(collect_events![])
            .commands(collect_commands![
                create_project,
                restore_session,
                app_ready
            ]);

        #[cfg(debug_assertions)] // <- Only export on non-release builds
        builder
            .export(
                specta_typescript::Typescript::default(),
                "../src/bindings.ts",
            )
            .expect("Failed to export typescript bindings");

        tauri::Builder::default()
            .manage(AppState {
                workbench: Workbench::new(service_group)?,
                project_service: ProjectService::new(db.clone()),
                session_service: SessionService::new(db.clone()),
            })
            // .invoke_handler(invoke_handler)
            .setup(move |app: &mut App| {
                let app_handle = app.handle().clone();

                tokio::task::block_in_place(|| {
                    tauri::async_runtime::block_on(async move {
                        // Example stream data emitting
                        tokio::spawn(async move {
                            while let Ok(data) = rx.recv().await {
                                app_handle.emit("data-stream", data).unwrap();
                            }
                        });
                    });
                });

                Ok(())
            })
            .menu(menu::setup_window_menu)
            .plugin(tauri_plugin_os::init())
            .build(tauri::generate_context!())?
            .run(|_, _| {});

        Ok(())
    }
}

pub struct SurrealClient<C: Connection>(Surreal<C>);

impl<C: Connection> SurrealClient<C> {
    pub async fn new<P>(
        address: impl IntoEndpoint<P, Client = C>,
        ns: impl Into<String>,
        db: impl Into<String>,
    ) -> Result<Self> {
        let client = Surreal::new::<P>(address).await?;
        client.use_ns(ns).use_db(db).await?;

        Ok(Self(client))
    }

    pub fn query(&self, query: impl IntoQuery) -> Query<C> {
        self.query(query)
    }
}

pub struct AppState {
    pub workbench: Workbench,
    pub project_service: ProjectService,
    pub session_service: SessionService,
}
