use serde::Serialize;
use serde_json::Value;
use std::fs;
use std::path::PathBuf;

fn claude_dir() -> PathBuf {
    dirs::home_dir()
        .expect("could not find home directory")
        .join(".claude")
}

fn read_json_file(path: &std::path::Path) -> Result<Value, String> {
    if !path.exists() {
        return Ok(Value::Object(serde_json::Map::new()));
    }
    let content = fs::read_to_string(path)
        .map_err(|e| format!("Failed to read {}: {}", path.display(), e))?;
    serde_json::from_str(&content)
        .map_err(|e| format!("Failed to parse {}: {}", path.display(), e))
}

#[derive(Debug, Serialize)]
pub struct McpServerInfo {
    pub name: String,
    pub command: Option<String>,
    pub args: Vec<String>,
    pub url: Option<String>,
    pub env_keys: Vec<String>,
    pub scope: String,
}

fn extract_mcp_servers(settings: &Value, scope: &str) -> Vec<McpServerInfo> {
    let mut servers = Vec::new();

    let mcp_servers = match settings.get("mcpServers") {
        Some(Value::Object(map)) => map,
        _ => return servers,
    };

    for (name, config) in mcp_servers {
        let command = config
            .get("command")
            .and_then(|c| c.as_str())
            .map(|s| s.to_string());

        let args = config
            .get("args")
            .and_then(|a| a.as_array())
            .map(|arr| {
                arr.iter()
                    .filter_map(|v| v.as_str().map(|s| s.to_string()))
                    .collect()
            })
            .unwrap_or_default();

        let url = config
            .get("url")
            .and_then(|u| u.as_str())
            .map(|s| s.to_string());

        let env_keys = config
            .get("env")
            .and_then(|e| e.as_object())
            .map(|obj| obj.keys().cloned().collect())
            .unwrap_or_default();

        servers.push(McpServerInfo {
            name: name.clone(),
            command,
            args,
            url,
            env_keys,
            scope: scope.to_string(),
        });
    }

    servers
}

#[tauri::command]
pub fn list_mcp_servers() -> Result<Vec<McpServerInfo>, String> {
    let mut all_servers = Vec::new();

    let global_path = claude_dir().join("settings.json");
    let global_settings = read_json_file(&global_path)?;
    all_servers.extend(extract_mcp_servers(&global_settings, "global"));

    let global_local_path = claude_dir().join("settings.local.json");
    let global_local = read_json_file(&global_local_path)?;
    all_servers.extend(extract_mcp_servers(&global_local, "global"));

    let projects_dir = claude_dir().join("projects");
    if projects_dir.exists() {
        let project_dirs: Vec<_> = fs::read_dir(&projects_dir)
            .map_err(|e| e.to_string())?
            .filter_map(|e| e.ok())
            .filter(|e| e.file_type().ok().map_or(false, |ft| ft.is_dir()))
            .filter(|e| {
                e.file_name()
                    .to_str()
                    .map_or(false, |n| n.starts_with('-'))
            })
            .collect();

        for project_entry in project_dirs {
            let encoded = project_entry.file_name().to_string_lossy().to_string();
            let decoded = encoded.replace('-', "/");
            let project_name = decoded
                .rsplit('/')
                .find(|s| !s.is_empty())
                .unwrap_or(&encoded)
                .to_string();

            let project_root = PathBuf::from(&decoded);
            let project_settings_path = project_root.join(".claude").join("settings.json");
            if let Ok(settings) = read_json_file(&project_settings_path) {
                all_servers.extend(extract_mcp_servers(&settings, &project_name));
            }

            let project_local_path = project_root.join(".claude").join("settings.local.json");
            if let Ok(settings) = read_json_file(&project_local_path) {
                all_servers.extend(extract_mcp_servers(&settings, &project_name));
            }
        }
    }

    Ok(all_servers)
}
