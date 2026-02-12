use serde_json::Value;
use std::fs;
use std::path::PathBuf;

use super::utils;

fn settings_path(scope: &str, file_type: &str) -> PathBuf {
    let filename = match file_type {
        "settings_local" => "settings.local.json",
        _ => "settings.json",
    };

    if scope == "global" {
        utils::claude_dir().join(filename)
    } else {
        PathBuf::from(scope).join(".claude").join(filename)
    }
}

fn merge_json(base: &Value, overlay: &Value) -> Value {
    match (base, overlay) {
        (Value::Object(base_map), Value::Object(overlay_map)) => {
            let mut merged = base_map.clone();
            for (key, overlay_val) in overlay_map {
                let merged_val = if let Some(base_val) = merged.get(key) {
                    merge_json(base_val, overlay_val)
                } else {
                    overlay_val.clone()
                };
                merged.insert(key.clone(), merged_val);
            }
            Value::Object(merged)
        }
        _ => overlay.clone(),
    }
}

#[tauri::command]
pub fn read_settings(scope: String, file_type: String) -> Result<Value, String> {
    utils::validate_scope(&scope)?;
    let path = settings_path(&scope, &file_type);
    utils::read_json_file(&path)
}

#[tauri::command]
pub fn write_settings(scope: String, file_type: String, content: Value) -> Result<(), String> {
    utils::validate_scope(&scope)?;
    let path = settings_path(&scope, &file_type);

    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent)
            .map_err(|e| format!("Failed to create directory {}: {}", parent.display(), e))?;
    }

    let json_str = serde_json::to_string_pretty(&content)
        .map_err(|e| format!("Failed to serialize: {}", e))?;

    fs::write(&path, json_str.as_bytes())
        .map_err(|e| format!("Failed to write {}: {}", path.display(), e))
}

#[tauri::command]
pub fn get_effective_settings(project_path: String) -> Result<Value, String> {
    utils::validate_scope(&project_path)?;
    let global_settings = utils::read_json_file(&settings_path("global", "settings"))?;
    let global_local = utils::read_json_file(&settings_path("global", "settings_local"))?;
    let project_settings = utils::read_json_file(&settings_path(&project_path, "settings"))?;
    let project_local = utils::read_json_file(&settings_path(&project_path, "settings_local"))?;

    let merged = merge_json(&global_settings, &global_local);
    let merged = merge_json(&merged, &project_settings);
    let merged = merge_json(&merged, &project_local);

    Ok(merged)
}

#[tauri::command]
pub fn add_permission(
    scope: String,
    file_type: String,
    category: String,
    permission: String,
) -> Result<(), String> {
    utils::validate_scope(&scope)?;
    let path = settings_path(&scope, &file_type);
    let mut settings = utils::read_json_file(&path)?;

    let permissions = settings
        .as_object_mut()
        .ok_or("Settings is not an object")?
        .entry("permissions")
        .or_insert_with(|| Value::Object(serde_json::Map::new()));

    let arr = permissions
        .as_object_mut()
        .ok_or("permissions is not an object")?
        .entry(&category)
        .or_insert_with(|| Value::Array(Vec::new()));

    let list = arr.as_array_mut().ok_or("category is not an array")?;

    if !list.iter().any(|v| v.as_str() == Some(&permission)) {
        list.push(Value::String(permission));
    }

    let json_str = serde_json::to_string_pretty(&settings)
        .map_err(|e| format!("Failed to serialize: {}", e))?;

    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent)
            .map_err(|e| format!("Failed to create directory: {}", e))?;
    }

    fs::write(&path, json_str.as_bytes())
        .map_err(|e| format!("Failed to write {}: {}", path.display(), e))
}

#[tauri::command]
pub fn remove_permission(
    scope: String,
    file_type: String,
    category: String,
    permission: String,
) -> Result<(), String> {
    utils::validate_scope(&scope)?;
    let path = settings_path(&scope, &file_type);
    let mut settings = utils::read_json_file(&path)?;

    if let Some(permissions) = settings.get_mut("permissions") {
        if let Some(arr) = permissions.get_mut(&category) {
            if let Some(list) = arr.as_array_mut() {
                list.retain(|v| v.as_str() != Some(&permission));
            }
        }
    }

    let json_str = serde_json::to_string_pretty(&settings)
        .map_err(|e| format!("Failed to serialize: {}", e))?;

    fs::write(&path, json_str.as_bytes())
        .map_err(|e| format!("Failed to write {}: {}", path.display(), e))
}
