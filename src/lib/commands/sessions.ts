import { invoke } from "$lib/tauri";

export interface ActiveSession {
  session_id: string;
  project: string;
  file_path: string;
  last_modified: number;
  message_count: number;
  last_message_preview: string;
  model: string;
}

export interface TailMessage {
  role: string;
  content: string;
  timestamp: string;
  model: string;
  tokens_in: number;
  tokens_out: number;
}

export interface TailResult {
  messages: TailMessage[];
  total_lines: number;
}

export async function listActiveSessions(
  thresholdSecs?: number,
): Promise<ActiveSession[]> {
  return invoke<ActiveSession[]>("list_active_sessions", { thresholdSecs });
}

export async function tailSession(
  filePath: string,
  fromLine: number,
): Promise<TailResult> {
  return invoke<TailResult>("tail_session", { filePath, fromLine });
}
