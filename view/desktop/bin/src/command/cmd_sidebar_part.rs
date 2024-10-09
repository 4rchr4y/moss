use tauri::State;
use workbench_tao::command::DescribeActivityOutput;

use crate::AppState;

#[tauri::command]
pub fn sidebar_get_all_activities(
    state: State<'_, AppState>,
) -> Result<Vec<DescribeActivityOutput>, String> {
    state
        .workbench
        .command_get_all_activities()
        .map_err(|err| format!("failed to get all activities: {err}"))
}
