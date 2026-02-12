<script lang="ts">
  let { data = [] }: { data: { date: string; value: number }[] } = $props();

  let maxValue = $derived(Math.max(...data.map((d) => d.value), 1));
  let barWidth = $derived(data.length > 0 ? Math.max(100 / data.length - 1, 2) : 10);
</script>

<svg viewBox="0 0 100 40" class="h-32 w-full" preserveAspectRatio="none">
  {#each data as d, i}
    {@const height = (d.value / maxValue) * 35}
    {@const x = (i / data.length) * 100}
    {@const width = barWidth}
    <rect
      {x}
      y={40 - height}
      {width}
      {height}
      fill="var(--color-accent)"
      opacity="0.8"
    >
      <title>{d.date}: {d.value}</title>
    </rect>
  {/each}
</svg>
<div class="mt-1 flex justify-between">
  {#if data.length > 0}
    <span class="text-[9px] text-text-tertiary">{data[0].date.slice(5)}</span>
    <span class="text-[9px] text-text-tertiary">{data[data.length - 1].date.slice(5)}</span>
  {/if}
</div>
