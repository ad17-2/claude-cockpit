import { invoke } from "$lib/tauri";

export type EntityType = "agents" | "rules" | "commands" | "skills" | "hooks";

export interface EntityInfo {
  name: string;
  entity_type: string;
  scope: string;
  description: string;
  file_path: string;
}

export interface EntityDetail {
  frontmatter: Record<string, string>;
  body: string;
  file_path: string;
}

export async function listEntities(
  entityType: EntityType,
): Promise<EntityInfo[]> {
  return invoke<EntityInfo[]>("list_entities", { entityType });
}

export async function readEntity(
  entityType: EntityType,
  scope: string,
  name: string,
): Promise<EntityDetail> {
  return invoke<EntityDetail>("read_entity", { entityType, scope, name });
}

export async function writeEntity(
  entityType: EntityType,
  scope: string,
  name: string,
  content: string,
): Promise<void> {
  return invoke<void>("write_entity", { entityType, scope, name, content });
}

export async function deleteEntity(
  entityType: EntityType,
  scope: string,
  name: string,
): Promise<void> {
  return invoke<void>("delete_entity", { entityType, scope, name });
}
