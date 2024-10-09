use specta::Type;
use std::path::PathBuf;
use sysinfo::System;
use ts_rs::TS;

#[derive(Debug, Clone, Serialize, TS)]
pub struct NativePlatformInfo {
    pub os: String,
    pub version: String,
    pub hostname: String,
}

impl NativePlatformInfo {
    pub fn new() -> Self {
        let mut sys = System::new_all();
        sys.refresh_all();

        Self {
            os: System::name().unwrap_or_else(|| "unknown".to_string()),
            version: System::os_version().unwrap_or_else(|| "unknown".to_string()),
            hostname: System::host_name().unwrap_or_else(|| "unknown".to_string()),
        }
    }
}

#[derive(Debug)]
pub struct NativeWindowConfiguration {
    pub home_dir: PathBuf,
    pub full_screen: bool,
    pub platform_info: NativePlatformInfo,
}

#[cfg(test)]
mod tests {
    #[test]
    fn simple_test() {}
}
