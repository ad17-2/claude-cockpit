import { invoke, isTauri } from "$lib/tauri";

export async function startWatching(): Promise<void> {
  return invoke<void>("start_watching");
}

export type WatchEvent =
  | "claude-md-changed"
  | "settings-changed"
  | "entity-changed"
  | "history-changed";

type UnlistenFn = () => void;

export async function onFileChange(
  event: WatchEvent,
  callback: () => void,
): Promise<UnlistenFn> {
  if (!isTauri()) return () => {};
  const { listen } = await import("@tauri-apps/api/event");
  return listen(event, callback);
}
