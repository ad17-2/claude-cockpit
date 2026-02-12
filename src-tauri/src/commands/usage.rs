use serde_json::Value;
use std::fs;
use std::path::PathBuf;

fn claude_dir() -> PathBuf {
    dirs::home_dir()
        .expect("could not find home directory")
        .join(".claude")
}

#[tauri::command]
pub fn read_stats_cache() -> Result<Value, String> {
    let base = claude_dir();
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
