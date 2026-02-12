<script lang="ts">
  import { page } from "$app/state";
  import { navItems } from "$lib/navigation.svelte";

  function isActive(href: string): boolean {
    if (href === "/") return page.url.pathname === "/";
    return page.url.pathname.startsWith(href);
  }
</script>

<nav class="flex h-full w-52 flex-col border-r border-border-primary bg-bg-secondary">
  <div class="flex h-10 items-center border-b border-border-primary px-3">
    <span class="text-xs font-medium text-text-tertiary">claude-cockpit</span>
  </div>

  <div class="flex flex-1 flex-col gap-px px-1.5 py-2">
    {#each navItems as item, i}
      <a
        href={item.href}
        class="flex items-center gap-2 px-2 py-1.5 text-xs transition-colors
          {isActive(item.href)
            ? 'bg-bg-active text-accent'
            : 'text-text-secondary hover:bg-bg-hover hover:text-text-primary'}"
      >
        <span class="w-3 text-[10px] text-text-tertiary">{isActive(item.href) ? '>' : ''}</span>
        <item.icon size={14} strokeWidth={1.5} />
        <span class="flex-1 lowercase">{item.label}</span>
        <kbd class="text-[10px] text-text-tertiary">[{i + 1}]</kbd>
      </a>
    {/each}
  </div>
</nav>
