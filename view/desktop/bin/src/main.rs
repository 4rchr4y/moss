// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use app::{context_compact::AppContextCompact, AppCompact};
use app_lib::DesktopMain;
use platform_window_tgui::window::NativeWindowConfiguration;
use tracing::error;

fn main() {
    AppCompact::new().run(|ctx: &mut AppContextCompact| {
        let configuration = NativeWindowConfiguration { full_screen: false };

        if let Err(err) = DesktopMain::new(configuration).open(ctx) {
            error!("{err:#?}")
        }
    })
}
