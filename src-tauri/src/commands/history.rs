use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::fs;
use std::io::{BufRead, BufReader, Write};
use std::path::{Path, PathBuf};

use super::utils;

#[derive(Debug, Serialize)]
pub struct ConversationMeta {
    pub session_id: String,
    pub project: String,
    pub first_message_preview: String,
    pub timestamp: String,
    pub message_count: u32,
    pub file_path: String,
}

#[derive(Debug, Serialize)]
pub struct ConversationMessage {
    pub role: String,
    pub content: String,
    pub timestamp: String,
    pub message_type: String,
}

#[derive(Debug, Serialize)]
pub struct SearchResult {
    pub session_path: String,
    pub project: String,
    pub matched_line: String,
    pub timestamp: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct HistoryEntry {
    pub display: String,
    pub project: String,
    pub timestamp: u64,
}

#[tauri::command]
pub fn list_conversations(project_filter: Option<String>) -> Result<Vec<ConversationMeta>, String> {
    let project_dirs = utils::list_project_dirs()?;

    let mut conversations = Vec::new();

    for project_entry in project_dirs {
        let project_name = project_entry.file_name().to_string_lossy().to_string();

        if let Some(ref filter) = project_filter {
            if project_name != *filter {
                continue;
            }
        }

        let project_path = project_entry.path();
        let jsonl_files = utils::list_jsonl_files(&project_path)?;

        for file_entry in jsonl_files {
            let file_path = file_entry.path();
            let session_id = file_path
                .file_stem()
                .and_then(|s| s.to_str())
                .unwrap_or("")
                .to_string();

            if let Ok(meta) =
                parse_conversation_meta(&file_path, &session_id, &project_name)
            {
                if !meta.first_message_preview.is_empty() {
                    conversations.push(meta);
                }
            }
        }
    }

    conversations.sort_by(|a, b| b.timestamp.cmp(&a.timestamp));
    Ok(conversations)
}

fn parse_conversation_meta(
    file_path: &Path,
    session_id: &str,
    project: &str,
) -> Result<ConversationMeta, String> {
    let file = fs::File::open(file_path).map_err(|e| e.to_string())?;
    let reader = BufReader::new(file);

    let mut first_message = String::new();
    let mut first_timestamp = String::new();
    let mut message_count: u32 = 0;

    for line in reader.lines() {
        let line = line.map_err(|e| e.to_string())?;
        if line.trim().is_empty() {
            continue;
        }

        let parsed: Value = match serde_json::from_str(&line) {
            Ok(v) => v,
            Err(_) => continue,
        };

        let msg_type = parsed.get("type").and_then(|t| t.as_str()).unwrap_or("");

        if msg_type == "user" || msg_type == "assistant" {
            message_count += 1;

            if first_message.is_empty() && msg_type == "user" {
                if let Some(message) = parsed.get("message") {
                    if let Some(content) = message.get("content") {
                        first_message = utils::extract_text_content(content);
                    }
                }
                if let Some(ts) = parsed.get("timestamp").and_then(|t| t.as_str()) {
                    first_timestamp = ts.to_string();
                }
            }
        }
    }

    Ok(ConversationMeta {
        session_id: session_id.to_string(),
        project: project.to_string(),
        first_message_preview: first_message,
        timestamp: first_timestamp,
        message_count,
        file_path: file_path.to_string_lossy().to_string(),
    })
}

#[tauri::command]
pub fn read_conversation(session_path: String) -> Result<Vec<ConversationMessage>, String> {
    let path = PathBuf::from(&session_path);
    if !path.exists() {
        return Err(format!("File not found: {}", session_path));
    }

    let file = fs::File::open(&path).map_err(|e| e.to_string())?;
    let reader = BufReader::new(file);
    let mut messages = Vec::new();

    for line in reader.lines() {
        let line = line.map_err(|e| e.to_string())?;
        if line.trim().is_empty() {
            continue;
        }

        let parsed: Value = match serde_json::from_str(&line) {
            Ok(v) => v,
            Err(_) => continue,
        };

        let msg_type = parsed
            .get("type")
            .and_then(|t| t.as_str())
            .unwrap_or("")
            .to_string();

        if msg_type != "user" && msg_type != "assistant" {
            continue;
        }

        let role = parsed
            .get("message")
            .and_then(|m| m.get("role"))
            .and_then(|r| r.as_str())
            .unwrap_or(&msg_type)
            .to_string();

        let content = parsed
            .get("message")
            .and_then(|m| m.get("content"))
            .map(|c| utils::extract_text_content(c))
            .unwrap_or_default();

        if content.is_empty() {
            continue;
        }

        let timestamp = parsed
            .get("timestamp")
            .and_then(|t| t.as_str())
            .unwrap_or("")
            .to_string();

        messages.push(ConversationMessage {
            role,
            content,
            timestamp,
            message_type: msg_type,
        });
    }

    Ok(messages)
}

#[tauri::command]
pub fn search_conversations(query: String, max_results: Option<u32>) -> Result<Vec<SearchResult>, String> {
    let project_dirs = utils::list_project_dirs()?;

    let max = max_results.unwrap_or(50) as usize;
    let query_lower = query.to_lowercase();
    let mut results = Vec::new();

    'outer: for project_entry in project_dirs {
        let project_name = project_entry.file_name().to_string_lossy().to_string();
        let project_path = project_entry.path();

        let jsonl_files = utils::list_jsonl_files(&project_path)?;

        for file_entry in jsonl_files {
            let file_path = file_entry.path();
            let file = match fs::File::open(&file_path) {
                Ok(f) => f,
                Err(_) => continue,
            };
            let reader = BufReader::new(file);

            for line in reader.lines() {
                let line = match line {
                    Ok(l) => l,
                    Err(_) => continue,
                };

                if !line.to_lowercase().contains(&query_lower) {
                    continue;
                }

                let parsed: Value = match serde_json::from_str(&line) {
                    Ok(v) => v,
                    Err(_) => continue,
                };

                let msg_type = parsed.get("type").and_then(|t| t.as_str()).unwrap_or("");
                if msg_type != "user" && msg_type != "assistant" {
                    continue;
                }

                let content = parsed
                    .get("message")
                    .and_then(|m| m.get("content"))
                    .map(|c| utils::extract_text_content(c))
                    .unwrap_or_default();

                let timestamp = parsed
                    .get("timestamp")
                    .and_then(|t| t.as_str())
                    .unwrap_or("")
                    .to_string();

                results.push(SearchResult {
                    session_path: file_path.to_string_lossy().to_string(),
                    project: project_name.clone(),
                    matched_line: content,
                    timestamp,
                });

                if results.len() >= max {
                    break 'outer;
                }
            }
        }
    }

    Ok(results)
}

fn remove_session(path: &Path) -> Result<(), String> {
    if path.exists() {
        fs::remove_file(path).map_err(|e| e.to_string())?;
    }
    let session_dir = path.with_extension("");
    if session_dir.is_dir() {
        fs::remove_dir_all(&session_dir).map_err(|e| e.to_string())?;
    }
    Ok(())
}

#[tauri::command]
pub fn delete_conversation(session_path: String) -> Result<(), String> {
    remove_session(&PathBuf::from(&session_path))
}

#[tauri::command]
pub fn read_command_history(limit: Option<u32>) -> Result<Vec<HistoryEntry>, String> {
    let path = utils::claude_dir().join("history.jsonl");
    if !path.exists() {
        return Ok(Vec::new());
    }

    let file = fs::File::open(&path).map_err(|e| e.to_string())?;
    let reader = BufReader::new(file);
    let max = limit.unwrap_or(100) as usize;

    let mut entries: Vec<HistoryEntry> = reader
        .lines()
        .filter_map(|line| line.ok())
        .filter(|line| !line.trim().is_empty())
        .filter_map(|line| serde_json::from_str::<HistoryEntry>(&line).ok())
        .collect();

    entries.sort_by(|a, b| b.timestamp.cmp(&a.timestamp));
    entries.truncate(max);

    Ok(entries)
}

#[tauri::command]
pub fn delete_command_entry(timestamp: u64) -> Result<(), String> {
    let path = utils::claude_dir().join("history.jsonl");
    if !path.exists() {
        return Ok(());
    }

    let file = fs::File::open(&path).map_err(|e| e.to_string())?;
    let reader = BufReader::new(file);

    let remaining: Vec<String> = reader
        .lines()
        .filter_map(|line| line.ok())
        .filter(|line| {
            if line.trim().is_empty() {
                return false;
            }
            match serde_json::from_str::<HistoryEntry>(line) {
                Ok(entry) => entry.timestamp != timestamp,
                Err(_) => true,
            }
        })
        .collect();

    let mut file = fs::File::create(&path).map_err(|e| e.to_string())?;
    for line in &remaining {
        writeln!(file, "{}", line).map_err(|e| e.to_string())?;
    }

    Ok(())
}

#[tauri::command]
pub fn clear_command_history() -> Result<(), String> {
    let path = utils::claude_dir().join("history.jsonl");
    if path.exists() {
        fs::write(&path, "").map_err(|e| e.to_string())?;
    }
    Ok(())
}

#[tauri::command]
pub fn clear_all_conversations(project_filter: Option<String>) -> Result<u32, String> {
    let project_dirs = utils::list_project_dirs()?;
    let mut count: u32 = 0;

    for project_entry in project_dirs {
        let project_name = project_entry.file_name().to_string_lossy().to_string();

        if let Some(ref filter) = project_filter {
            if project_name != *filter {
                continue;
            }
        }

        let project_path = project_entry.path();
        let jsonl_files = utils::list_jsonl_files(&project_path)?;

        for file_entry in jsonl_files {
            remove_session(&file_entry.path())?;
            count += 1;
        }
    }

    Ok(count)
}
