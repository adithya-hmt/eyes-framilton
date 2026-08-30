#![cfg(target_os = "linux")]

use super::Snapshot;
use atspi::proxy::accessible::ObjectRefExt;
use atspi::proxy::proxy_ext::ProxyExt;
use atspi::{AccessibilityConnection, Interface, Role, State};
use futures_lite::future::block_on;
use std::sync::{Mutex, OnceLock};

const MAX_NODES: usize = 512;
const MAX_TEXT_CHARS: i32 = 20_000;
const MAX_LINES: usize = 240;

// ponytail: one global connection lock keeps the reader small; split per-thread only if capture throughput matters.
static CONNECTION: OnceLock<Mutex<Option<AccessibilityConnection>>> = OnceLock::new();

fn connection() -> &'static Mutex<Option<AccessibilityConnection>> {
    CONNECTION.get_or_init(|| Mutex::new(None))
}

fn skip_role(role: Role) -> bool {
    matches!(role, Role::PasswordText)
}

pub fn snapshot() -> Option<Snapshot> {
    let slot = connection();
    let mut guard = slot.lock().ok()?;
    if guard.is_none() {
        *guard = block_on(AccessibilityConnection::new()).ok();
    }
    let connection = guard.as_ref()?;
    block_on(read_focused_window(connection))
}

async fn read_focused_window(connection: &AccessibilityConnection) -> Option<Snapshot> {
    let registry = connection.root_accessible_on_registry().await.ok()?;
    let applications = registry.get_children().await.ok()?;

    for application_ref in applications {
        if application_ref.is_null() {
            continue;
        }
        let Ok(application) = application_ref
            .into_accessible_proxy(connection.connection())
            .await
        else {
            continue;
        };
        let Some(app_name) = application
            .name()
            .await
            .ok()
            .filter(|name| !name.is_empty())
        else {
            continue;
        };

        let Ok(frames) = application.get_children().await else {
            continue;
        };
        for frame_ref in frames {
            if frame_ref.is_null() {
                continue;
            }
            let Ok(frame) = frame_ref
                .into_accessible_proxy(connection.connection())
                .await
            else {
                continue;
            };
            let Ok(state) = frame.get_state().await else {
                continue;
            };
            if !state.contains(State::Active) && !state.contains(State::Focused) {
                continue;
            }
            let Ok(role) = frame.get_role().await else {
                continue;
            };
            if skip_role(role) {
                continue;
            }

            let mut snapshot = Snapshot {
                app: app_name.clone(),
                window_title: frame.name().await.ok().filter(|name| !name.is_empty()),
                ..Snapshot::default()
            };
            collect_text(&frame, connection, &mut snapshot.text).await;
            if snapshot.window_title.is_some() || !snapshot.text.is_empty() {
                return Some(snapshot);
            }
        }
    }
    None
}

async fn collect_text(
    frame: &atspi::proxy::accessible::AccessibleProxy<'_>,
    connection: &AccessibilityConnection,
    output: &mut Vec<String>,
) {
    let mut pending = vec![frame.clone()];
    let mut visited = 0;

    while let Some(node) = pending.pop() {
        visited += 1;
        if visited > MAX_NODES || output.len() >= MAX_LINES {
            break;
        }

        let Ok(role) = node.get_role().await else {
            continue;
        };
        if skip_role(role) {
            continue;
        }

        if let Ok(interfaces) = node.get_interfaces().await {
            if interfaces.contains(Interface::Text) {
                if let Ok(proxies) = node.proxies().await {
                    if let Ok(text) = proxies.text().await {
                        if let Ok(character_count) = text.character_count().await {
                            let end = character_count.clamp(0, MAX_TEXT_CHARS);
                            if end > 0 {
                                if let Ok(value) = text.get_text(0, end).await {
                                    output.extend(
                                        value
                                            .lines()
                                            .map(str::trim)
                                            .filter(|line| !line.is_empty())
                                            .map(str::to_owned)
                                            .take(MAX_LINES.saturating_sub(output.len())),
                                    );
                                }
                            }
                        }
                    }
                }
            }
        }

        let Ok(children) = node.get_children().await else {
            continue;
        };
        for child in children.into_iter().rev() {
            if !child.is_null() {
                if let Ok(proxy) = child.into_accessible_proxy(connection.connection()).await {
                    pending.push(proxy);
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn password_roles_are_never_read() {
        assert!(skip_role(Role::PasswordText));
        assert!(!skip_role(Role::Entry));
    }
}
