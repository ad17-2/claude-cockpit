use serde_json::Value;
use std::fs;

use super::utils;

#[tauri::command]
pub fn read_stats_cache() -> Result<Value, String> {
    let base = utils::claude_dir();
    let candidates = ["stats-cache.json", "statsig-cache.json"];

    for name in &candidates {
        let path = base.join(name);
        if path.exists() {
            let content = fs::read_to_string(&path)
                .map_err(|e| format!("Failed to read {}: {}", name, e))?;
            return serde_json::from_str(&content)
                .map_err(|e| format!("Failed to parse {}: {}", name, e));
        }
    }

    Ok(Value::Null)
}
