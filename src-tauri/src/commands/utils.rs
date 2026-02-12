use serde_json::Value;
use std::fs;
use std::path::{Path, PathBuf};

pub fn claude_dir() -> PathBuf {
    dirs::home_dir()
        .expect("could not find home directory")
        .join(".claude")
}

pub fn projects_dir() -> PathBuf {
    claude_dir().join("projects")
}

pub fn decode_encoded_path(encoded: &str) -> String {
    encoded.replace('-', "/")
}

pub fn decode_project_name(encoded: &str) -> String {
    let decoded = encoded.replace('-', "/");
    decoded
        .rsplit('/')
        .find(|s| !s.is_empty())
        .unwrap_or(encoded)
        .to_string()
}

pub fn truncate_str(s: &str, max_chars: usize) -> String {
    let truncated: String = s.chars().take(max_chars).collect();
    if truncated.len() < s.len() {
        format!("{}...", truncated)
    } else {
        truncated
    }
}

pub fn extract_text_content(content: &Value) -> String {
    match content {
        Value::String(s) => truncate_str(s.trim(), 200),
        Value::Array(arr) => {
            for item in arr {
                if item.get("type").and_then(|t| t.as_str()) == Some("text") {
                    if let Some(text) = item.get("text").and_then(|t| t.as_str()) {
                        return truncate_str(text.trim(), 200);
                    }
                }
            }
            String::new()
        }
        _ => String::new(),
    }
}

pub fn read_json_file(path: &Path) -> Result<Value, String> {
    if !path.exists() {
        return Ok(Value::Object(serde_json::Map::new()));
    }
    let content = fs::read_to_string(path)
        .map_err(|e| format!("Failed to read {}: {}", path.display(), e))?;
    serde_json::from_str(&content)
        .map_err(|e| format!("Failed to parse {}: {}", path.display(), e))
}

pub fn list_project_dirs() -> Result<Vec<fs::DirEntry>, String> {
    let projects_dir = projects_dir();
    if !projects_dir.exists() {
        return Ok(Vec::new());
    }
    let entries: Vec<_> = fs::read_dir(&projects_dir)
        .map_err(|e| e.to_string())?
        .filter_map(|e| e.ok())
        .filter(|e| e.file_type().ok().map_or(false, |ft| ft.is_dir()))
        .filter(|e| {
            e.file_name()
                .to_str()
                .map_or(false, |n| n.starts_with('-'))
        })
        .collect();
    Ok(entries)
}
