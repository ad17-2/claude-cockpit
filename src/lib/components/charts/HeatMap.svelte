<script lang="ts">
  let { data = [] }: { data: number[] } = $props();

  let maxVal = $derived(Math.max(...data, 1));

  function getOpacity(value: number): number {
    if (value === 0) return 0.05;
    return 0.1 + (value / maxVal) * 0.9;
  }
</script>

<div class="flex gap-0.5">
  {#each Array(24) as _, hour}
    {@const value = data[hour] ?? 0}
    <div
      class="flex-1 rounded-sm"
      style="background-color: var(--color-accent); opacity: {getOpacity(value)}; min-height: 24px;"
      title="{hour}:00 — {value}"
    ></div>
  {/each}
</div>
<div class="mt-1 flex justify-between">
  <span class="text-[9px] text-text-tertiary">0</span>
  <span class="text-[9px] text-text-tertiary">6</span>
  <span class="text-[9px] text-text-tertiary">12</span>
  <span class="text-[9px] text-text-tertiary">18</span>
  <span class="text-[9px] text-text-tertiary">23</span>
</div>
