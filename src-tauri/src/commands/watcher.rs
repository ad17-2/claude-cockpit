use notify::RecursiveMode;
use notify_debouncer_mini::new_debouncer;
use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::sync::OnceLock;
use std::time::{Duration, Instant};
use tauri::{AppHandle, Emitter};

use super::utils;

static WATCHER_STARTED: OnceLock<()> = OnceLock::new();

fn classify_event(path: &Path) -> Option<&'static str> {
    let path_str = path.to_string_lossy();

    if path_str.ends_with("CLAUDE.md") {
        return Some("claude-md-changed");
    }
    if path_str.contains("settings") && path_str.ends_with(".json") {
        return Some("settings-changed");
    }
    if path_str.ends_with(".jsonl") {
        return Some("history-changed");
    }

    let entity_dirs = ["agents", "rules", "commands", "skills", "hooks"];
    for dir in entity_dirs {
        if path_str.contains(&format!("/{}/", dir)) || path_str.contains(&format!("\\{}\\", dir)) {
            return Some("entity-changed");
        }
    }

    None
}

#[tauri::command]
pub fn start_watching(app: AppHandle) -> Result<(), String> {
    if WATCHER_STARTED.get().is_some() {
        return Ok(());
    }

    let claude_dir = utils::claude_dir();

    if !claude_dir.exists() {
        return Ok(());
    }

    let app_handle = app.clone();

    std::thread::spawn(move || {
        let (tx, rx) = std::sync::mpsc::channel();

        let mut debouncer = match new_debouncer(Duration::from_millis(500), tx) {
            Ok(d) => d,
            Err(e) => {
                eprintln!("Failed to create file watcher: {}", e);
                return;
            }
        };

        if let Err(e) = debouncer.watcher().watch(&claude_dir, RecursiveMode::Recursive) {
            eprintln!("Failed to watch directory: {}", e);
            return;
        }

        let mut active_sessions: HashMap<PathBuf, Instant> = HashMap::new();
        let session_timeout = Duration::from_secs(60);

        loop {
            match rx.recv_timeout(Duration::from_secs(10)) {
                Ok(Ok(events)) => {
                    let mut emitted = std::collections::HashSet::new();
                    for event in &events {
                        if let Some(event_name) = classify_event(&event.path) {
                            if emitted.insert(event_name) {
                                let _ = app_handle.emit(event_name, ());
                            }
                        }

                        if event.path.extension().and_then(|e| e.to_str()) == Some("jsonl") {
                            active_sessions.insert(event.path.clone(), Instant::now());
                        }
                    }
                }
                Ok(Err(e)) => {
                    eprintln!("Watch error: {}", e);
                }
                Err(std::sync::mpsc::RecvTimeoutError::Timeout) => {}
                Err(std::sync::mpsc::RecvTimeoutError::Disconnected) => break,
            }

            let now = Instant::now();
            let completed: Vec<PathBuf> = active_sessions
                .iter()
                .filter(|(_, last_seen)| now.duration_since(**last_seen) > session_timeout)
                .map(|(path, _)| path.clone())
                .collect();

            for path in completed {
                active_sessions.remove(&path);
                let session_id = path
                    .file_stem()
                    .and_then(|s| s.to_str())
                    .unwrap_or("")
                    .to_string();
                let _ = app_handle.emit("session-completed", session_id);
            }
        }
    });

    let _ = WATCHER_STARTED.set(());
    Ok(())
}
