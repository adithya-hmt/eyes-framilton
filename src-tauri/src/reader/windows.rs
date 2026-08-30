#![cfg(target_os = "windows")]

use super::Snapshot;
use std::path::Path;
use uiautomation::patterns::UITextPattern;
use uiautomation::types::ControlType;
use uiautomation::{UIAutomation, UIElement};
use windows::core::PWSTR;
use windows::Win32::Foundation::CloseHandle;
use windows::Win32::System::Threading::{
    OpenProcess, QueryFullProcessImageNameW, PROCESS_NAME_WIN32, PROCESS_QUERY_LIMITED_INFORMATION,
};

const MAX_NODES: usize = 512;
const MAX_TEXT_CHARS: i32 = 20_000;
const MAX_LINES: usize = 240;

fn append_lines(output: &mut Vec<String>, value: &str) {
    output.extend(
        value
            .lines()
            .map(str::trim)
            .filter(|line| !line.is_empty())
            .map(str::to_owned)
            .take(MAX_LINES.saturating_sub(output.len())),
    );
}

pub fn snapshot() -> Option<Snapshot> {
    let automation = UIAutomation::new().ok()?;
    let focused = automation.get_focused_element().ok()?;
    if focused.is_password().ok()? {
        return None;
    }

    let walker = automation.get_control_view_walker().ok()?;
    let window = window_ancestor(&walker, &focused).unwrap_or(focused);
    if window.is_password().ok()? {
        return None;
    }

    let window_title = window.get_name().ok().filter(|value| !value.is_empty());
    let app = process_name(&window)
        .or_else(|| window_title.clone())
        .unwrap_or_else(|| "Windows".to_owned());
    let mut text = Vec::new();
    collect_text(&walker, &window, &mut text);

    if window_title.is_none() && text.is_empty() {
        return None;
    }
    Some(Snapshot {
        app,
        window_title,
        document: None,
        url: None,
        text,
    })
}

fn window_ancestor(walker: &uiautomation::UITreeWalker, focused: &UIElement) -> Option<UIElement> {
    let mut current = focused.clone();
    for _ in 0..32 {
        if current.get_control_type().ok() == Some(ControlType::Window) {
            return Some(current);
        }
        current = walker.get_parent(&current).ok()?;
    }
    None
}

fn collect_text(walker: &uiautomation::UITreeWalker, window: &UIElement, output: &mut Vec<String>) {
    let mut pending = vec![window.clone()];
    let mut visited = 0;

    while let Some(node) = pending.pop() {
        visited += 1;
        if visited > MAX_NODES || output.len() >= MAX_LINES {
            break;
        }
        if node.is_password().unwrap_or(true) {
            continue;
        }

        if let Ok(text_pattern) = node.get_pattern::<UITextPattern>() {
            if let Ok(range) = text_pattern.get_document_range() {
                if let Ok(value) = range.get_text(MAX_TEXT_CHARS) {
                    append_lines(output, &value);
                }
            }
        }

        let Ok(first_child) = walker.get_first_child(&node) else {
            continue;
        };
        let mut sibling = first_child;
        loop {
            pending.push(sibling.clone());
            let Ok(next) = walker.get_next_sibling(&sibling) else {
                break;
            };
            sibling = next;
        }
    }
}

fn process_name(window: &UIElement) -> Option<String> {
    let pid = window.get_process_id().ok()?;
    let process = unsafe { OpenProcess(PROCESS_QUERY_LIMITED_INFORMATION, false, pid).ok()? };
    let mut path = [0u16; 260];
    let mut length = path.len() as u32;
    let result = unsafe {
        QueryFullProcessImageNameW(
            process,
            PROCESS_NAME_WIN32,
            PWSTR(path.as_mut_ptr()),
            &mut length,
        )
    };
    let _ = unsafe { CloseHandle(process) };
    result.ok()?;

    Path::new(&String::from_utf16_lossy(&path[..length as usize]))
        .file_stem()
        .and_then(|name| name.to_str())
        .filter(|name| !name.is_empty())
        .map(str::to_owned)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn text_collection_is_bounded() {
        let mut lines = Vec::new();
        append_lines(&mut lines, "one\ntwo\nthree");
        assert_eq!(lines, ["one", "two", "three"]);
    }
}
