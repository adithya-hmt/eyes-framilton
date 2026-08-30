#![cfg(target_os = "linux")]

use super::Snapshot;

/// The Linux adapter owns the AT-SPI2 connection. Keep it behind this small
/// seam so the capture loop, redaction, and Markdown format stay portable.
///
/// AT-SPI exposes a D-Bus tree, not a single synchronous "read this window"
/// call. The production adapter should connect once per capture thread, track
/// focus events, and walk only the focused application's subtree. Returning
/// `None` here is safer than falling back to screenshots or shell scraping.
pub fn snapshot() -> Option<Snapshot> {
    None
}
