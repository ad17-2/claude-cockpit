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
  import CodeMirrorEditor from "$lib/components/CodeMirrorEditor.svelte";
  import { Plus, Trash2, ArrowLeft, Save, Globe, FolderOpen } from "lucide-svelte";

  const entityTypes: { key: EntityType; label: string }[] = [
    { key: "agents", label: "Agents" },
    { key: "rules", label: "Rules" },
    { key: "commands", label: "Commands" },
    { key: "skills", label: "Skills" },
    { key: "hooks", label: "Hooks" },
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
      const fileName = entity.file_path.split("/").pop()?.replace(".md", "") ?? entity.name;
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
      const fileName = selectedEntity.file_path.split("/").pop()?.replace(".md", "") ?? selectedEntity.name;
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
    const fileName = selectedEntity.file_path.split("/").pop()?.replace(".md", "") ?? selectedEntity.name;
    await deleteEntity(activeType, selectedEntity.scope, fileName);
    selectedEntity = null;
    detail = null;
    await loadEntities();
  }

  async function handleCreate(): Promise<void> {
    const trimmed = newName.trim();
    if (!trimmed) return;
    await writeEntity(activeType, "global", trimmed, `---\nname: ${trimmed}\ndescription: \n---\n`);
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

  onMount(() => loadEntities());
</script>

<svelte:window onkeydown={handleKeydown} />

<div class="flex h-full flex-col">
  <div class="flex items-center justify-between border-b border-border-primary px-4 py-2">
    <div class="flex items-center gap-2">
      {#if selectedEntity}
        <button onclick={goBack} class="text-text-secondary hover:text-text-primary">
          <ArrowLeft size={16} />
        </button>
        <h1 class="text-sm font-semibold text-text-primary">{selectedEntity.name}</h1>
      {:else}
        <h1 class="text-sm font-semibold text-text-primary">Entities</h1>
      {/if}
    </div>
    <div class="flex items-center gap-2">
      {#if selectedEntity}
        {#if isDirty}
          <span class="text-xs text-warning">Unsaved</span>
        {/if}
        <button
          onclick={saveEntity}
          disabled={!isDirty || saving}
          class="flex items-center gap-1.5 rounded-md px-2.5 py-1 text-xs font-medium transition-colors
            {isDirty ? 'bg-accent text-white hover:bg-accent-hover' : 'bg-bg-tertiary text-text-tertiary cursor-not-allowed'}"
        >
          <Save size={13} />
          Save
        </button>
        <button
          onclick={handleDelete}
          class="flex items-center gap-1 rounded-md px-2 py-1 text-xs text-text-secondary transition-colors hover:bg-bg-hover hover:text-danger"
        >
          <Trash2 size={13} />
        </button>
      {:else}
        <button
          onclick={() => creating = !creating}
          class="flex items-center gap-1 rounded-md bg-accent px-2.5 py-1 text-xs font-medium text-white transition-colors hover:bg-accent-hover"
        >
          <Plus size={13} />
          New
        </button>
      {/if}
    </div>
  </div>

  {#if !selectedEntity}
    <div class="flex items-center gap-1 border-b border-border-primary px-3 py-2">
      {#each entityTypes as et}
        <button
          onclick={() => handleTypeChange(et.key)}
          class="rounded-md px-2.5 py-1 text-xs font-medium transition-colors
            {activeType === et.key ? 'bg-bg-active text-text-primary' : 'text-text-secondary hover:text-text-primary'}"
        >
          {et.label}
        </button>
      {/each}
    </div>
  {/if}

  {#if error}
    <div class="px-4 py-3">
      <p class="text-sm text-danger">{error}</p>
    </div>
  {/if}

  {#if creating}
    <div class="flex items-center gap-2 border-b border-border-primary px-4 py-2">
      <input
        type="text"
        bind:value={newName}
        placeholder="Entity name (filename without .md)"
        onkeydown={(e) => e.key === "Enter" && handleCreate()}
        class="flex-1 rounded-md border border-border-primary bg-bg-tertiary px-2.5 py-1.5 text-xs text-text-primary placeholder-text-tertiary outline-none focus:border-border-focus"
      />
      <button
        onclick={handleCreate}
        disabled={!newName.trim()}
        class="rounded-md bg-accent px-3 py-1.5 text-xs font-medium text-white hover:bg-accent-hover disabled:opacity-50"
      >
        Create
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
        <p class="text-sm text-text-tertiary">Loading...</p>
      </div>
    {:else if entities.length === 0}
      <div class="flex h-full items-center justify-center">
        <p class="text-sm text-text-tertiary">No {activeType} found.</p>
      </div>
    {:else}
      <div class="space-y-1 p-3">
        {#each entities as entity}
          <button
            onclick={() => selectEntity(entity)}
            class="flex w-full items-start gap-3 rounded-md border border-border-primary bg-bg-secondary px-3 py-2.5 text-left transition-colors hover:bg-bg-hover"
          >
            <div class="flex-1 min-w-0">
              <div class="flex items-center gap-2">
                <p class="text-sm font-medium text-text-primary">{entity.name}</p>
                <span class="flex items-center gap-1 text-[10px] text-text-tertiary">
                  {#if entity.scope === "global"}
                    <Globe size={10} />
                  {:else}
                    <FolderOpen size={10} />
                  {/if}
                  {entity.scope}
                </span>
              </div>
              {#if entity.description}
                <p class="mt-0.5 truncate text-xs text-text-secondary">{entity.description}</p>
              {/if}
            </div>
          </button>
        {/each}
      </div>
    {/if}
  </div>
</div>
