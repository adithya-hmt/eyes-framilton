use crate::reader::Snapshot;
use chrono::{DateTime, Local};
use std::fs::{self, OpenOptions};
use std::io::Write;
use std::path::{Path, PathBuf};

pub fn day_path(folder: &Path, at: DateTime<Local>) -> PathBuf {
    folder.join(format!("{}.md", at.format("%Y-%m-%d")))
}

pub fn append_snapshot(
    folder: &Path,
    snapshot: &Snapshot,
    at: DateTime<Local>,
) -> std::io::Result<()> {
    fs::create_dir_all(folder)?;
    let path = day_path(folder, at);
    let new_file = !path.exists();
    let mut file = OpenOptions::new().create(true).append(true).open(path)?;
    if new_file {
        writeln!(file, "---")?;
        writeln!(file, "date: {}", at.format("%Y-%m-%d"))?;
        writeln!(file, "captured_by: Eyes 0.1.0")?;
        writeln!(file, "---\n")?;
    }
    let source = snapshot
        .url
        .as_deref()
        .or(snapshot.document.as_deref())
        .unwrap_or("");
    writeln!(file, "## {} · {}", at.format("%H:%M"), snapshot.app)?;
    if let Some(title) = snapshot.window_title.as_deref() {
        writeln!(file, "\n{}", title)?;
    }
    if !source.is_empty() {
        writeln!(file, "\nsource: {}", source)?;
    }
    for line in snapshot
        .text
        .iter()
        .filter(|line| !line.trim().is_empty())
        .take(80)
    {
        writeln!(file, "\n{}", line)?;
    }
    writeln!(file)?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn creates_a_readable_day_file() {
        let temp = tempfile::tempdir().unwrap();
        let at = Local::now();
        append_snapshot(
            temp.path(),
            &Snapshot {
                app: "Editor".into(),
                window_title: Some("notes".into()),
                text: vec!["ship it".into()],
                ..Snapshot::default()
            },
            at,
        )
        .unwrap();
        let output = std::fs::read_to_string(day_path(temp.path(), at)).unwrap();
        assert!(output.contains("captured_by: Eyes 0.1.0"));
        assert!(output.contains("ship it"));
    }
}
