import { invoke } from "$lib/tauri";

export async function readClaudeMd(
  scope: string,
): Promise<string | null> {
  return invoke<string | null>("read_claude_md", { scope });
}

export async function writeClaudeMd(
  scope: string,
  content: string,
): Promise<void> {
  return invoke<void>("write_claude_md", { scope, content });
}
