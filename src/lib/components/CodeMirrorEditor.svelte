<script lang="ts">
  import { onMount, onDestroy } from "svelte";
  import { EditorView, keymap, lineNumbers, highlightActiveLine, drawSelection } from "@codemirror/view";
  import { EditorState } from "@codemirror/state";
  import { markdown } from "@codemirror/lang-markdown";
  import { oneDark } from "@codemirror/theme-one-dark";
  import { defaultKeymap, indentWithTab } from "@codemirror/commands";
  import { syntaxHighlighting, defaultHighlightStyle } from "@codemirror/language";

  interface Props {
    value: string;
    onchange: (value: string) => void;
  }

  let { value, onchange }: Props = $props();

  let container: HTMLDivElement;
  let view: EditorView | undefined;
  let skipUpdate = false;

  onMount(() => {
    const updateListener = EditorView.updateListener.of((update) => {
      if (update.docChanged) {
        skipUpdate = true;
        onchange(update.state.doc.toString());
      }
    });

    const state = EditorState.create({
      doc: value,
      extensions: [
        lineNumbers(),
        highlightActiveLine(),
        drawSelection(),
        markdown(),
        oneDark,
        syntaxHighlighting(defaultHighlightStyle),
        keymap.of([...defaultKeymap, indentWithTab]),
        updateListener,
        EditorView.theme({
          "&": {
            height: "100%",
            fontSize: "13px",
          },
          ".cm-scroller": {
            fontFamily: "var(--font-mono)",
          },
          ".cm-content": {
            padding: "16px 0",
          },
        }),
      ],
    });

    view = new EditorView({
      state,
      parent: container,
    });
  });

  $effect(() => {
    if (view && !skipUpdate) {
      const current = view.state.doc.toString();
      if (current !== value) {
        view.dispatch({
          changes: { from: 0, to: current.length, insert: value },
        });
      }
    }
    skipUpdate = false;
  });

  onDestroy(() => {
    view?.destroy();
  });
</script>

<div bind:this={container} class="h-full overflow-hidden rounded-md border border-border-primary"></div>
