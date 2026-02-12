import { invoke } from "$lib/tauri";

export interface ConversationMeta {
  session_id: string;
  project: string;
  first_message_preview: string;
  timestamp: string;
  message_count: number;
  file_path: string;
}

export interface ConversationMessage {
  role: string;
  content: string;
  timestamp: string;
  message_type: string;
}

export interface SearchResult {
  session_path: string;
  project: string;
  matched_line: string;
  timestamp: string;
}

export interface HistoryEntry {
  display: string;
  project: string;
  timestamp: number;
}

export async function listConversations(
  projectFilter?: string,
): Promise<ConversationMeta[]> {
  return invoke<ConversationMeta[]>("list_conversations", { projectFilter });
}

export async function readConversation(
  sessionPath: string,
): Promise<ConversationMessage[]> {
  return invoke<ConversationMessage[]>("read_conversation", { sessionPath });
}

export async function searchConversations(
  query: string,
  maxResults?: number,
): Promise<SearchResult[]> {
  return invoke<SearchResult[]>("search_conversations", { query, maxResults });
}

export async function deleteConversation(
  sessionPath: string,
): Promise<void> {
  return invoke<void>("delete_conversation", { sessionPath });
}

export async function readCommandHistory(
  limit?: number,
): Promise<HistoryEntry[]> {
  return invoke<HistoryEntry[]>("read_command_history", { limit });
}
