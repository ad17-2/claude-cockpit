<script lang="ts">
  import "../app.css";
  import { onMount } from "svelte";
  import { goto } from "$app/navigation";
  import Sidebar from "$lib/components/Sidebar.svelte";
  import CommandPalette from "$lib/components/CommandPalette.svelte";
  import { navItems } from "$lib/navigation.svelte";
  import { isTauri } from "$lib/tauri";

  let { children } = $props();
  let paletteOpen = $state(false);

  function handleKeydown(e: KeyboardEvent): void {
    if ((e.metaKey || e.ctrlKey) && e.key === "k") {
      e.preventDefault();
      paletteOpen = !paletteOpen;
      return;
    }

    if (!e.metaKey && !e.ctrlKey) return;

    const index = parseInt(e.key) - 1;
    if (index >= 0 && index < navItems.length) {
      e.preventDefault();
      goto(navItems[index].href);
    }
  }

  onMount(() => {
    let unlisten: (() => void) | undefined;

    if (!isTauri()) return;

    (async () => {
      const { listen } = await import("@tauri-apps/api/event");
      unlisten = await listen<string>("session-completed", async (event) => {
        try {
          const { sendNotification, isPermissionGranted, requestPermission } = await import("@tauri-apps/plugin-notification");
          let granted = await isPermissionGranted();
          if (!granted) {
            const perm = await requestPermission();
            granted = perm === "granted";
          }
          if (granted) {
            sendNotification({
              title: "Session Completed",
              body: `Claude session finished${event.payload ? `: ${event.payload}` : ""}`,
            });
          }
        } catch {}
      });
    })();

    return () => unlisten?.();
  });
</script>

<svelte:window onkeydown={handleKeydown} />

<div class="flex h-screen">
  <Sidebar />
  <main class="flex-1 overflow-y-auto">
    {@render children()}
  </main>
</div>

<CommandPalette bind:open={paletteOpen} />
