mod privacy;
mod reader;
mod writer;

use chrono::Local;
use serde::Serialize;
use std::path::Path;
use std::sync::atomic::{AtomicBool, AtomicUsize, Ordering};
use std::sync::Arc;
use std::thread;
use std::time::Duration;
use tauri::menu::{MenuBuilder, MenuItemBuilder};
use tauri::tray::{MouseButton, MouseButtonState, TrayIconBuilder, TrayIconEvent};
use tauri::{AppHandle, Manager};

#[derive(Clone, Default)]
struct CaptureState {
    running: Arc<AtomicBool>,
    blocks_today: Arc<AtomicUsize>,
    last_snapshot: Arc<std::sync::Mutex<Option<reader::Snapshot>>>,
}

#[derive(Serialize)]
struct Status {
    running: bool,
    blocks_today: usize,
    platform: &'static str,
}

fn status(state: &CaptureState) -> Status {
    Status {
        running: state.running.load(Ordering::Relaxed),
        blocks_today: state.blocks_today.load(Ordering::Relaxed),
        platform: reader::platform_name(),
    }
}

#[tauri::command]
fn capture_status(state: tauri::State<'_, CaptureState>) -> Status {
    status(&state)
}

#[tauri::command]
fn toggle_capture(state: tauri::State<'_, CaptureState>) -> Status {
    let next = !state.running.load(Ordering::Relaxed);
    state.running.store(next, Ordering::Relaxed);
    status(&state)
}

#[tauri::command]
fn capture_once(
    app: tauri::AppHandle,
    state: tauri::State<'_, CaptureState>,
) -> Result<Status, String> {
    let folder = app
        .path()
        .app_data_dir()
        .map_err(|error| error.to_string())?
        .join("daily");
    capture_to_folder(&state, &folder)?;
    Ok(status(&state))
}

fn capture_to_folder(state: &CaptureState, folder: &Path) -> Result<(), String> {
    if !state.running.load(Ordering::Relaxed) {
        return Ok(());
    }
    let Some(snapshot) = reader::snapshot() else {
        return Ok(());
    };
    let Some(clean) = privacy::redact_snapshot(snapshot) else {
        return Ok(());
    };
    let mut last_snapshot = state
        .last_snapshot
        .lock()
        .map_err(|error| error.to_string())?;
    if same_snapshot(last_snapshot.as_ref(), &clean) {
        return Ok(());
    }
    writer::append_snapshot(folder, &clean, Local::now()).map_err(|error| error.to_string())?;
    state.blocks_today.fetch_add(1, Ordering::Relaxed);
    *last_snapshot = Some(clean);
    Ok(())
}

fn same_snapshot(last: Option<&reader::Snapshot>, current: &reader::Snapshot) -> bool {
    last == Some(current)
}

fn show_main(app: &AppHandle) {
    if let Some(window) = app.get_webview_window("main") {
        let _ = window.show();
        let _ = window.set_focus();
    }
}

pub fn run() {
    tauri::Builder::default()
        .manage(CaptureState::default())
        .invoke_handler(tauri::generate_handler![
            capture_status,
            toggle_capture,
            capture_once
        ])
        .setup(|app| {
            let state = app.state::<CaptureState>().inner().clone();
            let app_handle = app.handle().clone();
            thread::spawn(move || loop {
                if let Ok(folder) = app_handle.path().app_data_dir() {
                    let folder = folder.join("daily");
                    let _ = capture_to_folder(&state, &folder);
                }
                thread::sleep(Duration::from_secs(5));
            });

            let toggle = MenuItemBuilder::with_id("toggle", "Toggle recording").build(app)?;
            let open = MenuItemBuilder::with_id("open", "Open Eyes").build(app)?;
            let quit = MenuItemBuilder::with_id("quit", "Quit").build(app)?;
            let menu = MenuBuilder::new(app)
                .items(&[&toggle, &open, &quit])
                .build()?;
            TrayIconBuilder::new()
                .menu(&menu)
                .show_menu_on_left_click(false)
                .on_menu_event(|app, event| match event.id().as_ref() {
                    "toggle" => {
                        let state = app.state::<CaptureState>();
                        state.running.fetch_xor(true, Ordering::Relaxed);
                    }
                    "open" => show_main(app),
                    "quit" => app.exit(0),
                    _ => {}
                })
                .on_tray_icon_event(|tray, event| {
                    if matches!(
                        event,
                        TrayIconEvent::Click {
                            button: MouseButton::Left,
                            button_state: MouseButtonState::Up,
                            ..
                        }
                    ) {
                        show_main(tray.app_handle());
                    }
                })
                .build(app)?;
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running Eyes");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn repeated_snapshots_are_detected() {
        let snapshot = reader::Snapshot {
            app: "Editor".into(),
            window_title: Some("notes".into()),
            text: vec!["same work".into()],
            ..reader::Snapshot::default()
        };
        assert!(same_snapshot(Some(&snapshot), &snapshot));
        assert!(!same_snapshot(None, &snapshot));
    }
}
