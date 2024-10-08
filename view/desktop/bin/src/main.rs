// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use app_lib::run;
use std::process::ExitCode;
use workbench_tao::window::{NativePlatformInfo, NativeWindowConfiguration};

fn main() -> ExitCode {
    let home_dir = std::env::var("HOME")
        .expect("Failed to retrieve the $HOME environment variable")
        .into();

    if let Err(e) = run(NativeWindowConfiguration {
        home_dir,
        full_screen: false,
        platform_info: NativePlatformInfo::new(),
    }) {
        tracing::error!("{}", e);
        ExitCode::FAILURE
    } else {
        ExitCode::SUCCESS
    }
}
