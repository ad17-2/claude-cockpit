<script lang="ts">
  import { goto } from "$app/navigation";
  import { navItems } from "$lib/navigation.svelte";
  import { listProjects, type ProjectInfo } from "$lib/commands/projects";
  import { listEntities, type EntityInfo } from "$lib/commands/entities";
  import { searchConversations, type SearchResult } from "$lib/commands/history";
  import { fuzzySearch, type FuzzyResult } from "$lib/utils/fuzzy";

  let { open = $bindable(false) }: { open: boolean } = $props();

  let query = $state("");
  let selectedIndex = $state(0);
  let inputEl: HTMLInputElement | undefined = $state();

  let projects = $state<ProjectInfo[]>([]);
  let entities = $state<EntityInfo[]>([]);
  let conversationResults = $state<SearchResult[]>([]);

  let searchTimeout: ReturnType<typeof setTimeout> | undefined;

  interface PaletteItem {
    type: "page" | "project" | "entity" | "conversation";
    label: string;
    detail: string;
    action: () => void;
  }

  let results = $derived.by(() => {
    const items: PaletteItem[] = [];
    const q = query.trim();

    const pageMatches = fuzzySearch(q, navItems, (n) => n.label);
    for (const { item } of pageMatches) {
      items.push({
        type: "page",
        label: item.label,
        detail: item.href,
        action: () => { goto(item.href); close(); },
      });
    }

    const projectMatches = fuzzySearch(q, projects, (p) => p.name);
    for (const { item } of projectMatches.slice(0, 5)) {
      items.push({
        type: "project",
        label: item.name,
        detail: item.decoded_path,
        action: () => { goto(`/history`); close(); },
      });
    }

    const entityMatches = fuzzySearch(q, entities, (e) => e.name);
    for (const { item } of entityMatches.slice(0, 5)) {
      items.push({
        type: "entity",
        label: item.name,
        detail: `${item.entity_type} · ${item.scope}`,
        action: () => { goto(`/entities`); close(); },
      });
    }

    for (const r of conversationResults.slice(0, 5)) {
      items.push({
        type: "conversation",
        label: r.matched_line,
        detail: r.project,
        action: () => { goto(`/history`); close(); },
      });
    }

    return items;
  });

  function close(): void {
    open = false;
    query = "";
    selectedIndex = 0;
    conversationResults = [];
  }

  function handleKeydown(e: KeyboardEvent): void {
    if (e.key === "Escape") {
      e.preventDefault();
      close();
      return;
    }

    if (e.key === "ArrowDown") {
      e.preventDefault();
      selectedIndex = Math.min(selectedIndex + 1, results.length - 1);
      return;
    }

    if (e.key === "ArrowUp") {
      e.preventDefault();
      selectedIndex = Math.max(selectedIndex - 1, 0);
      return;
    }

    if (e.key === "Enter" && results[selectedIndex]) {
      e.preventDefault();
      results[selectedIndex].action();
      return;
    }
  }

  function handleInput(): void {
    selectedIndex = 0;
    clearTimeout(searchTimeout);
    if (query.trim().length >= 3) {
      searchTimeout = setTimeout(async () => {
        try {
          conversationResults = await searchConversations(query.trim(), 10);
        } catch {
          conversationResults = [];
        }
      }, 300);
    } else {
      conversationResults = [];
    }
  }

  async function loadData(): Promise<void> {
    try {
      const [p, agents, rules, commands, skills, hooks] = await Promise.all([
        listProjects(),
        listEntities("agents"),
        listEntities("rules"),
        listEntities("commands"),
        listEntities("skills"),
        listEntities("hooks"),
      ]);
      projects = p;
      entities = [...agents, ...rules, ...commands, ...skills, ...hooks];
    } catch {
      // silent
    }
  }

  $effect(() => {
    if (open) {
      loadData();
      setTimeout(() => inputEl?.focus(), 10);
    }
  });

  const typeLabels: Record<string, string> = {
    page: "page",
    project: "project",
    entity: "entity",
    conversation: "conversation",
  };
</script>

{#if open}
  <div class="fixed inset-0 z-50 flex items-start justify-center pt-24">
    <button class="fixed inset-0 bg-black/60" onclick={close} tabindex="-1" aria-label="Close palette"></button>
    <div class="relative z-10 w-full max-w-lg border border-border-primary bg-bg-primary shadow-2xl">
      <div class="flex items-center gap-2 border-b border-border-primary px-3 py-2">
        <span class="text-xs text-text-tertiary">></span>
        <input
          bind:this={inputEl}
          bind:value={query}
          oninput={handleInput}
          onkeydown={handleKeydown}
          placeholder="search pages, projects, entities..."
          class="flex-1 bg-transparent text-xs text-text-primary placeholder-text-tertiary outline-none"
        />
        <kbd class="text-[10px] text-text-tertiary">esc</kbd>
      </div>

      {#if results.length > 0}
        <div class="max-h-80 overflow-y-auto py-1">
          {#each results as item, i}
            <button
              onclick={item.action}
              class="flex w-full items-center gap-2 px-3 py-1.5 text-left text-xs transition-colors
                {i === selectedIndex ? 'bg-bg-hover text-text-primary' : 'text-text-secondary hover:bg-bg-hover'}"
            >
              <span class="shrink-0 bg-accent-muted px-1 py-0.5 text-[9px] text-accent">{typeLabels[item.type]}</span>
              <span class="min-w-0 flex-1 truncate">{item.label}</span>
              <span class="shrink-0 text-[10px] text-text-tertiary">{item.detail}</span>
            </button>
          {/each}
        </div>
      {:else if query.trim().length > 0}
        <div class="px-3 py-4 text-center">
          <p class="text-xs text-text-tertiary">// no results</p>
        </div>
      {/if}
    </div>
  </div>
{/if}
