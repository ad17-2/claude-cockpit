<script lang="ts">
  import { onMount } from "svelte";
  import {
    listEntities,
    readEntity,
    writeEntity,
    deleteEntity,
    type EntityType,
    type EntityInfo,
    type EntityDetail,
  } from "$lib/commands/entities";
  import { onFileChange } from "$lib/commands/watcher";
  import { getTemplate } from "$lib/templates/entity-templates";
  import CodeMirrorEditor from "$lib/components/CodeMirrorEditor.svelte";
  import { Plus, Trash2, ArrowLeft, Save, Globe, FolderOpen } from "lucide-svelte";

  const entityTypes: { key: EntityType; label: string }[] = [
    { key: "agents", label: "agents" },
    { key: "rules", label: "rules" },
    { key: "commands", label: "commands" },
    { key: "skills", label: "skills" },
    { key: "hooks", label: "hooks" },
  ];

  let activeType = $state<EntityType>("agents");
  let entities = $state<EntityInfo[]>([]);
  let loading = $state(true);
  let error = $state<string | null>(null);

  let selectedEntity = $state<EntityInfo | null>(null);
  let detail = $state<EntityDetail | null>(null);
  let editContent = $state("");
  let savedContent = $state("");
  let saving = $state(false);
  let creating = $state(false);
  let newName = $state("");

  let isDirty = $derived(editContent !== savedContent);

  function fileNameFromPath(filePath: string): string {
    return filePath.split("/").pop()?.replace(".md", "") ?? "";
  }

  async function loadEntities(): Promise<void> {
    loading = true;
    error = null;
    try {
      entities = await listEntities(activeType);
    } catch (e) {
      error = String(e);
      entities = [];
    } finally {
      loading = false;
    }
  }

  async function selectEntity(entity: EntityInfo): Promise<void> {
    selectedEntity = entity;
    creating = false;
    try {
      const fileName = fileNameFromPath(entity.file_path) || entity.name;
      detail = await readEntity(
        activeType,
        entity.scope,
        fileName,
      );
      const raw = buildRawContent(detail);
      editContent = raw;
      savedContent = raw;
    } catch (e) {
      error = String(e);
    }
  }

  function buildRawContent(d: EntityDetail): string {
    const fm = d.frontmatter;
    const keys = Object.keys(fm);
    if (keys.length === 0) return d.body;

    let result = "---\n";
    for (const key of keys) {
      result += `${key}: ${fm[key]}\n`;
    }
    result += "---\n";
    if (d.body) {
      result += "\n" + d.body;
    }
    return result;
  }

  async function saveEntity(): Promise<void> {
    if (!selectedEntity || !isDirty || saving) return;
    saving = true;
    try {
      const fileName = fileNameFromPath(selectedEntity.file_path) || selectedEntity.name;
      await writeEntity(activeType, selectedEntity.scope, fileName, editContent);
      savedContent = editContent;
      await loadEntities();
    } catch (e) {
      error = String(e);
    } finally {
      saving = false;
    }
  }

  async function handleDelete(): Promise<void> {
    if (!selectedEntity) return;
    const fileName = fileNameFromPath(selectedEntity.file_path) || selectedEntity.name;
    await deleteEntity(activeType, selectedEntity.scope, fileName);
    selectedEntity = null;
    detail = null;
    await loadEntities();
  }

  async function handleCreate(): Promise<void> {
    const trimmed = newName.trim();
    if (!trimmed) return;
    const template = getTemplate(activeType);
    const content = template.replace(/^(name: )$/m, `name: ${trimmed}`);
    await writeEntity(activeType, "global", trimmed, content);
    newName = "";
    creating = false;
    await loadEntities();
  }

  function handleTypeChange(type: EntityType): void {
    activeType = type;
    selectedEntity = null;
    detail = null;
    loadEntities();
  }

  function goBack(): void {
    selectedEntity = null;
    detail = null;
  }

  function handleKeydown(e: KeyboardEvent): void {
    if ((e.metaKey || e.ctrlKey) && e.key === "s") {
      e.preventDefault();
      saveEntity();
    }
  }

  onMount(() => {
    let unlisten: (() => void) | undefined;

    (async () => {
      await loadEntities();
      unlisten = await onFileChange("entity-changed", () => loadEntities());
    })();

    return () => unlisten?.();
  });
</script>

<svelte:window onkeydown={handleKeydown} />

<div class="flex h-full flex-col">
  <div class="flex items-center justify-between border-b border-border-primary px-4 py-2">
    <div class="flex items-center gap-2">
      {#if selectedEntity}
        <button onclick={goBack} class="text-text-secondary hover:text-text-primary">
          <ArrowLeft size={14} />
        </button>
        <h1 class="text-xs font-medium text-text-secondary">// {selectedEntity.name}</h1>
      {:else}
        <h1 class="text-xs font-medium text-text-secondary">// entities</h1>
      {/if}
    </div>
    <div class="flex items-center gap-2">
      {#if selectedEntity}
        {#if isDirty}
          <span class="text-[11px] text-warning">[modified]</span>
        {/if}
        <button
          onclick={saveEntity}
          disabled={!isDirty || saving}
          class="flex items-center gap-1.5 px-2 py-1 text-xs font-medium transition-colors
            {isDirty ? 'bg-accent text-bg-primary hover:bg-accent-hover' : 'bg-bg-tertiary text-text-tertiary cursor-not-allowed'}"
        >
          <Save size={12} />
          save
        </button>
        <button
          onclick={handleDelete}
          class="flex items-center gap-1 px-2 py-1 text-xs text-text-secondary transition-colors hover:bg-bg-hover hover:text-danger"
        >
          <Trash2 size={12} />
        </button>
      {:else}
        <button
          onclick={() => creating = !creating}
          class="flex items-center gap-1 bg-accent px-2 py-1 text-xs font-medium text-bg-primary transition-colors hover:bg-accent-hover"
        >
          <Plus size={12} />
          new
        </button>
      {/if}
    </div>
  </div>

  {#if !selectedEntity}
    <div class="flex items-center gap-1 border-b border-border-primary px-3 py-1.5">
      {#each entityTypes as et}
        <button
          onclick={() => handleTypeChange(et.key)}
          class="px-2 py-1 text-xs transition-colors
            {activeType === et.key ? 'text-accent' : 'text-text-secondary hover:text-text-primary'}"
        >
          [{et.label}]
        </button>
      {/each}
    </div>
  {/if}

  {#if error}
    <div class="px-4 py-2">
      <p class="text-xs text-danger">{error}</p>
    </div>
  {/if}

  {#if creating}
    <div class="flex items-center gap-2 border-b border-border-primary px-4 py-2">
      <input
        type="text"
        bind:value={newName}
        placeholder="entity name (filename without .md)"
        onkeydown={(e) => e.key === "Enter" && handleCreate()}
        class="flex-1 border border-border-primary bg-bg-tertiary px-2 py-1.5 text-xs text-text-primary placeholder-text-tertiary outline-none focus:border-border-focus"
      />
      <button
        onclick={handleCreate}
        disabled={!newName.trim()}
        class="bg-accent px-2.5 py-1.5 text-xs font-medium text-bg-primary hover:bg-accent-hover disabled:opacity-50"
      >
        create
      </button>
    </div>
  {/if}

  <div class="flex-1 overflow-y-auto">
    {#if selectedEntity && detail}
      <CodeMirrorEditor
        value={editContent}
        onchange={(v) => editContent = v}
      />
    {:else if loading}
      <div class="flex h-full items-center justify-center">
        <p class="text-xs text-text-tertiary">loading...</p>
      </div>
    {:else if entities.length === 0}
      <div class="flex h-full items-center justify-center">
        <p class="text-xs text-text-tertiary">// no {activeType} found</p>
      </div>
    {:else}
      <div class="space-y-px p-3">
        {#each entities as entity}
          <button
            onclick={() => selectEntity(entity)}
            class="flex w-full items-start gap-2.5 border border-border-primary bg-bg-secondary px-3 py-2 text-left transition-colors hover:bg-bg-hover"
          >
            <div class="flex-1 min-w-0">
              <div class="flex items-center gap-2">
                <span class="text-[10px] text-accent">></span>
                <p class="text-xs text-text-primary">{entity.name}</p>
                <span class="flex items-center gap-1 text-[10px] text-text-tertiary">
                  {#if entity.scope === "global"}
                    <Globe size={9} />
                  {:else}
                    <FolderOpen size={9} />
                  {/if}
                  {entity.scope}
                </span>
              </div>
              {#if entity.description}
                <p class="mt-0.5 truncate pl-5 text-[11px] text-text-secondary">{entity.description}</p>
              {/if}
            </div>
          </button>
        {/each}
      </div>
    {/if}
  </div>
</div>
