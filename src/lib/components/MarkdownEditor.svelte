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
  <div class="flex items-center gap-1 border-b border-border-primary px-2 py-1">
    <button
      onclick={() => mode = "raw"}
      class="flex items-center gap-1.5 px-2 py-1 text-xs transition-colors
        {mode === 'raw' ? 'text-accent' : 'text-text-secondary hover:text-text-primary'}"
    >
      <Code size={13} />
      [raw]
    </button>
    <button
      onclick={() => mode = "wysiwyg"}
      class="flex items-center gap-1.5 px-2 py-1 text-xs transition-colors
        {mode === 'wysiwyg' ? 'text-accent' : 'text-text-secondary hover:text-text-primary'}"
    >
      <Eye size={13} />
      [preview]
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
