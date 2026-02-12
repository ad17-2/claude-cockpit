use std::fs;
use std::path::PathBuf;

use super::utils;

fn global_claude_md_path() -> PathBuf {
    utils::claude_dir().join("CLAUDE.md")
}

fn project_claude_md_path(project_path: &str) -> PathBuf {
    let root = PathBuf::from(project_path);
    let dotclaude = root.join(".claude").join("CLAUDE.md");
    if dotclaude.exists() {
        return dotclaude;
    }
    root.join("CLAUDE.md")
}

fn project_claude_md_write_path(project_path: &str) -> PathBuf {
    let root = PathBuf::from(project_path);
    let dotclaude_dir = root.join(".claude");
    if dotclaude_dir.exists() {
        return dotclaude_dir.join("CLAUDE.md");
    }
    root.join("CLAUDE.md")
}

#[tauri::command]
pub fn read_claude_md(scope: String) -> Result<Option<String>, String> {
    let path = if scope == "global" {
        global_claude_md_path()
    } else {
        project_claude_md_path(&scope)
    };

    if !path.exists() {
        return Ok(None);
    }

    fs::read_to_string(&path)
        .map(Some)
        .map_err(|e| format!("Failed to read {}: {}", path.display(), e))
}

#[tauri::command]
pub fn write_claude_md(scope: String, content: String) -> Result<(), String> {
    let path = if scope == "global" {
        global_claude_md_path()
    } else {
        project_claude_md_write_path(&scope)
    };

    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent)
            .map_err(|e| format!("Failed to create directory {}: {}", parent.display(), e))?;
    }

    fs::write(&path, content)
        .map_err(|e| format!("Failed to write {}: {}", path.display(), e))
}
