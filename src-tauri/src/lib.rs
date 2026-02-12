mod commands;
mod tray;

use commands::{claude_md, entities, history, mcp, projects, sessions, settings, usage, watcher};

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_notification::init())
        .invoke_handler(tauri::generate_handler![
            projects::list_projects,
            projects::decode_project_path,
            projects::delete_project,
            claude_md::read_claude_md,
            claude_md::write_claude_md,
            settings::read_settings,
            settings::write_settings,
            settings::get_effective_settings,
            settings::add_permission,
            settings::remove_permission,
            entities::list_entities,
            entities::read_entity,
            entities::write_entity,
            entities::delete_entity,
            history::list_conversations,
            history::read_conversation,
            history::search_conversations,
            history::delete_conversation,
            history::read_command_history,
            history::delete_command_entry,
            history::clear_command_history,
            history::clear_all_conversations,
            watcher::start_watching,
            sessions::list_active_sessions,
            sessions::tail_session,
            usage::read_stats_cache,
            mcp::list_mcp_servers,
        ])
        .setup(|app| {
            tray::setup(app)?;
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
