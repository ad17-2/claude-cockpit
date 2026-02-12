type InvokeFn = <T>(cmd: string, args?: Record<string, unknown>) => Promise<T>;

let _invoke: InvokeFn | null = null;

async function getInvoke(): Promise<InvokeFn> {
  if (_invoke) return _invoke;
  const mod = await import("@tauri-apps/api/core");
  _invoke = mod.invoke;
  return _invoke;
}

export function isTauri(): boolean {
  return typeof window !== "undefined" && "__TAURI_INTERNALS__" in window;
}

export async function invoke<T>(cmd: string, args?: Record<string, unknown>): Promise<T> {
  if (!isTauri()) {
    throw new Error(`Not running in Tauri — cannot invoke "${cmd}"`);
  }
  const fn = await getInvoke();
  return fn<T>(cmd, args);
}
