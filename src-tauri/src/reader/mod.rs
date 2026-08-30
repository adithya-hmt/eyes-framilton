use serde::Serialize;

#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize)]
pub struct Snapshot {
    pub app: String,
    pub window_title: Option<String>,
    pub document: Option<String>,
    pub url: Option<String>,
    pub text: Vec<String>,
}

#[cfg(target_os = "linux")]
mod linux;
#[cfg(target_os = "windows")]
mod windows;

pub fn snapshot() -> Option<Snapshot> {
    #[cfg(target_os = "linux")]
    return linux::snapshot();

    #[cfg(target_os = "windows")]
    return windows::snapshot();

    #[cfg(not(any(target_os = "linux", target_os = "windows")))]
    None
}

pub fn platform_name() -> &'static str {
    #[cfg(target_os = "linux")]
    return "Linux / AT-SPI2";

    #[cfg(target_os = "windows")]
    return "Windows / UI Automation";

    #[cfg(not(any(target_os = "linux", target_os = "windows")))]
    "Unsupported desktop"
}
