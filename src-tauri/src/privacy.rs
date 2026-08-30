use crate::reader::Snapshot;
use regex::Regex;
use std::sync::OnceLock;

const EXCLUDED_APPS: &[&str] = &[
    "1password",
    "bitwarden",
    "dashlane",
    "enpass",
    "keepass",
    "lastpass",
    "nordpass",
    "proton pass",
    "passwords",
    "keychain",
];

const PRIVATE_MARKERS: &[&str] = &["private browsing", "incognito", "inprivate"];

fn secret_patterns() -> &'static [Regex] {
    static PATTERNS: OnceLock<Vec<Regex>> = OnceLock::new();
    PATTERNS.get_or_init(|| {
        vec![
            Regex::new(r"\bAKIA[0-9A-Z]{16}\b").unwrap(),
            Regex::new(r"\b(?:sk|pk|rk)[-_][A-Za-z0-9_-]{16,}\b").unwrap(),
            Regex::new(r"(?i)\bbearer\s+[A-Za-z0-9._-]{16,}").unwrap(),
            Regex::new(r"(?i)\b(?:api[_-]?key|secret|token|password|passwd)\b\s*[:=]\s*\S+")
                .unwrap(),
            Regex::new(r"\b(?:\d[ -]?){13,19}\b").unwrap(),
        ]
    })
}

pub fn redact_line(line: &str) -> String {
    secret_patterns()
        .iter()
        .fold(line.to_owned(), |value, pattern| {
            pattern.replace_all(&value, "[redacted]").into_owned()
        })
}

pub fn redact_snapshot(snapshot: Snapshot) -> Option<Snapshot> {
    let app = snapshot.app.to_lowercase();
    if EXCLUDED_APPS.iter().any(|name| app.contains(name)) {
        return None;
    }
    if snapshot.window_title.as_deref().is_some_and(|title| {
        PRIVATE_MARKERS
            .iter()
            .any(|marker| title.to_lowercase().contains(marker))
    }) {
        return None;
    }
    Some(Snapshot {
        app: redact_line(&snapshot.app),
        window_title: snapshot.window_title.map(|value| redact_line(&value)),
        document: snapshot.document.map(|value| redact_line(&value)),
        url: snapshot.url.map(|value| redact_line(&value)),
        text: snapshot
            .text
            .iter()
            .map(|value| redact_line(value))
            .collect(),
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn removes_secrets_before_the_snapshot_is_written() {
        let snapshot = Snapshot {
            app: "Firefox".into(),
            window_title: Some("Dashboard".into()),
            text: vec!["token: abcdefghijklmnop".into(), "ordinary work".into()],
            ..Snapshot::default()
        };
        let clean = redact_snapshot(snapshot).unwrap();
        assert_eq!(clean.text[0], "[redacted]");
        assert_eq!(clean.text[1], "ordinary work");
    }

    #[test]
    fn drops_private_windows_and_password_managers() {
        assert!(redact_snapshot(Snapshot {
            app: "Chrome".into(),
            window_title: Some("Private Browsing".into()),
            ..Snapshot::default()
        })
        .is_none());
        assert!(redact_snapshot(Snapshot {
            app: "Bitwarden".into(),
            ..Snapshot::default()
        })
        .is_none());
    }
}
