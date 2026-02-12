<script lang="ts">
  import { onMount } from "svelte";
  import { listProjects, type ProjectInfo } from "$lib/commands/projects";
  import { readClaudeMd, writeClaudeMd } from "$lib/commands/claude-md";
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

  onMount(async () => {
    try {
      projects = await listProjects();
    } catch (_) {
      // projects list is non-critical
    }
    await loadContent(activeScope);
  });
</script>

<svelte:window onkeydown={handleKeydown} />

<div class="flex h-full flex-col">
  <div class="flex items-center justify-between border-b border-border-primary px-4 py-2">
    <h1 class="text-sm font-semibold text-text-primary">CLAUDE.md</h1>
    <div class="flex items-center gap-2">
      {#if isDirty}
        <span class="text-xs text-warning">Unsaved changes</span>
      {/if}
      <button
        onclick={save}
        disabled={!isDirty || saving}
        class="flex items-center gap-1.5 rounded-md px-2.5 py-1 text-xs font-medium transition-colors
          {isDirty ? 'bg-accent text-white hover:bg-accent-hover' : 'bg-bg-tertiary text-text-tertiary cursor-not-allowed'}"
      >
        <Save size={13} />
        {saving ? "Saving..." : "Save"}
      </button>
    </div>
  </div>

  <ScopeTabBar {projects} {activeScope} onselect={handleScopeChange} />

  {#if error}
    <div class="px-4 py-3">
      <p class="text-sm text-danger">{error}</p>
    </div>
  {/if}

  <div class="flex-1 overflow-hidden">
    {#if loading}
      <div class="flex h-full items-center justify-center">
        <p class="text-sm text-text-tertiary">Loading...</p>
      </div>
    {:else}
      <MarkdownEditor
        value={content}
        onchange={(v) => content = v}
      />
    {/if}
  </div>
</div>
