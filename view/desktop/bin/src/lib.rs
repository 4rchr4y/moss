mod command;
mod config;
mod mem;
mod menu;
mod service;

use anyhow::{Context as _, Result};
use command::cmd_sidebar_part;
use platform_core::context_v2::async_context::AsyncContext;
use platform_core::context_v2::ContextCell;
use platform_core::platform::cross::client::CrossPlatformClient;
use platform_core::platform::AnyPlatform;
use platform_formation::service_registry::ServiceRegistry;
use platform_fs::disk::file_system_service::DiskFileSystemService;
use platform_workspace::WorkspaceId;
use service::project_service::ProjectService;
use service::session_service::SessionService;
use std::env;
use std::rc::Rc;
use std::sync::Arc;
use surrealdb::engine::remote::ws::Client;
use surrealdb::{engine::remote::ws::Ws, Surreal};
use tauri::{App, Manager};
use tauri_specta::{collect_commands, collect_events, Builder};
use workbench_service_environment_tao::environment_service::NativeEnvironmentService;
use workbench_tao::window::{NativePlatformInfo, NativeWindowConfiguration};
use workbench_tao::Workbench;

use crate::command::{cmd_base, cmd_dummy};

#[macro_use]
extern crate serde;

#[macro_use]
extern crate tracing;

pub struct MockStorageService {}

struct SimpleWindowState {
    workspace_id: WorkspaceId,
}

impl MockStorageService {
    fn new() -> Self {
        Self {}
    }

    fn get_last_window_state(&self) -> SimpleWindowState {
        SimpleWindowState {
            workspace_id: WorkspaceId::Some("workspace_path_hash".to_string()),
        }
    }
}

pub struct AppState {
    pub workbench: Arc<Workbench>,
    pub platform_info: NativePlatformInfo,
    pub project_service: ProjectService,
    pub session_service: SessionService,
}

pub fn run(native_window_configuration: NativeWindowConfiguration) -> Result<()> {
    let platform_client = Rc::new(CrossPlatformClient::new());
    platform_client.run(async {
        let ctx_cell = ContextCell::new(platform_client.clone());
        let async_ctx = ctx_cell.borrow().to_async();
        let tao_app = initialize_app(
            async_ctx,
            platform_client.clone(),
            native_window_configuration,
        )
        .expect("Failed to build tauri app");

        Ok(tao_app.run(|_, _| {}))
    })
}

fn initialize_app(
    ctx: AsyncContext,
    platform_client: Rc<CrossPlatformClient>,
    native_window_configuration: NativeWindowConfiguration,
) -> Result<App> {
    // let builder = create_specta_builder();

    // #[cfg(debug_assertions)]
    // export_typescript_bindings(&builder)?;

    //  TODO: move to StorageService
    let db = Arc::new(
        platform_client
            .background_executor()
            .block_on(init_db_client())?,
    );

    let platform_info_clone = native_window_configuration.platform_info.clone();
    let service_group = create_service_registry(native_window_configuration)?;
    let tao_app = tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            cmd_dummy::workbench_get_state,
            cmd_dummy::create_project,
            cmd_dummy::restore_session,
            cmd_dummy::app_ready,
            cmd_dummy::update_font_size,
            cmd_dummy::fetch_all_themes,
            cmd_dummy::read_theme,
            cmd_base::native_platform_info,
            cmd_sidebar_part::sidebar_get_all_activities
        ])
        .setup(move |app: &mut App| setup_app(app, ctx, service_group, db, platform_info_clone))
        .menu(menu::setup_window_menu)
        .plugin(tauri_plugin_os::init())
        .build(tauri::generate_context!())?;

    Ok(tao_app)
}

fn setup_app(
    app: &mut App,
    mut ctx: AsyncContext,
    service_group: ServiceRegistry,
    db: Arc<Surreal<Client>>,
    platform_info: NativePlatformInfo,
) -> std::result::Result<(), Box<dyn std::error::Error>> {
    let window_state = service_group
        .get_unchecked::<MockStorageService>()
        .get_last_window_state();

    let mut workbench = Workbench::new(&mut ctx, service_group, window_state.workspace_id)?;
    workbench.initialize(&mut ctx)?;

    // workbench.add_activity(
    //     "activityBar.launchpad",
    //     Activity {
    //         title: "Launchpad".to_string(),
    //         icon: "icon_name.svg".to_string(),
    //         order: Some(1),
    //         spot: ActivitySpot::Left,
    //     },
    // );
    // workbench.add_activity(
    //     "activityBar.essentials",
    //     Activity {
    //         title: "Essentials".to_string(),
    //         icon: "icon_name.svg".to_string(),
    //         order: Some(2),
    //         spot: ActivitySpot::Left,
    //     },
    // );

    let window = app.get_webview_window("main").unwrap();
    let app_state = AppState {
        workbench: Arc::new(workbench),
        platform_info,
        project_service: ProjectService::new(db.clone()),
        session_service: SessionService::new(db.clone()),
    };

    ctx.apply(|tx_ctx| {
        app_state
            .workbench
            .set_configuration_window_size(window)
            .unwrap();

        app_state
            .workbench
            .set_tao_handle(tx_ctx, app.handle().clone());
    })?;

    {
        app.handle().manage(ctx);
        app.handle().manage(app_state);
    }

    Ok(())
}

// fn create_specta_builder() -> Builder {
//     tauri_specta::Builder::<tauri::Wry>::new()
//         .events(collect_events![])
//         .commands(collect_commands![
//             cmd_dummy::workbench_get_state,
//             cmd_dummy::create_project,
//             cmd_dummy::restore_session,
//             cmd_dummy::app_ready,
//             cmd_dummy::update_font_size,
//             cmd_dummy::fetch_all_themes,
//             cmd_dummy::read_theme,
//             cmd_base::native_platform_info,
//         ])
// }

fn create_service_registry(
    native_window_configuration: NativeWindowConfiguration,
) -> Result<ServiceRegistry> {
    let mut service_registry = ServiceRegistry::new();

    let mock_storage_service = MockStorageService::new();

    let fs_service = DiskFileSystemService::new();
    let environment_service =
        NativeEnvironmentService::new(native_window_configuration.home_dir.clone());

    service_registry.insert(mock_storage_service);
    service_registry.insert(environment_service);
    service_registry.insert(Arc::new(fs_service));

    Ok(service_registry)
}

fn export_typescript_bindings(builder: &tauri_specta::Builder) -> Result<()> {
    Ok(builder
        .export(
            specta_typescript::Typescript::default()
                .formatter(specta_typescript::formatter::prettier)
                .header("/* eslint-disable */"),
            "../src/bindings.ts",
        )
        .context("Failed to export typescript bindings")?)
}

async fn init_db_client() -> Result<Surreal<Client>> {
    // let db = Surreal::new::<File>("../rocksdb").await.unwrap();

    let db = Surreal::new::<Ws>("127.0.0.1:8000")
        .await
        .expect("failed to connect to db");
    db.use_ns("moss").use_db("compass").await?;

    Ok(db)
}

// An example of how the logging could function
// fn init_custom_logging(app_handle: tauri::AppHandle) {
//     struct TauriLogWriter {
//         app_handle: tauri::AppHandle,
//     }

//     impl std::io::Write for TauriLogWriter {
//         fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
//             let log_message = String::from_utf8_lossy(buf).to_string();
//             let _ = self.app_handle.emit("logs-stream", log_message);
//             Ok(buf.len())
//         }

//         fn flush(&mut self) -> std::io::Result<()> {
//             Ok(())
//         }
//     }

//     tracing_subscriber::registry()
//         // log to stdout
//         .with(tracing_subscriber::fmt::layer().with_writer(std::io::stdout))
//         // log to frontend
//         .with(
//             tracing_subscriber::fmt::layer().with_writer(move || TauriLogWriter {
//                 app_handle: app_handle.clone(),
//             }),
//         )
//         .init();

//     event!(tracing::Level::DEBUG, "Logging init");
//     info!("Logging initialized");
// }
