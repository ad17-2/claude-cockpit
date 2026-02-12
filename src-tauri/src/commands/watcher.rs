use notify::RecursiveMode;
use notify_debouncer_mini::new_debouncer;
use std::path::Path;
use std::sync::OnceLock;
use std::time::Duration;
use tauri::{AppHandle, Emitter};

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

    let claude_dir = dirs::home_dir()
        .ok_or("could not find home directory")?
        .join(".claude");

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

        loop {
            match rx.recv() {
                Ok(Ok(events)) => {
                    let mut emitted = std::collections::HashSet::new();
                    for event in &events {
                        if let Some(event_name) = classify_event(&event.path) {
                            if emitted.insert(event_name) {
                                let _ = app_handle.emit(event_name, ());
                            }
                        }
                    }
                }
                Ok(Err(e)) => {
                    eprintln!("Watch error: {}", e);
                }
                Err(_) => break,
            }
        }
    });

    let _ = WATCHER_STARTED.set(());
    Ok(())
}
