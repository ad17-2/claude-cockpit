use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;
use std::fs;
use std::path::Path;

use super::utils;

fn entity_dir(entity_type: &str, scope: &str) -> std::path::PathBuf {
    if scope == "global" {
        utils::claude_dir().join(entity_type)
    } else {
        std::path::PathBuf::from(scope)
            .join(".claude")
            .join(entity_type)
    }
}

fn parse_frontmatter(content: &str) -> (HashMap<String, String>, String) {
    let mut frontmatter = HashMap::new();
    let body;

    if content.starts_with("---") {
        let rest = &content[3..];
        if let Some(end_idx) = rest.find("\n---") {
            let fm_str = &rest[..end_idx];
            body = rest[end_idx + 4..].trim_start_matches('\n').to_string();

            for line in fm_str.lines() {
                let line = line.trim();
                if line.is_empty() {
                    continue;
                }
                if let Some(colon_idx) = line.find(':') {
                    let key = line[..colon_idx].trim().to_string();
                    let value = line[colon_idx + 1..].trim().to_string();
                    frontmatter.insert(key, value);
                }
            }
        } else {
            body = content.to_string();
        }
    } else {
        body = content.to_string();
    }

    (frontmatter, body)
}

#[derive(Debug, Serialize, Deserialize)]
pub struct EntityInfo {
    pub name: String,
    pub entity_type: String,
    pub scope: String,
    pub description: String,
    pub file_path: String,
}

#[derive(Debug, Serialize)]
pub struct EntityDetail {
    pub frontmatter: Value,
    pub body: String,
    pub file_path: String,
}

#[tauri::command]
pub fn list_entities(entity_type: String) -> Result<Vec<EntityInfo>, String> {
    utils::validate_entity_type(&entity_type)?;

    let mut entities = Vec::new();

    let dir = entity_dir(&entity_type, "global");
    if dir.exists() {
        collect_entities(&dir, &entity_type, "global", &mut entities)?;
    }

    entities.sort_by(|a, b| a.name.to_lowercase().cmp(&b.name.to_lowercase()));
    Ok(entities)
}

fn collect_entities(
    dir: &Path,
    entity_type: &str,
    scope: &str,
    entities: &mut Vec<EntityInfo>,
) -> Result<(), String> {
    let entries = fs::read_dir(dir).map_err(|e| e.to_string())?;

    for entry in entries.flatten() {
        let path = entry.path();
        if path.extension().and_then(|e| e.to_str()) != Some("md") {
            continue;
        }

        let content = fs::read_to_string(&path).map_err(|e| e.to_string())?;
        let (fm, _) = parse_frontmatter(&content);

        let file_name = path
            .file_stem()
            .and_then(|s| s.to_str())
            .unwrap_or("")
            .to_string();

        let name = fm
            .get("name")
            .cloned()
            .unwrap_or_else(|| file_name.clone());

        let description = fm.get("description").cloned().unwrap_or_default();

        entities.push(EntityInfo {
            name,
            entity_type: entity_type.to_string(),
            scope: scope.to_string(),
            description,
            file_path: path.to_string_lossy().to_string(),
        });
    }

    Ok(())
}

#[tauri::command]
pub fn read_entity(
    entity_type: String,
    scope: String,
    name: String,
) -> Result<EntityDetail, String> {
    utils::validate_entity_type(&entity_type)?;
    utils::validate_scope(&scope)?;
    utils::validate_safe_name(&name)?;

    let dir = entity_dir(&entity_type, &scope);
    let path = dir.join(format!("{}.md", name));

    if !path.exists() {
        return Err(format!("Entity not found: {}", path.display()));
    }

    let content = fs::read_to_string(&path).map_err(|e| e.to_string())?;
    let (fm, body) = parse_frontmatter(&content);

    let frontmatter = serde_json::to_value(fm).map_err(|e| e.to_string())?;

    Ok(EntityDetail {
        frontmatter,
        body,
        file_path: path.to_string_lossy().to_string(),
    })
}

#[tauri::command]
pub fn write_entity(
    entity_type: String,
    scope: String,
    name: String,
    content: String,
) -> Result<(), String> {
    utils::validate_entity_type(&entity_type)?;
    utils::validate_scope(&scope)?;
    utils::validate_safe_name(&name)?;

    let dir = entity_dir(&entity_type, &scope);
    fs::create_dir_all(&dir).map_err(|e| e.to_string())?;

    let path = dir.join(format!("{}.md", name));
    fs::write(&path, content).map_err(|e| e.to_string())
}

#[tauri::command]
pub fn delete_entity(
    entity_type: String,
    scope: String,
    name: String,
) -> Result<(), String> {
    utils::validate_entity_type(&entity_type)?;
    utils::validate_scope(&scope)?;
    utils::validate_safe_name(&name)?;

    let dir = entity_dir(&entity_type, &scope);
    let path = dir.join(format!("{}.md", name));

    if path.exists() {
        fs::remove_file(&path).map_err(|e| e.to_string())?;
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_frontmatter() {
        let content = "---\nname: test\ndescription: A test agent\nmodel: haiku\n---\nBody content here";
        let (fm, body) = parse_frontmatter(content);
        assert_eq!(fm.get("name").unwrap(), "test");
        assert_eq!(fm.get("description").unwrap(), "A test agent");
        assert_eq!(fm.get("model").unwrap(), "haiku");
        assert_eq!(body, "Body content here");
    }

    #[test]
    fn test_parse_no_frontmatter() {
        let content = "Just body content";
        let (fm, body) = parse_frontmatter(content);
        assert!(fm.is_empty());
        assert_eq!(body, "Just body content");
    }
}
