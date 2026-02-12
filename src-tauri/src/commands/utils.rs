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
    encoded
        .replace('-', "/")
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

pub fn list_jsonl_files(dir: &Path) -> Result<Vec<fs::DirEntry>, String> {
    let entries: Vec<_> = fs::read_dir(dir)
        .map_err(|e| e.to_string())?
        .filter_map(|e| e.ok())
        .filter(|e| {
            e.path()
                .extension()
                .and_then(|ext| ext.to_str())
                == Some("jsonl")
        })
        .collect();
    Ok(entries)
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

const ALLOWED_ENTITY_TYPES: &[&str] = &["agents", "rules", "commands", "skills", "hooks"];

pub fn validate_entity_type(entity_type: &str) -> Result<(), String> {
    if ALLOWED_ENTITY_TYPES.contains(&entity_type) {
        Ok(())
    } else {
        Err(format!("Invalid entity type: {}", entity_type))
    }
}

pub fn validate_safe_name(name: &str) -> Result<(), String> {
    if name.is_empty() {
        return Err("Name cannot be empty".to_string());
    }
    if name.contains('/') || name.contains('\\') || name.contains("..") {
        return Err("Name contains invalid characters".to_string());
    }
    if name.starts_with('.') {
        return Err("Name cannot start with a dot".to_string());
    }
    Ok(())
}

pub fn validate_scope(scope: &str) -> Result<(), String> {
    if scope == "global" {
        return Ok(());
    }
    let path = PathBuf::from(scope);
    if !path.is_absolute() {
        return Err("Scope must be 'global' or an absolute path".to_string());
    }
    if scope.contains("..") {
        return Err("Scope contains invalid path components".to_string());
    }
    Ok(())
}

fn canonicalize_within_projects_dir(path: &Path) -> Result<PathBuf, String> {
    let canonical = path
        .canonicalize()
        .map_err(|_| format!("Invalid path: {}", path.display()))?;

    let projects = projects_dir()
        .canonicalize()
        .map_err(|_| "Cannot resolve projects directory".to_string())?;

    if !canonical.starts_with(&projects) {
        return Err("Path outside allowed directory".to_string());
    }

    Ok(canonical)
}

pub fn validate_session_path(session_path: &str) -> Result<PathBuf, String> {
    let path = PathBuf::from(session_path);
    if !path.is_absolute() {
        return Err("Session path must be absolute".to_string());
    }

    let canonical = canonicalize_within_projects_dir(&path)?;

    if canonical.extension().and_then(|e| e.to_str()) != Some("jsonl") {
        return Err("Invalid file type".to_string());
    }

    Ok(canonical)
}

pub fn validate_within_projects_dir(path: &Path) -> Result<PathBuf, String> {
    canonicalize_within_projects_dir(path)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_validate_entity_type_accepts_allowed() {
        for t in ALLOWED_ENTITY_TYPES {
            assert!(validate_entity_type(t).is_ok());
        }
    }

    #[test]
    fn test_validate_entity_type_rejects_unknown() {
        assert!(validate_entity_type("malicious").is_err());
        assert!(validate_entity_type("").is_err());
        assert!(validate_entity_type("../agents").is_err());
    }

    #[test]
    fn test_validate_safe_name_accepts_valid() {
        assert!(validate_safe_name("my-agent").is_ok());
        assert!(validate_safe_name("rule_v2").is_ok());
        assert!(validate_safe_name("CamelCase").is_ok());
    }

    #[test]
    fn test_validate_safe_name_rejects_empty() {
        assert!(validate_safe_name("").is_err());
    }

    #[test]
    fn test_validate_safe_name_rejects_path_separators() {
        assert!(validate_safe_name("../etc/passwd").is_err());
        assert!(validate_safe_name("foo/bar").is_err());
        assert!(validate_safe_name("foo\\bar").is_err());
    }

    #[test]
    fn test_validate_safe_name_rejects_dot_prefix() {
        assert!(validate_safe_name(".hidden").is_err());
        assert!(validate_safe_name("..sneaky").is_err());
    }

    #[test]
    fn test_validate_scope_accepts_global() {
        assert!(validate_scope("global").is_ok());
    }

    #[test]
    fn test_validate_scope_accepts_absolute_path() {
        assert!(validate_scope("/Users/test/project").is_ok());
    }

    #[test]
    fn test_validate_scope_rejects_relative_path() {
        assert!(validate_scope("relative/path").is_err());
        assert!(validate_scope("").is_err());
    }

    #[test]
    fn test_validate_scope_rejects_traversal() {
        assert!(validate_scope("/Users/test/../etc").is_err());
    }

    #[test]
    fn test_validate_session_path_rejects_relative() {
        assert!(validate_session_path("relative/file.jsonl").is_err());
    }
}
