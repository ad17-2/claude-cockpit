<script lang="ts">
  import "../app.css";
  import { goto } from "$app/navigation";
  import Sidebar from "$lib/components/Sidebar.svelte";
  import { navItems } from "$lib/navigation.svelte";

  let { children } = $props();

  function handleKeydown(e: KeyboardEvent): void {
    if (!e.metaKey && !e.ctrlKey) return;

    const index = parseInt(e.key) - 1;
    if (index >= 0 && index < navItems.length) {
      e.preventDefault();
      goto(navItems[index].href);
    }
  }
</script>

<svelte:window onkeydown={handleKeydown} />

<div class="flex h-screen">
  <Sidebar />
  <main class="flex-1 overflow-y-auto">
    {@render children()}
  </main>
</div>
