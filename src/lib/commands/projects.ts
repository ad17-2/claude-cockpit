import { invoke } from "$lib/tauri";

export interface ProjectInfo {
  encoded_path: string;
  decoded_path: string;
  name: string;
  has_claude_md: boolean;
  has_settings: boolean;
  is_root: boolean;
}

export async function listProjects(): Promise<ProjectInfo[]> {
  return invoke<ProjectInfo[]>("list_projects");
}

export async function decodeProjectPath(encoded: string): Promise<string> {
  return invoke<string>("decode_project_path", { encoded });
}

export async function deleteProject(encodedPath: string): Promise<void> {
  return invoke<void>("delete_project", { encodedPath });
}
