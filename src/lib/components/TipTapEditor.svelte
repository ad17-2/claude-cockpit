<script lang="ts">
  import { onMount, onDestroy } from "svelte";
  import { Editor } from "@tiptap/core";
  import StarterKit from "@tiptap/starter-kit";
  import { Markdown } from "tiptap-markdown";

  interface MarkdownStorage {
    getMarkdown: () => string;
  }

  function getMarkdownFromEditor(e: Editor): string {
    // TipTap types storage as browser Storage, but it's actually a Record at runtime
    const storage = e.storage as unknown as Record<string, MarkdownStorage>;
    return storage.markdown.getMarkdown();
  }

  interface Props {
    value: string;
    onchange: (value: string) => void;
  }

  let { value, onchange }: Props = $props();

  let element: HTMLDivElement;
  let editor: Editor | undefined;
  let skipUpdate = false;

  onMount(() => {
    editor = new Editor({
      element,
      extensions: [
        StarterKit,
        Markdown.configure({
          html: false,
          transformPastedText: true,
          transformCopiedText: true,
        }),
      ],
      content: value,
      editorProps: {
        attributes: {
          class: "prose prose-invert prose-sm max-w-none p-4 outline-none min-h-full",
        },
      },
      onUpdate: ({ editor: e }) => {
        skipUpdate = true;
        const md = getMarkdownFromEditor(e);
        onchange(md);
      },
    });
  });

  $effect(() => {
    if (editor && !skipUpdate) {
      const current = getMarkdownFromEditor(editor);
      if (current !== value) {
        editor.commands.setContent(value);
      }
    }
    skipUpdate = false;
  });

  onDestroy(() => {
    editor?.destroy();
  });
</script>

<div
  bind:this={element}
  class="h-full overflow-y-auto rounded-md border border-border-primary bg-bg-secondary"
></div>

<style>
  :global(.tiptap) {
    min-height: 100%;
    color: var(--color-text-primary);
    font-family: var(--font-sans);
    font-size: 13px;
    line-height: 1.6;
  }

  :global(.tiptap h1) {
    font-size: 1.5em;
    font-weight: 700;
    margin: 1em 0 0.5em;
    color: var(--color-text-primary);
  }

  :global(.tiptap h2) {
    font-size: 1.25em;
    font-weight: 600;
    margin: 0.8em 0 0.4em;
    color: var(--color-text-primary);
  }

  :global(.tiptap h3) {
    font-size: 1.1em;
    font-weight: 600;
    margin: 0.6em 0 0.3em;
    color: var(--color-text-primary);
  }

  :global(.tiptap p) {
    margin: 0.4em 0;
  }

  :global(.tiptap code) {
    background: var(--color-bg-tertiary);
    padding: 0.15em 0.3em;
    border-radius: 3px;
    font-family: var(--font-mono);
    font-size: 0.9em;
  }

  :global(.tiptap pre) {
    background: var(--color-bg-tertiary);
    border-radius: 6px;
    padding: 0.75em 1em;
    margin: 0.5em 0;
    overflow-x: auto;
  }

  :global(.tiptap pre code) {
    background: none;
    padding: 0;
  }

  :global(.tiptap ul, .tiptap ol) {
    padding-left: 1.5em;
    margin: 0.4em 0;
  }

  :global(.tiptap blockquote) {
    border-left: 3px solid var(--color-border-secondary);
    padding-left: 1em;
    margin: 0.5em 0;
    color: var(--color-text-secondary);
  }

  :global(.tiptap hr) {
    border: none;
    border-top: 1px solid var(--color-border-primary);
    margin: 1em 0;
  }

  :global(.tiptap a) {
    color: var(--color-accent);
    text-decoration: underline;
  }

  :global(.tiptap table) {
    border-collapse: collapse;
    width: 100%;
    margin: 0.5em 0;
  }

  :global(.tiptap th, .tiptap td) {
    border: 1px solid var(--color-border-secondary);
    padding: 0.4em 0.6em;
    text-align: left;
  }

  :global(.tiptap th) {
    background: var(--color-bg-tertiary);
    font-weight: 600;
  }
</style>
