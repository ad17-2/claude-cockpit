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
}

#[tauri::command]
pub fn decode_project_path(encoded: String) -> String {
    utils::decode_encoded_path(&encoded)
}

#[tauri::command]
pub fn list_projects() -> Result<Vec<ProjectInfo>, String> {
    let projects_dir = utils::projects_dir();

    if !projects_dir.exists() {
        return Ok(Vec::new());
    }

    let entries = fs::read_dir(&projects_dir).map_err(|e| e.to_string())?;

    let mut projects: Vec<ProjectInfo> = entries
        .filter_map(|entry| {
            let entry = entry.ok()?;
            let file_type = entry.file_type().ok()?;
            if !file_type.is_dir() {
                return None;
            }

            let dir_name = entry.file_name().into_string().ok()?;
            if !dir_name.starts_with('-') {
                return None;
            }

            let decoded_path = utils::decode_encoded_path(&dir_name);
            let name = utils::decode_project_name(&dir_name);

            let project_root = PathBuf::from(&decoded_path);
            let has_claude_md = project_root.join("CLAUDE.md").exists()
                || project_root.join(".claude").join("CLAUDE.md").exists();
            let has_settings = project_root.join(".claude").join("settings.json").exists();

            Some(ProjectInfo {
                encoded_path: dir_name,
                decoded_path,
                name,
                has_claude_md,
                has_settings,
            })
        })
        .collect();

    projects.sort_by(|a, b| a.name.to_lowercase().cmp(&b.name.to_lowercase()));

    Ok(projects)
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
