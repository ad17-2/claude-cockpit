import { invoke } from "$lib/tauri";

export type FileType = "settings" | "settings_local";
export type PermissionCategory = "allow" | "deny" | "ask";

export async function readSettings(
  scope: string,
  fileType: FileType,
): Promise<Record<string, unknown>> {
  return invoke<Record<string, unknown>>("read_settings", {
    scope,
    fileType,
  });
}

export async function writeSettings(
  scope: string,
  fileType: FileType,
  content: Record<string, unknown>,
): Promise<void> {
  return invoke<void>("write_settings", { scope, fileType, content });
}

export async function getEffectiveSettings(
  projectPath: string,
): Promise<Record<string, unknown>> {
  return invoke<Record<string, unknown>>("get_effective_settings", {
    projectPath,
  });
}

export async function addPermission(
  scope: string,
  fileType: FileType,
  category: PermissionCategory,
  permission: string,
): Promise<void> {
  return invoke<void>("add_permission", {
    scope,
    fileType,
    category,
    permission,
  });
}

export async function removePermission(
  scope: string,
  fileType: FileType,
  category: PermissionCategory,
  permission: string,
): Promise<void> {
  return invoke<void>("remove_permission", {
    scope,
    fileType,
    category,
    permission,
  });
}
