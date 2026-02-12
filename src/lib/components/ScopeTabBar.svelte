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

<div class="flex items-center gap-1 overflow-x-auto border-b border-border-primary px-3 py-2">
  <button
    onclick={() => onselect("global")}
    class="flex shrink-0 items-center gap-1.5 rounded-md px-2.5 py-1 text-xs font-medium transition-colors
      {activeScope === 'global' ? 'bg-bg-active text-text-primary' : 'text-text-secondary hover:text-text-primary'}"
  >
    <Globe size={13} />
    Global
  </button>

  {#each projects as project}
    <button
      onclick={() => onselect(project.decoded_path)}
      class="flex shrink-0 items-center gap-1.5 rounded-md px-2.5 py-1 text-xs font-medium transition-colors
        {activeScope === project.decoded_path ? 'bg-bg-active text-text-primary' : 'text-text-secondary hover:text-text-primary'}"
    >
      <FolderOpen size={13} />
      {project.name}
    </button>
  {/each}
</div>
