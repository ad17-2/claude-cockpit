<script lang="ts">
  import CodeMirrorEditor from "./CodeMirrorEditor.svelte";
  import TipTapEditor from "./TipTapEditor.svelte";
  import { Code, Eye } from "lucide-svelte";

  interface Props {
    value: string;
    onchange: (value: string) => void;
  }

  let { value, onchange }: Props = $props();

  let mode = $state<"raw" | "wysiwyg">("raw");
</script>

<div class="flex h-full flex-col">
  <div class="flex items-center gap-1 border-b border-border-primary px-2 py-1.5">
    <button
      onclick={() => mode = "raw"}
      class="flex items-center gap-1.5 rounded px-2 py-1 text-xs font-medium transition-colors
        {mode === 'raw' ? 'bg-bg-active text-text-primary' : 'text-text-secondary hover:text-text-primary'}"
    >
      <Code size={14} />
      Raw
    </button>
    <button
      onclick={() => mode = "wysiwyg"}
      class="flex items-center gap-1.5 rounded px-2 py-1 text-xs font-medium transition-colors
        {mode === 'wysiwyg' ? 'bg-bg-active text-text-primary' : 'text-text-secondary hover:text-text-primary'}"
    >
      <Eye size={14} />
      Preview
    </button>
  </div>

  <div class="flex-1 overflow-hidden">
    {#if mode === "raw"}
      <CodeMirrorEditor {value} {onchange} />
    {:else}
      <TipTapEditor {value} {onchange} />
    {/if}
  </div>
</div>
