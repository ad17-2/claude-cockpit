<script lang="ts">
  import { onMount } from "svelte";
  import { listProjects, type ProjectInfo } from "$lib/commands/projects";
  import {
    readSettings,
    getEffectiveSettings,
    type FileType,
    type PermissionCategory,
  } from "$lib/commands/settings";
  import { listMcpServers, type McpServerInfo } from "$lib/commands/mcp";
  import { onFileChange } from "$lib/commands/watcher";
  import { decodeProject } from "$lib/utils/format";
  import ScopeTabBar from "$lib/components/ScopeTabBar.svelte";
  import PermissionsList from "$lib/components/PermissionsList.svelte";
  import { Shield, ShieldOff, HelpCircle, ChevronRight, ChevronDown } from "lucide-svelte";

  let projects = $state<ProjectInfo[]>([]);
  let activeScope = $state("global");
  let activeFileType = $state<FileType>("settings_local");
  let settings = $state<Record<string, unknown>>({});
  let loading = $state(true);
  let error = $state<string | null>(null);

  let effectiveMode = $state(false);
  let effectiveProject = $state<string | null>(null);
  let effectiveSettings = $state<Record<string, unknown>>({});
  let effectiveLoading = $state(false);

  let mcpServers = $state<McpServerInfo[]>([]);
  let expandedServers = $state(new Set<string>());

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

  async function loadEffective(): Promise<void> {
    if (!effectiveProject) return;
    effectiveLoading = true;
    error = null;
    try {
      const decoded = decodeProject(effectiveProject);
      effectiveSettings = await getEffectiveSettings(decoded);
    } catch (e) {
      error = String(e);
      effectiveSettings = {};
    } finally {
      effectiveLoading = false;
    }
  }

  function toggleEffectiveMode(): void {
    effectiveMode = !effectiveMode;
    if (effectiveMode && projects.length > 0 && !effectiveProject) {
      effectiveProject = projects[0].decoded_path;
      loadEffective();
    }
  }

  function handleEffectiveProjectChange(decoded: string): void {
    effectiveProject = decoded;
    loadEffective();
  }

  function toggleServer(name: string): void {
    if (expandedServers.has(name)) {
      expandedServers.delete(name);
    } else {
      expandedServers.add(name);
    }
    expandedServers = new Set(expandedServers);
  }

  function handleScopeChange(scope: string): void {
    activeScope = scope;
    loadSettings();
  }

  function handleFileTypeChange(ft: FileType): void {
    activeFileType = ft;
    loadSettings();
  }

  onMount(() => {
    let unlisten: (() => void) | undefined;

    (async () => {
      try {
        projects = await listProjects();
      } catch {}
      await loadSettings();

      try {
        mcpServers = await listMcpServers();
      } catch {}

      unlisten = await onFileChange("settings-changed", () => {
        loadSettings();
        listMcpServers().then((s) => mcpServers = s).catch(() => {});
      });
    })();

    return () => unlisten?.();
  });

  const categories: { key: PermissionCategory; label: string; icon: typeof Shield }[] = [
    { key: "allow", label: "allow", icon: Shield },
    { key: "deny", label: "deny", icon: ShieldOff },
    { key: "ask", label: "ask", icon: HelpCircle },
  ];
</script>

<div class="flex h-full flex-col">
  <div class="flex items-center justify-between border-b border-border-primary px-4 py-2">
    <h1 class="text-xs font-medium text-text-secondary">// settings</h1>
    <div class="flex items-center gap-1">
      <button
        onclick={() => handleFileTypeChange("settings")}
        class="px-2 py-1 text-xs transition-colors
          {activeFileType === 'settings' && !effectiveMode ? 'text-accent' : 'text-text-secondary hover:text-text-primary'}"
      >
        [settings.json]
      </button>
      <button
        onclick={() => handleFileTypeChange("settings_local")}
        class="px-2 py-1 text-xs transition-colors
          {activeFileType === 'settings_local' && !effectiveMode ? 'text-accent' : 'text-text-secondary hover:text-text-primary'}"
      >
        [settings.local.json]
      </button>
      <button
        onclick={toggleEffectiveMode}
        class="px-2 py-1 text-xs transition-colors
          {effectiveMode ? 'text-accent' : 'text-text-secondary hover:text-text-primary'}"
      >
        [effective]
      </button>
    </div>
  </div>

  {#if effectiveMode}
    <div class="flex items-center gap-2 border-b border-border-primary px-3 py-1.5">
      {#each projects as project}
        <button
          onclick={() => handleEffectiveProjectChange(project.decoded_path)}
          class="px-2 py-1 text-xs transition-colors
            {effectiveProject === project.decoded_path ? 'text-accent' : 'text-text-secondary hover:text-text-primary'}"
        >
          [{project.name}]
        </button>
      {/each}
    </div>
  {:else}
    <ScopeTabBar {projects} {activeScope} onselect={handleScopeChange} />
  {/if}

  {#if error}
    <div class="px-4 py-2">
      <p class="text-xs text-danger">{error}</p>
    </div>
  {/if}

  <div class="flex-1 overflow-y-auto">
    {#if effectiveMode}
      {#if effectiveLoading}
        <div class="flex h-full items-center justify-center">
          <p class="text-xs text-text-tertiary">loading...</p>
        </div>
      {:else}
        <div class="p-4">
          <pre class="overflow-x-auto border border-border-primary bg-bg-tertiary p-3 text-xs text-text-secondary">{JSON.stringify(effectiveSettings, null, 2)}</pre>
        </div>
      {/if}
    {:else if loading}
      <div class="flex h-full items-center justify-center">
        <p class="text-xs text-text-tertiary">loading...</p>
      </div>
    {:else}
      <div class="space-y-5 p-4">
        {#each categories as cat}
          <div>
            <div class="mb-2 flex items-center gap-2">
              <cat.icon size={13} class="text-text-secondary" />
              <h2 class="text-[11px] uppercase tracking-wider text-text-secondary">
                // {cat.label} ({permissions[cat.key].length})
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
            <h2 class="mb-2 text-[11px] uppercase tracking-wider text-text-secondary">// other settings</h2>
            <pre class="overflow-x-auto border border-border-primary bg-bg-tertiary p-3 text-xs text-text-secondary">{JSON.stringify(
              Object.fromEntries(
                Object.entries(settings).filter(([k]) => k !== "permissions")
              ),
              null,
              2,
            )}</pre>
          </div>
        {/if}

        {#if mcpServers.length > 0}
          <div>
            <h2 class="mb-2 text-[11px] uppercase tracking-wider text-text-secondary">
              // mcp servers ({mcpServers.length})
            </h2>
            <div class="space-y-px">
              {#each mcpServers as server}
                <div class="border border-border-primary bg-bg-secondary">
                  <button
                    onclick={() => toggleServer(server.name)}
                    class="flex w-full items-center gap-2 px-3 py-2 text-left transition-colors hover:bg-bg-hover"
                  >
                    {#if expandedServers.has(server.name)}
                      <ChevronDown size={12} class="text-text-tertiary" />
                    {:else}
                      <ChevronRight size={12} class="text-text-tertiary" />
                    {/if}
                    <span class="text-xs text-text-primary">{server.name}</span>
                    <span class="bg-accent-muted px-1.5 py-0.5 text-[10px] text-accent">{server.scope}</span>
                  </button>
                  {#if expandedServers.has(server.name)}
                    <div class="border-t border-border-primary px-3 py-2 space-y-1">
                      {#if server.command}
                        <p class="text-[11px] text-text-secondary">
                          <span class="text-text-tertiary">command:</span> {server.command}
                        </p>
                      {/if}
                      {#if server.url}
                        <p class="text-[11px] text-text-secondary">
                          <span class="text-text-tertiary">url:</span> {server.url}
                        </p>
                      {/if}
                      {#if server.args.length > 0}
                        <p class="text-[11px] text-text-secondary">
                          <span class="text-text-tertiary">args:</span> {server.args.join(" ")}
                        </p>
                      {/if}
                      {#if server.env_keys.length > 0}
                        <p class="text-[11px] text-text-secondary">
                          <span class="text-text-tertiary">env:</span> {server.env_keys.join(", ")}
                        </p>
                      {/if}
                    </div>
                  {/if}
                </div>
              {/each}
            </div>
          </div>
        {/if}
      </div>
    {/if}
  </div>
</div>
