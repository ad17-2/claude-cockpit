<script lang="ts">
  import { Globe, FolderOpen } from "lucide-svelte";
  import type { ProjectInfo } from "$lib/commands/projects";

  interface Props {
    projects: ProjectInfo[];
    activeScope: string;
    onselect: (scope: string) => void;
  }

  let { projects, activeScope, onselect }: Props = $props();
</script>

<div class="flex items-center gap-1 overflow-x-auto border-b border-border-primary px-3 py-1.5">
  <button
    onclick={() => onselect("global")}
    class="flex shrink-0 items-center gap-1.5 px-2 py-1 text-xs transition-colors
      {activeScope === 'global' ? 'text-accent border-b border-accent' : 'text-text-secondary hover:text-text-primary'}"
  >
    <Globe size={12} />
    [global]
  </button>

  {#each projects.filter(p => !p.is_root) as project}
    <button
      onclick={() => onselect(project.decoded_path)}
      class="flex shrink-0 items-center gap-1.5 px-2 py-1 text-xs transition-colors
        {activeScope === project.decoded_path ? 'text-accent border-b border-accent' : 'text-text-secondary hover:text-text-primary'}"
    >
      <FolderOpen size={12} />
      [{project.name}]
    </button>
  {/each}
</div>
