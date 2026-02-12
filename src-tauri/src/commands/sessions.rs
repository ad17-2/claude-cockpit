use serde::Serialize;
use serde_json::Value;
use std::fs;
use std::io::{BufRead, BufReader};
use std::path::PathBuf;
use std::time::{SystemTime, UNIX_EPOCH};

use super::utils;

#[derive(Debug, Serialize)]
pub struct ActiveSession {
    pub session_id: String,
    pub project: String,
    pub file_path: String,
    pub last_modified: u64,
    pub message_count: u32,
    pub last_message_preview: String,
    pub model: String,
}

#[derive(Debug, Serialize)]
pub struct TailMessage {
    pub role: String,
    pub content: String,
    pub timestamp: String,
    pub model: String,
    pub tokens_in: u64,
    pub tokens_out: u64,
}

#[derive(Debug, Serialize)]
pub struct TailResult {
    pub messages: Vec<TailMessage>,
    pub total_lines: u32,
}

#[tauri::command]
pub fn list_active_sessions(threshold_secs: Option<u64>) -> Result<Vec<ActiveSession>, String> {
    let project_dirs = utils::list_project_dirs()?;

    let threshold = threshold_secs.unwrap_or(300);
    let now = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map_err(|e| e.to_string())?
        .as_secs();

    let mut sessions = Vec::new();

    for project_entry in project_dirs {
        let project_name_encoded = project_entry.file_name().to_string_lossy().to_string();
        let project_name = utils::decode_project_name(&project_name_encoded);
        let project_path = project_entry.path();

        let jsonl_files: Vec<_> = fs::read_dir(&project_path)
            .map_err(|e| e.to_string())?
            .filter_map(|e| e.ok())
            .filter(|e| {
                e.path()
                    .extension()
                    .and_then(|ext| ext.to_str())
                    == Some("jsonl")
            })
            .collect();

        for file_entry in jsonl_files {
            let file_path = file_entry.path();
            let metadata = match fs::metadata(&file_path) {
                Ok(m) => m,
                Err(_) => continue,
            };

            let modified = metadata
                .modified()
                .ok()
                .and_then(|t| t.duration_since(UNIX_EPOCH).ok())
                .map(|d| d.as_secs())
                .unwrap_or(0);

            if now - modified > threshold {
                continue;
            }

            let session_id = file_path
                .file_stem()
                .and_then(|s| s.to_str())
                .unwrap_or("")
                .to_string();

            let file = match fs::File::open(&file_path) {
                Ok(f) => f,
                Err(_) => continue,
            };
            let reader = BufReader::new(file);

            let mut message_count: u32 = 0;
            let mut last_message_preview = String::new();
            let mut model = String::new();

            for line in reader.lines() {
                let line = match line {
                    Ok(l) => l,
                    Err(_) => continue,
                };
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

                    if let Some(message) = parsed.get("message") {
                        if let Some(content) = message.get("content") {
                            let text = utils::extract_text_content(content);
                            if !text.is_empty() {
                                last_message_preview = text;
                            }
                        }
                        if let Some(m) = message.get("model").and_then(|m| m.as_str()) {
                            model = m.to_string();
                        }
                    }
                }
            }

            if message_count == 0 {
                continue;
            }

            sessions.push(ActiveSession {
                session_id,
                project: project_name.clone(),
                file_path: file_path.to_string_lossy().to_string(),
                last_modified: modified * 1000,
                message_count,
                last_message_preview,
                model,
            });
        }
    }

    sessions.sort_by(|a, b| b.last_modified.cmp(&a.last_modified));
    Ok(sessions)
}

#[tauri::command]
pub fn tail_session(file_path: String, from_line: u32) -> Result<TailResult, String> {
    let path = PathBuf::from(&file_path);
    if !path.exists() {
        return Err(format!("File not found: {}", file_path));
    }

    let file = fs::File::open(&path).map_err(|e| e.to_string())?;
    let reader = BufReader::new(file);
    let mut messages = Vec::new();
    let mut total_lines: u32 = 0;

    for line in reader.lines() {
        let line = match line {
            Ok(l) => l,
            Err(_) => continue,
        };
        if line.trim().is_empty() {
            continue;
        }

        total_lines += 1;
        if total_lines <= from_line {
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

        let role = parsed
            .get("message")
            .and_then(|m| m.get("role"))
            .and_then(|r| r.as_str())
            .unwrap_or(msg_type)
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

        let model = parsed
            .get("message")
            .and_then(|m| m.get("model"))
            .and_then(|m| m.as_str())
            .unwrap_or("")
            .to_string();

        let usage = parsed.get("message").and_then(|m| m.get("usage"));
        let tokens_in = usage
            .and_then(|u| u.get("input_tokens"))
            .and_then(|t| t.as_u64())
            .unwrap_or(0);
        let tokens_out = usage
            .and_then(|u| u.get("output_tokens"))
            .and_then(|t| t.as_u64())
            .unwrap_or(0);

        messages.push(TailMessage {
            role,
            content,
            timestamp,
            model,
            tokens_in,
            tokens_out,
        });
    }

    Ok(TailResult {
        messages,
        total_lines,
    })
}
