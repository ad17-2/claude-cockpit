import { invoke } from "$lib/tauri";

export interface StatsCache {
  daily_stats?: Record<string, DayStat>;
  model_usage?: Record<string, ModelUsage>;
  hourly_activity?: number[];
  total_sessions?: number;
  total_messages?: number;
  total_tokens?: number;
  days_active?: number;
  longest_session?: {
    duration_mins: number;
    message_count: number;
    date: string;
  };
}

export interface DayStat {
  messages: number;
  tokens: number;
  sessions: number;
}

export interface ModelUsage {
  input_tokens: number;
  output_tokens: number;
  cache_read_tokens: number;
  cache_creation_tokens: number;
}

export async function readStatsCache(): Promise<StatsCache | null> {
  return invoke<StatsCache | null>("read_stats_cache");
}
