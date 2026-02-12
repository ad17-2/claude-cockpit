<script lang="ts">
  import { onMount } from "svelte";
  import { listProjects, type ProjectInfo } from "$lib/commands/projects";
  import { readClaudeMd, writeClaudeMd } from "$lib/commands/claude-md";
  import { onFileChange } from "$lib/commands/watcher";
  import MarkdownEditor from "$lib/components/MarkdownEditor.svelte";
  import ScopeTabBar from "$lib/components/ScopeTabBar.svelte";
  import { Save } from "lucide-svelte";

  let projects = $state<ProjectInfo[]>([]);
  let activeScope = $state("global");
  let content = $state("");
  let savedContent = $state("");
  let loading = $state(true);
  let saving = $state(false);
  let error = $state<string | null>(null);

  let isDirty = $derived(content !== savedContent);

  async function loadContent(scope: string): Promise<void> {
    loading = true;
    error = null;
    try {
      const result = await readClaudeMd(scope);
      content = result ?? "";
      savedContent = content;
    } catch (e) {
      error = String(e);
      content = "";
      savedContent = "";
    } finally {
      loading = false;
    }
  }

  async function save(): Promise<void> {
    if (!isDirty || saving) return;
    saving = true;
    error = null;
    try {
      await writeClaudeMd(activeScope, content);
      savedContent = content;
    } catch (e) {
      error = String(e);
    } finally {
      saving = false;
    }
  }

  function handleScopeChange(scope: string): void {
    activeScope = scope;
    loadContent(scope);
  }

  function handleKeydown(e: KeyboardEvent): void {
    if ((e.metaKey || e.ctrlKey) && e.key === "s") {
      e.preventDefault();
      save();
    }
  }

  onMount(() => {
    let unlisten: (() => void) | undefined;

    (async () => {
      try {
        projects = await listProjects();
      } catch {}
      await loadContent(activeScope);

      unlisten = await onFileChange("claude-md-changed", () => {
        loadContent(activeScope);
      });
    })();

    return () => unlisten?.();
  });
</script>

<svelte:window onkeydown={handleKeydown} />

<div class="flex h-full flex-col">
  <div class="flex items-center justify-between border-b border-border-primary px-4 py-2">
    <h1 class="text-xs font-medium text-text-secondary">// CLAUDE.md</h1>
    <div class="flex items-center gap-2">
      {#if isDirty}
        <span class="text-[11px] text-warning">[modified]</span>
      {/if}
      <button
        onclick={save}
        disabled={!isDirty || saving}
        class="flex items-center gap-1.5 px-2 py-1 text-xs font-medium transition-colors
          {isDirty ? 'bg-accent text-bg-primary hover:bg-accent-hover' : 'bg-bg-tertiary text-text-tertiary cursor-not-allowed'}"
      >
        <Save size={12} />
        {saving ? "saving..." : "save"}
      </button>
    </div>
  </div>

  <ScopeTabBar {projects} {activeScope} onselect={handleScopeChange} />

  {#if error}
    <div class="px-4 py-2">
      <p class="text-xs text-danger">{error}</p>
    </div>
  {/if}

  <div class="flex-1 overflow-hidden">
    {#if loading}
      <div class="flex h-full items-center justify-center">
        <p class="text-xs text-text-tertiary">loading...</p>
      </div>
    {:else}
      <MarkdownEditor
        value={content}
        onchange={(v) => content = v}
      />
    {/if}
  </div>
</div>
