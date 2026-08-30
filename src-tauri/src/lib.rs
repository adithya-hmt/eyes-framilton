mod privacy;
mod reader;
mod writer;

use chrono::Local;
use serde::Serialize;
use std::path::PathBuf;
use std::sync::atomic::{AtomicBool, AtomicUsize, Ordering};
use std::sync::Arc;
use tauri::menu::{MenuBuilder, MenuItemBuilder};
use tauri::tray::{MouseButton, MouseButtonState, TrayIconBuilder, TrayIconEvent};
use tauri::{AppHandle, Manager};

#[derive(Clone, Default)]
struct CaptureState {
    running: Arc<AtomicBool>,
    blocks_today: Arc<AtomicUsize>,
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
fn capture_once(state: tauri::State<'_, CaptureState>, folder: String) -> Result<Status, String> {
    if !state.running.load(Ordering::Relaxed) {
        return Ok(status(&state));
    }
    let Some(snapshot) = reader::snapshot() else {
        return Ok(status(&state));
    };
    let Some(clean) = privacy::redact_snapshot(snapshot) else {
        return Ok(status(&state));
    };
    writer::append_snapshot(&PathBuf::from(folder), &clean, Local::now())
        .map_err(|error| error.to_string())?;
    state.blocks_today.fetch_add(1, Ordering::Relaxed);
    Ok(status(&state))
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
