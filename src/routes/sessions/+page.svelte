<script lang="ts">
  import { onMount } from "svelte";
  import {
    listActiveSessions,
    tailSession,
    type ActiveSession,
    type TailMessage,
    type TailResult,
  } from "$lib/commands/sessions";
  import { ChevronRight, ChevronDown, Activity } from "lucide-svelte";

  let sessions = $state<ActiveSession[]>([]);
  let loading = $state(true);
  let error = $state<string | null>(null);

  let expandedSession = $state<string | null>(null);
  let tailMessages = $state<TailMessage[]>([]);
  let tailFromLine = $state(0);
  let tailLoading = $state(false);

  let pollInterval: ReturnType<typeof setInterval> | undefined;
  let tailPollInterval: ReturnType<typeof setInterval> | undefined;

  async function loadSessions(): Promise<void> {
    try {
      sessions = await listActiveSessions();
    } catch (e) {
      error = String(e);
    } finally {
      loading = false;
    }
  }

  async function toggleSession(session: ActiveSession): Promise<void> {
    if (expandedSession === session.file_path) {
      expandedSession = null;
      tailMessages = [];
      tailFromLine = 0;
      clearInterval(tailPollInterval);
      return;
    }

    expandedSession = session.file_path;
    tailMessages = [];
    tailFromLine = 0;
    tailLoading = true;

    try {
      const result = await tailSession(session.file_path, 0);
      const lastN = result.messages.slice(-20);
      tailMessages = lastN;
      tailFromLine = result.total_lines;
    } catch (e) {
      error = String(e);
    } finally {
      tailLoading = false;
    }

    clearInterval(tailPollInterval);
    tailPollInterval = setInterval(async () => {
      if (!expandedSession) return;
      try {
        const result = await tailSession(expandedSession, tailFromLine);
        if (result.messages.length > 0) {
          tailMessages = [...tailMessages, ...result.messages];
          tailFromLine = result.total_lines;
        }
      } catch {
        // silent
      }
    }, 2000);
  }

  function formatAge(lastModified: number): string {
    const secs = Math.floor((Date.now() - lastModified) / 1000);
    if (secs < 60) return `${secs}s ago`;
    const mins = Math.floor(secs / 60);
    if (mins < 60) return `${mins}m ago`;
    const hours = Math.floor(mins / 60);
    return `${hours}h ago`;
  }

  function isRecentlyActive(lastModified: number): boolean {
    return Date.now() - lastModified < 30000;
  }

  function formatTimestamp(ts: string): string {
    if (!ts) return "";
    try {
      return new Date(ts).toLocaleTimeString(undefined, {
        hour: "2-digit",
        minute: "2-digit",
        second: "2-digit",
      });
    } catch {
      return ts;
    }
  }

  onMount(() => {
    loadSessions();
    pollInterval = setInterval(loadSessions, 3000);
    return () => {
      clearInterval(pollInterval);
      clearInterval(tailPollInterval);
    };
  });
</script>

<div class="flex h-full flex-col">
  <div class="border-b border-border-primary px-4 py-2">
    <h1 class="text-xs font-medium text-text-secondary">// sessions</h1>
  </div>

  {#if error}
    <div class="px-4 py-2">
      <p class="text-xs text-danger">{error}</p>
    </div>
  {/if}

  <div class="flex-1 overflow-y-auto">
    {#if loading}
      <div class="flex h-full items-center justify-center">
        <p class="text-xs text-text-tertiary">loading...</p>
      </div>
    {:else if sessions.length === 0}
      <div class="flex h-full items-center justify-center">
        <div class="text-center">
          <Activity size={24} class="mx-auto mb-2 text-text-tertiary" />
          <p class="text-xs text-text-tertiary">// no active sessions</p>
          <p class="mt-1 text-[10px] text-text-tertiary">sessions modified in the last 5 minutes will appear here</p>
        </div>
      </div>
    {:else}
      <div class="space-y-px p-3">
        {#each sessions as session}
          <div>
            <button
              onclick={() => toggleSession(session)}
              class="flex w-full items-start gap-2.5 border border-border-primary bg-bg-secondary px-3 py-2 text-left transition-colors hover:bg-bg-hover"
            >
              <span class="mt-0.5 shrink-0">
                {#if expandedSession === session.file_path}
                  <ChevronDown size={12} class="text-accent" />
                {:else}
                  <ChevronRight size={12} class="text-text-tertiary" />
                {/if}
              </span>
              <div class="min-w-0 flex-1">
                <div class="flex items-center gap-2">
                  {#if isRecentlyActive(session.last_modified)}
                    <span class="h-1.5 w-1.5 rounded-full bg-success animate-pulse"></span>
                  {/if}
                  <p class="text-xs text-text-primary">{session.project}</p>
                  <span class="text-[10px] text-text-tertiary">{formatAge(session.last_modified)}</span>
                </div>
                <div class="mt-0.5 flex items-center gap-2">
                  <span class="text-[10px] text-text-tertiary">{session.message_count} msgs</span>
                  {#if session.model}
                    <span class="text-[10px] text-text-tertiary">{session.model}</span>
                  {/if}
                </div>
                <p class="mt-0.5 truncate text-[11px] text-text-secondary">{session.last_message_preview}</p>
              </div>
            </button>

            {#if expandedSession === session.file_path}
              <div class="border border-t-0 border-border-primary bg-bg-tertiary">
                {#if tailLoading}
                  <div class="flex items-center justify-center py-4">
                    <p class="text-xs text-text-tertiary">loading messages...</p>
                  </div>
                {:else if tailMessages.length === 0}
                  <div class="flex items-center justify-center py-4">
                    <p class="text-xs text-text-tertiary">// no messages</p>
                  </div>
                {:else}
                  <div class="max-h-96 overflow-y-auto p-2 space-y-1">
                    {#each tailMessages as msg}
                      <div class="px-2 py-1">
                        <div class="flex items-baseline gap-2">
                          <span class="shrink-0 text-[10px] font-medium {msg.role === 'user' ? 'text-accent' : 'text-text-secondary'}">[{msg.role}]</span>
                          <span class="text-[10px] text-text-tertiary">{formatTimestamp(msg.timestamp)}</span>
                          {#if msg.tokens_in || msg.tokens_out}
                            <span class="text-[10px] text-text-tertiary">{msg.tokens_in} {msg.tokens_out}</span>
                          {/if}
                        </div>
                        <p class="mt-0.5 whitespace-pre-wrap break-words pl-2 text-[11px] text-text-primary">{msg.content}</p>
                      </div>
                    {/each}
                  </div>
                {/if}
              </div>
            {/if}
          </div>
        {/each}
      </div>
    {/if}
  </div>
</div>
