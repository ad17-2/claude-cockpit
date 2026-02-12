<script lang="ts">
  import { onMount } from "svelte";
  import {
    listConversations,
    searchConversations,
    deleteConversation,
    readConversation,
    readCommandHistory,
    type ConversationMeta,
    type ConversationMessage,
    type SearchResult,
    type HistoryEntry,
  } from "$lib/commands/history";
  import { listProjects, type ProjectInfo } from "$lib/commands/projects";
  import { onFileChange } from "$lib/commands/watcher";
  import { Search, Trash2, X, ChevronRight, ChevronDown, Terminal } from "lucide-svelte";

  let projects = $state<ProjectInfo[]>([]);
  let conversations = $state<ConversationMeta[]>([]);
  let loading = $state(true);
  let error = $state<string | null>(null);

  let projectFilter = $state<string | null>(null);
  let searchQuery = $state("");
  let searchResults = $state<SearchResult[]>([]);
  let searching = $state(false);

  let activeTab = $state<"conversations" | "commands">("conversations");
  let commandHistory = $state<HistoryEntry[]>([]);
  let commandsLoading = $state(false);

  let expandedSessions = $state(new Set<string>());
  let sessionMessages = $state(new Map<string, ConversationMessage[]>());

  let isSearchMode = $derived(searchQuery.trim().length > 0);

  async function loadConversations(): Promise<void> {
    loading = true;
    error = null;
    try {
      conversations = await listConversations(projectFilter ?? undefined);
    } catch (e) {
      error = String(e);
      conversations = [];
    } finally {
      loading = false;
    }
  }

  async function loadCommandHistory(): Promise<void> {
    commandsLoading = true;
    error = null;
    try {
      commandHistory = await readCommandHistory(200);
    } catch (e) {
      error = String(e);
      commandHistory = [];
    } finally {
      commandsLoading = false;
    }
  }

  async function handleTabChange(tab: "conversations" | "commands"): Promise<void> {
    activeTab = tab;
    if (tab === "commands" && commandHistory.length === 0) {
      await loadCommandHistory();
    }
  }

  async function handleSearch(): Promise<void> {
    const q = searchQuery.trim();
    if (!q) {
      searchResults = [];
      return;
    }
    searching = true;
    try {
      searchResults = await searchConversations(q, 50);
    } catch (e) {
      error = String(e);
      searchResults = [];
    } finally {
      searching = false;
    }
  }

  async function handleDelete(filePath: string): Promise<void> {
    try {
      await deleteConversation(filePath);
      await loadConversations();
    } catch (e) {
      error = String(e);
    }
  }

  async function toggleSession(conv: ConversationMeta): Promise<void> {
    const key = conv.file_path;
    if (expandedSessions.has(key)) {
      expandedSessions.delete(key);
      expandedSessions = new Set(expandedSessions);
    } else {
      if (!sessionMessages.has(key)) {
        try {
          const msgs = await readConversation(conv.file_path);
          sessionMessages.set(key, msgs);
          sessionMessages = new Map(sessionMessages);
        } catch (e) {
          error = String(e);
          return;
        }
      }
      expandedSessions.add(key);
      expandedSessions = new Set(expandedSessions);
    }
  }

  function getPreviewMessages(messages: ConversationMessage[]): { first: ConversationMessage[]; last: ConversationMessage[]; hiddenCount: number } {
    if (messages.length <= 6) {
      return { first: messages, last: [], hiddenCount: 0 };
    }
    return {
      first: messages.slice(0, 3),
      last: messages.slice(-3),
      hiddenCount: messages.length - 6,
    };
  }

  function handleProjectFilter(encoded: string | null): void {
    projectFilter = encoded;
    loadConversations();
  }

  function clearSearch(): void {
    searchQuery = "";
    searchResults = [];
  }

  function formatTimestamp(ts: string): string {
    if (!ts) return "";
    try {
      const d = new Date(ts);
      return d.toLocaleDateString(undefined, {
        month: "short",
        day: "numeric",
        hour: "2-digit",
        minute: "2-digit",
      });
    } catch {
      return ts;
    }
  }

  function formatEpoch(ts: number): string {
    if (!ts) return "";
    try {
      const d = new Date(ts);
      return d.toLocaleDateString(undefined, {
        month: "short",
        day: "numeric",
        hour: "2-digit",
        minute: "2-digit",
      });
    } catch {
      return String(ts);
    }
  }

  function decodeProject(encoded: string): string {
    if (!encoded || !encoded.startsWith("-")) return encoded;
    return encoded.substring(1).replace(/-/g, "/");
  }

  let searchTimeout: ReturnType<typeof setTimeout> | undefined;
  function handleSearchInput(): void {
    clearTimeout(searchTimeout);
    searchTimeout = setTimeout(handleSearch, 300);
  }

  onMount(() => {
    let unlisten: (() => void) | undefined;

    (async () => {
      try {
        projects = await listProjects();
      } catch {}
      await loadConversations();

      unlisten = await onFileChange("history-changed", () => {
        if (activeTab === "conversations") {
          loadConversations();
        } else {
          loadCommandHistory();
        }
      });
    })();

    return () => unlisten?.();
  });
</script>

<div class="flex h-full flex-col">
  <div class="border-b border-border-primary px-4 py-2">
    <h1 class="text-xs font-medium text-text-secondary">// history</h1>
  </div>

  <div class="flex items-center gap-1 border-b border-border-primary px-3 py-1.5">
    <button
      onclick={() => handleTabChange("conversations")}
      class="px-2 py-1 text-xs transition-colors
        {activeTab === 'conversations' ? 'text-accent' : 'text-text-secondary hover:text-text-primary'}"
    >
      [conversations]
    </button>
    <button
      onclick={() => handleTabChange("commands")}
      class="px-2 py-1 text-xs transition-colors
        {activeTab === 'commands' ? 'text-accent' : 'text-text-secondary hover:text-text-primary'}"
    >
      [commands]
    </button>
  </div>

  {#if activeTab === "conversations"}
    <div class="flex items-center gap-2 border-b border-border-primary px-3 py-1.5">
      <div class="relative flex-1">
        <Search size={12} class="absolute left-2 top-1/2 -translate-y-1/2 text-text-tertiary" />
        <input
          type="text"
          bind:value={searchQuery}
          oninput={handleSearchInput}
          placeholder="search conversations..."
          class="w-full border border-border-primary bg-bg-tertiary py-1.5 pl-7 pr-7 text-xs text-text-primary placeholder-text-tertiary outline-none focus:border-border-focus"
        />
        {#if searchQuery}
          <button
            onclick={clearSearch}
            class="absolute right-2 top-1/2 -translate-y-1/2 text-text-tertiary hover:text-text-primary"
          >
            <X size={12} />
          </button>
        {/if}
      </div>
    </div>

    <div class="flex items-center gap-1 overflow-x-auto border-b border-border-primary px-3 py-1.5">
      <button
        onclick={() => handleProjectFilter(null)}
        class="shrink-0 px-2 py-1 text-xs transition-colors
          {projectFilter === null ? 'text-accent' : 'text-text-secondary hover:text-text-primary'}"
      >
        [all]
      </button>
      {#each projects as project}
        <button
          onclick={() => handleProjectFilter(project.encoded_path)}
          class="shrink-0 px-2 py-1 text-xs transition-colors
            {projectFilter === project.encoded_path ? 'text-accent' : 'text-text-secondary hover:text-text-primary'}"
        >
          [{project.name}]
        </button>
      {/each}
    </div>

    {#if error}
      <div class="px-4 py-2">
        <p class="text-xs text-danger">{error}</p>
      </div>
    {/if}

    <div class="flex-1 overflow-y-auto">
      {#if isSearchMode}
        {#if searching}
          <div class="flex h-full items-center justify-center">
            <p class="text-xs text-text-tertiary">searching...</p>
          </div>
        {:else if searchResults.length === 0}
          <div class="flex h-full items-center justify-center">
            <p class="text-xs text-text-tertiary">// no results found</p>
          </div>
        {:else}
          <div class="space-y-px p-3">
            {#each searchResults as result}
              <div
                class="flex w-full items-start gap-2.5 border border-border-primary bg-bg-secondary px-3 py-2 transition-colors hover:bg-bg-hover"
              >
                <span class="mt-0.5 text-[10px] text-accent">></span>
                <div class="min-w-0 flex-1">
                  <p class="truncate text-xs text-text-primary">{result.matched_line}</p>
                  <div class="mt-0.5 flex items-center gap-2">
                    <span class="text-[10px] text-text-tertiary">{decodeProject(result.project)}</span>
                    {#if result.timestamp}
                      <span class="text-[10px] text-text-tertiary">{formatTimestamp(result.timestamp)}</span>
                    {/if}
                  </div>
                </div>
                <button
                  onclick={() => handleDelete(result.session_path)}
                  class="shrink-0 p-1 text-text-tertiary transition-colors hover:text-danger"
                >
                  <Trash2 size={12} />
                </button>
              </div>
            {/each}
          </div>
        {/if}
      {:else if loading}
        <div class="flex h-full items-center justify-center">
          <p class="text-xs text-text-tertiary">loading...</p>
        </div>
      {:else if conversations.length === 0}
        <div class="flex h-full items-center justify-center">
          <p class="text-xs text-text-tertiary">// no conversations found</p>
        </div>
      {:else}
        <div class="space-y-px p-3">
          {#each conversations as conv}
            <div>
              <div
                role="button"
                tabindex="0"
                onclick={() => toggleSession(conv)}
                onkeydown={(e) => { if (e.key === 'Enter' || e.key === ' ') toggleSession(conv); }}
                class="flex w-full cursor-pointer items-start gap-2.5 border border-border-primary bg-bg-secondary px-3 py-2 text-left transition-colors hover:bg-bg-hover"
              >
                <span class="mt-0.5 shrink-0">
                  {#if expandedSessions.has(conv.file_path)}
                    <ChevronDown size={12} class="text-accent" />
                  {:else}
                    <ChevronRight size={12} class="text-text-tertiary" />
                  {/if}
                </span>
                <div class="min-w-0 flex-1">
                  <p class="truncate text-xs text-text-primary">{conv.first_message_preview}</p>
                  <div class="mt-0.5 flex items-center gap-2">
                    <span class="text-[10px] text-text-tertiary">{decodeProject(conv.project)}</span>
                    <span class="text-[10px] text-text-tertiary">{conv.message_count} msgs</span>
                    {#if conv.timestamp}
                      <span class="text-[10px] text-text-tertiary">{formatTimestamp(conv.timestamp)}</span>
                    {/if}
                  </div>
                </div>
                <button
                  onclick={(e) => { e.stopPropagation(); handleDelete(conv.file_path); }}
                  class="shrink-0 p-1 text-text-tertiary transition-colors hover:text-danger"
                >
                  <Trash2 size={12} />
                </button>
              </div>

              {#if expandedSessions.has(conv.file_path) && sessionMessages.has(conv.file_path)}
                {@const messages = sessionMessages.get(conv.file_path) ?? []}
                {@const preview = getPreviewMessages(messages)}
                <div class="ml-4 space-y-px border-l-2 border-accent py-1">
                  {#each preview.first as msg}
                    <div class="px-3 py-1">
                      <div class="flex items-baseline gap-2">
                        <span class="shrink-0 text-[10px] font-medium {msg.role === 'user' ? 'text-accent' : 'text-text-secondary'}">[{msg.role}]</span>
                        <p class="truncate text-[11px] text-text-primary">{msg.content}</p>
                      </div>
                      {#if msg.timestamp}
                        <p class="mt-0.5 pl-12 text-[10px] text-text-tertiary">{formatTimestamp(msg.timestamp)}</p>
                      {/if}
                    </div>
                  {/each}
                  {#if preview.hiddenCount > 0}
                    <div class="px-3 py-1">
                      <p class="text-[10px] text-text-tertiary">... {preview.hiddenCount} more messages ...</p>
                    </div>
                  {/if}
                  {#each preview.last as msg}
                    <div class="px-3 py-1">
                      <div class="flex items-baseline gap-2">
                        <span class="shrink-0 text-[10px] font-medium {msg.role === 'user' ? 'text-accent' : 'text-text-secondary'}">[{msg.role}]</span>
                        <p class="truncate text-[11px] text-text-primary">{msg.content}</p>
                      </div>
                      {#if msg.timestamp}
                        <p class="mt-0.5 pl-12 text-[10px] text-text-tertiary">{formatTimestamp(msg.timestamp)}</p>
                      {/if}
                    </div>
                  {/each}
                </div>
              {/if}
            </div>
          {/each}
        </div>
      {/if}
    </div>
  {:else}
    {#if error}
      <div class="px-4 py-2">
        <p class="text-xs text-danger">{error}</p>
      </div>
    {/if}

    <div class="flex-1 overflow-y-auto">
      {#if commandsLoading}
        <div class="flex h-full items-center justify-center">
          <p class="text-xs text-text-tertiary">loading...</p>
        </div>
      {:else if commandHistory.length === 0}
        <div class="flex h-full items-center justify-center">
          <p class="text-xs text-text-tertiary">// no command history found</p>
        </div>
      {:else}
        <div class="space-y-px p-3">
          {#each commandHistory as entry}
            <div class="flex w-full items-start gap-2.5 border border-border-primary bg-bg-secondary px-3 py-2">
              <Terminal size={12} class="mt-0.5 shrink-0 text-text-tertiary" />
              <div class="min-w-0 flex-1">
                <p class="truncate text-xs text-text-primary">{entry.display}</p>
                <div class="mt-0.5 flex items-center gap-2">
                  {#if entry.project}
                    <span class="text-[10px] text-text-tertiary">{entry.project}</span>
                  {/if}
                  {#if entry.timestamp}
                    <span class="text-[10px] text-text-tertiary">{formatEpoch(entry.timestamp)}</span>
                  {/if}
                </div>
              </div>
            </div>
          {/each}
        </div>
      {/if}
    </div>
  {/if}
</div>
