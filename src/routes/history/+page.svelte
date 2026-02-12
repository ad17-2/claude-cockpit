<script lang="ts">
  import { onMount } from "svelte";
  import {
    listConversations,
    searchConversations,
    deleteConversation,
    type ConversationMeta,
    type SearchResult,
  } from "$lib/commands/history";
  import { listProjects, type ProjectInfo } from "$lib/commands/projects";
  import { Search, Trash2, X } from "lucide-svelte";

  let projects = $state<ProjectInfo[]>([]);
  let conversations = $state<ConversationMeta[]>([]);
  let loading = $state(true);
  let error = $state<string | null>(null);

  let projectFilter = $state<string | null>(null);
  let searchQuery = $state("");
  let searchResults = $state<SearchResult[]>([]);
  let searching = $state(false);

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

  function decodeProject(encoded: string): string {
    if (!encoded || !encoded.startsWith("-")) return encoded;
    return encoded.substring(1).replace(/-/g, "/");
  }

  let searchTimeout: ReturnType<typeof setTimeout> | undefined;
  function handleSearchInput(): void {
    clearTimeout(searchTimeout);
    searchTimeout = setTimeout(handleSearch, 300);
  }

  onMount(async () => {
    try {
      projects = await listProjects();
    } catch {
      // non-critical
    }
    await loadConversations();
  });
</script>

<div class="flex h-full flex-col">
  <div class="border-b border-border-primary px-4 py-2">
    <h1 class="text-xs font-medium text-text-secondary">// history</h1>
  </div>

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
          <div
            class="flex w-full items-start gap-2.5 border border-border-primary bg-bg-secondary px-3 py-2 transition-colors hover:bg-bg-hover"
          >
            <span class="mt-0.5 text-[10px] text-accent">></span>
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
              onclick={() => handleDelete(conv.file_path)}
              class="shrink-0 p-1 text-text-tertiary transition-colors hover:text-danger"
            >
              <Trash2 size={12} />
            </button>
          </div>
        {/each}
      </div>
    {/if}
  </div>
</div>
