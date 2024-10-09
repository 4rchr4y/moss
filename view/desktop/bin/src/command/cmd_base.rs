use tauri::State;
use workbench_tao::window::NativePlatformInfo;

use crate::AppState;

#[tauri::command]
pub fn native_platform_info(state: State<'_, AppState>) -> NativePlatformInfo {
    state.platform_info.clone()
}

// #[tauri::command]
// #[specta::specta]
// pub fn get_workbench_activites(state: State<'_, AppState>) ->
