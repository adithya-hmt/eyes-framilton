#![cfg(target_os = "windows")]

use super::Snapshot;

/// Windows' reader belongs on UI Automation's COM thread. It should call
/// `GetFocusedElement`, skip password controls, and walk the focused window's
/// text-bearing descendants. No process-wide screen scrape is acceptable.
pub fn snapshot() -> Option<Snapshot> {
    None
}
