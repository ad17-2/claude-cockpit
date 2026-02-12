use serde::Serialize;
use std::fs;
use std::path::PathBuf;

use super::utils;

#[derive(Debug, Serialize)]
pub struct ProjectInfo {
    pub encoded_path: String,
    pub decoded_path: String,
    pub name: String,
    pub has_claude_md: bool,
    pub has_settings: bool,
    pub is_root: bool,
}

#[tauri::command]
pub fn decode_project_path(encoded: String) -> String {
    utils::decode_encoded_path(&encoded)
}

#[tauri::command]
pub fn list_projects() -> Result<Vec<ProjectInfo>, String> {
    let home = dirs::home_dir().map(|p| p.to_string_lossy().to_string());

    let mut projects: Vec<ProjectInfo> = utils::list_project_dirs()?
        .into_iter()
        .filter_map(|entry| {
            let dir_name = entry.file_name().into_string().ok()?;
            let decoded_path = utils::decode_encoded_path(&dir_name);
            let name = utils::decode_project_name(&dir_name);

            let project_root = PathBuf::from(&decoded_path);
            let claude_dir = project_root.join(".claude");
            let has_claude_md =
                project_root.join("CLAUDE.md").exists() || claude_dir.join("CLAUDE.md").exists();
            let has_settings = claude_dir.join("settings.json").exists();

            let is_root = home.as_deref() == Some(decoded_path.as_str());

            Some(ProjectInfo {
                encoded_path: dir_name,
                decoded_path,
                name,
                has_claude_md,
                has_settings,
                is_root,
            })
        })
        .collect();

    projects.sort_by(|a, b| a.name.to_lowercase().cmp(&b.name.to_lowercase()));

    Ok(projects)
}

#[tauri::command]
pub fn delete_project(encoded_path: String) -> Result<(), String> {
    if !encoded_path.starts_with('-') {
        return Err("Invalid project path".to_string());
    }

    let project_dir = utils::projects_dir().join(&encoded_path);
    if !project_dir.is_dir() {
        return Ok(());
    }

    let canonical = utils::validate_within_projects_dir(&project_dir)?;
    fs::remove_dir_all(&canonical).map_err(|e| e.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_decode_encoded_path() {
        assert_eq!(
            utils::decode_encoded_path("-Users-adtzy-Finku"),
            "/Users/adtzy/Finku"
        );
        assert_eq!(
            utils::decode_encoded_path("-Users-adtzy-Personal"),
            "/Users/adtzy/Personal"
        );
        assert_eq!(
            utils::decode_encoded_path("-Users-adtzy-Finku-finku-users"),
            "/Users/adtzy/Finku/finku/users"
        );
    }

    #[test]
    fn test_decode_project_name() {
        assert_eq!(
            utils::decode_project_name("-Users-adtzy-Finku"),
            "Finku"
        );
        assert_eq!(
            utils::decode_project_name("-Users-adtzy-Personal"),
            "Personal"
        );
    }
}
