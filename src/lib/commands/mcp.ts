import { invoke } from "$lib/tauri";

export interface McpServerInfo {
  name: string;
  command: string | null;
  args: string[];
  url: string | null;
  env_keys: string[];
  scope: string;
}

export async function listMcpServers(): Promise<McpServerInfo[]> {
  return invoke<McpServerInfo[]>("list_mcp_servers");
}
