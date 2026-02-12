import { invoke } from "$lib/tauri";

export interface StatsCache {
  version: number;
  lastComputedDate: string;
  dailyActivity: DayActivity[];
  dailyModelTokens?: Record<string, unknown>;
  modelUsage: Record<string, ModelUsage>;
  totalSessions: number;
  totalMessages: number;
  longestSession: LongestSession | null;
  firstSessionDate: string;
  hourCounts: Record<string, number>;
  totalSpeculationTimeSavedMs?: number;
}

export interface DayActivity {
  date: string;
  messageCount: number;
  sessionCount: number;
  toolCallCount: number;
}

export interface ModelUsage {
  inputTokens: number;
  outputTokens: number;
  cacheReadInputTokens: number;
  cacheCreationInputTokens: number;
  webSearchRequests?: number;
  costUSD?: number;
  contextWindow?: number;
  maxOutputTokens?: number;
}

export interface LongestSession {
  sessionId: string;
  duration: number;
  messageCount: number;
  timestamp: string;
}

export async function readStatsCache(): Promise<StatsCache | null> {
  return invoke<StatsCache | null>("read_stats_cache");
}
