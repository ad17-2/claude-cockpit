<script lang="ts">
  import { X, Plus } from "lucide-svelte";
  import type { PermissionCategory, FileType } from "$lib/commands/settings";
  import { addPermission, removePermission } from "$lib/commands/settings";

  interface Props {
    scope: string;
    fileType: FileType;
    category: PermissionCategory;
    permissions: string[];
    onupdate: () => void;
  }

  let { scope, fileType, category, permissions, onupdate }: Props = $props();

  let newPermission = $state("");
  let adding = $state(false);

  interface PermissionGroup {
    prefix: string;
    items: string[];
  }

  let grouped: PermissionGroup[] = $derived.by(() => {
    const groups = new Map<string, string[]>();
    for (const perm of permissions) {
      let prefix: string;
      if (perm.startsWith("Bash(")) {
        prefix = "Bash";
      } else if (perm.startsWith("WebFetch(")) {
        prefix = "WebFetch";
      } else if (perm.startsWith("WebSearch")) {
        prefix = "WebSearch";
      } else if (perm.startsWith("mcp__")) {
        const parts = perm.split("__");
        prefix = parts.length >= 3 ? `mcp__${parts[1]}__${parts[2]}` : perm;
      } else {
        prefix = "Other";
      }
      const list = groups.get(prefix) ?? [];
      list.push(perm);
      groups.set(prefix, list);
    }

    return Array.from(groups.entries())
      .sort(([a], [b]) => a.localeCompare(b))
      .map(([prefix, items]) => ({ prefix, items: items.sort() }));
  });

  async function handleAdd(): Promise<void> {
    const trimmed = newPermission.trim();
    if (!trimmed || adding) return;
    adding = true;
    try {
      await addPermission(scope, fileType, category, trimmed);
      newPermission = "";
      onupdate();
    } finally {
      adding = false;
    }
  }

  async function handleRemove(permission: string): Promise<void> {
    await removePermission(scope, fileType, category, permission);
    onupdate();
  }

  function handleKeydown(e: KeyboardEvent): void {
    if (e.key === "Enter") {
      e.preventDefault();
      handleAdd();
    }
  }
</script>

<div class="space-y-3">
  <div class="flex items-center gap-2">
    <input
      type="text"
      bind:value={newPermission}
      onkeydown={handleKeydown}
      placeholder='e.g. Bash(git:*), WebSearch, mcp__plugin__tool'
      class="flex-1 rounded-md border border-border-primary bg-bg-tertiary px-2.5 py-1.5 text-xs text-text-primary placeholder-text-tertiary outline-none focus:border-border-focus"
    />
    <button
      onclick={handleAdd}
      disabled={!newPermission.trim() || adding}
      class="flex items-center gap-1 rounded-md bg-accent px-2.5 py-1.5 text-xs font-medium text-white transition-colors hover:bg-accent-hover disabled:cursor-not-allowed disabled:opacity-50"
    >
      <Plus size={12} />
      Add
    </button>
  </div>

  {#if permissions.length === 0}
    <p class="text-xs text-text-tertiary">No {category} permissions.</p>
  {:else}
    {#each grouped as group}
      <div>
        <p class="mb-1 text-[11px] font-medium uppercase tracking-wider text-text-tertiary">{group.prefix} ({group.items.length})</p>
        <div class="space-y-0.5">
          {#each group.items as perm}
            <div class="group flex items-center justify-between rounded px-2 py-1 hover:bg-bg-hover">
              <span class="font-mono text-xs text-text-secondary">{perm}</span>
              <button
                onclick={() => handleRemove(perm)}
                class="invisible text-text-tertiary transition-colors hover:text-danger group-hover:visible"
              >
                <X size={13} />
              </button>
            </div>
          {/each}
        </div>
      </div>
    {/each}
  {/if}
</div>
