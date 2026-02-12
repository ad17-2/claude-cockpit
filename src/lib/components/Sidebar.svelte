<script lang="ts">
  import { page } from "$app/state";
  import { navItems } from "$lib/navigation.svelte";

  function isActive(href: string): boolean {
    if (href === "/") return page.url.pathname === "/";
    return page.url.pathname.startsWith(href);
  }
</script>

<nav class="flex h-full w-56 flex-col border-r border-border-primary bg-bg-secondary">
  <div class="flex h-12 items-center px-4">
    <span class="text-sm font-semibold text-text-primary">Claude Cockpit</span>
  </div>

  <div class="flex flex-1 flex-col gap-0.5 px-2 py-2">
    {#each navItems as item, i}
      <a
        href={item.href}
        class="flex items-center gap-2.5 rounded-md px-2.5 py-1.5 text-[13px] font-medium transition-colors
          {isActive(item.href)
            ? 'bg-bg-active text-text-primary'
            : 'text-text-secondary hover:bg-bg-hover hover:text-text-primary'}"
      >
        <item.icon size={16} strokeWidth={1.75} />
        <span class="flex-1">{item.label}</span>
        <kbd class="text-[10px] text-text-tertiary">{i + 1}</kbd>
      </a>
    {/each}
  </div>
</nav>
