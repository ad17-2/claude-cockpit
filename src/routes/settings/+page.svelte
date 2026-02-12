<script lang="ts">
  import { onMount } from "svelte";
  import { listProjects, type ProjectInfo } from "$lib/commands/projects";
  import {
    readSettings,
    type FileType,
    type PermissionCategory,
  } from "$lib/commands/settings";
  import ScopeTabBar from "$lib/components/ScopeTabBar.svelte";
  import PermissionsList from "$lib/components/PermissionsList.svelte";
  import { Shield, ShieldOff, HelpCircle } from "lucide-svelte";

  let projects = $state<ProjectInfo[]>([]);
  let activeScope = $state("global");
  let activeFileType = $state<FileType>("settings_local");
  let settings = $state<Record<string, unknown>>({});
  let loading = $state(true);
  let error = $state<string | null>(null);

  interface Permissions {
    allow: string[];
    deny: string[];
    ask: string[];
  }

  let permissions = $derived<Permissions>({
    allow: ((settings.permissions as Record<string, unknown>)?.allow as string[]) ?? [],
    deny: ((settings.permissions as Record<string, unknown>)?.deny as string[]) ?? [],
    ask: ((settings.permissions as Record<string, unknown>)?.ask as string[]) ?? [],
  });

  async function loadSettings(): Promise<void> {
    loading = true;
    error = null;
    try {
      settings = await readSettings(activeScope, activeFileType);
    } catch (e) {
      error = String(e);
      settings = {};
    } finally {
      loading = false;
    }
  }

  function handleScopeChange(scope: string): void {
    activeScope = scope;
    loadSettings();
  }

  function handleFileTypeChange(ft: FileType): void {
    activeFileType = ft;
    loadSettings();
  }

  onMount(async () => {
    try {
      projects = await listProjects();
    } catch (_) {
      // non-critical
    }
    await loadSettings();
  });

  const categories: { key: PermissionCategory; label: string; icon: typeof Shield }[] = [
    { key: "allow", label: "Allowed", icon: Shield },
    { key: "deny", label: "Denied", icon: ShieldOff },
    { key: "ask", label: "Ask", icon: HelpCircle },
  ];
</script>

<div class="flex h-full flex-col">
  <div class="flex items-center justify-between border-b border-border-primary px-4 py-2">
    <h1 class="text-sm font-semibold text-text-primary">Settings</h1>
    <div class="flex items-center gap-1">
      <button
        onclick={() => handleFileTypeChange("settings")}
        class="rounded px-2 py-1 text-xs font-medium transition-colors
          {activeFileType === 'settings' ? 'bg-bg-active text-text-primary' : 'text-text-secondary hover:text-text-primary'}"
      >
        settings.json
      </button>
      <button
        onclick={() => handleFileTypeChange("settings_local")}
        class="rounded px-2 py-1 text-xs font-medium transition-colors
          {activeFileType === 'settings_local' ? 'bg-bg-active text-text-primary' : 'text-text-secondary hover:text-text-primary'}"
      >
        settings.local.json
      </button>
    </div>
  </div>

  <ScopeTabBar {projects} {activeScope} onselect={handleScopeChange} />

  {#if error}
    <div class="px-4 py-3">
      <p class="text-sm text-danger">{error}</p>
    </div>
  {/if}

  <div class="flex-1 overflow-y-auto">
    {#if loading}
      <div class="flex h-full items-center justify-center">
        <p class="text-sm text-text-tertiary">Loading...</p>
      </div>
    {:else}
      <div class="space-y-6 p-4">
        {#each categories as cat}
          <div>
            <div class="mb-2 flex items-center gap-2">
              <cat.icon size={14} class="text-text-secondary" />
              <h2 class="text-xs font-semibold uppercase tracking-wider text-text-secondary">
                {cat.label} ({permissions[cat.key].length})
              </h2>
            </div>
            <PermissionsList
              scope={activeScope}
              fileType={activeFileType}
              category={cat.key}
              permissions={permissions[cat.key]}
              onupdate={loadSettings}
            />
          </div>
        {/each}

        {#if Object.keys(settings).filter(k => k !== "permissions").length > 0}
          <div>
            <h2 class="mb-2 text-xs font-semibold uppercase tracking-wider text-text-secondary">Other Settings</h2>
            <pre class="overflow-x-auto rounded-md border border-border-primary bg-bg-tertiary p-3 font-mono text-xs text-text-secondary">{JSON.stringify(
              Object.fromEntries(
                Object.entries(settings).filter(([k]) => k !== "permissions")
              ),
              null,
              2,
            )}</pre>
          </div>
        {/if}
      </div>
    {/if}
  </div>
</div>
