<script lang="ts">
  import { onMount } from "svelte";
  import { readStatsCache, type StatsCache, type DayStat, type ModelUsage } from "$lib/commands/usage";
  import { onFileChange } from "$lib/commands/watcher";
  import { formatNumber } from "$lib/utils/format";
  import BarChart from "$lib/components/charts/BarChart.svelte";
  import HeatMap from "$lib/components/charts/HeatMap.svelte";
  import { BarChart3, MessageSquare, Zap, Calendar } from "lucide-svelte";

  let stats = $state<StatsCache | null>(null);
  let loading = $state(true);
  let error = $state<string | null>(null);

  let totalSessions = $derived(stats?.total_sessions ?? 0);
  let totalMessages = $derived(stats?.total_messages ?? 0);
  let totalTokens = $derived(stats?.total_tokens ?? 0);
  let daysActive = $derived(stats?.days_active ?? 0);

  let dailyData = $derived(
    stats?.daily_stats
      ? Object.entries(stats.daily_stats)
          .map(([date, stat]) => ({ date, value: stat.messages }))
          .sort((a, b) => a.date.localeCompare(b.date))
          .slice(-30)
      : []
  );

  let modelBreakdown = $derived(
    stats?.model_usage
      ? Object.entries(stats.model_usage)
          .map(([name, usage]) => ({
            name,
            input: usage.input_tokens,
            output: usage.output_tokens,
            cache_read: usage.cache_read_tokens,
            cache_creation: usage.cache_creation_tokens,
            total: usage.input_tokens + usage.output_tokens + usage.cache_read_tokens + usage.cache_creation_tokens,
          }))
          .sort((a, b) => b.total - a.total)
      : []
  );

  let hourlyActivity = $derived(stats?.hourly_activity ?? []);
  let longestSession = $derived(stats?.longest_session ?? null);

  async function loadStats(): Promise<void> {
    loading = true;
    error = null;
    try {
      stats = await readStatsCache();
    } catch (e) {
      error = String(e);
    } finally {
      loading = false;
    }
  }

  onMount(() => {
    let unlisten: (() => void) | undefined;

    (async () => {
      await loadStats();
      unlisten = await onFileChange("history-changed", loadStats);
    })();

    return () => unlisten?.();
  });

  interface StatCard {
    label: string;
    value: string;
    icon: typeof BarChart3;
  }

  let statCards = $derived<StatCard[]>([
    { label: "sessions", value: formatNumber(totalSessions), icon: BarChart3 },
    { label: "messages", value: formatNumber(totalMessages), icon: MessageSquare },
    { label: "tokens", value: formatNumber(totalTokens), icon: Zap },
    { label: "days active", value: String(daysActive), icon: Calendar },
  ]);

  let totalModelTokens = $derived(modelBreakdown.reduce((sum, m) => sum + m.total, 0));
</script>

<div class="flex h-full flex-col">
  <div class="border-b border-border-primary px-4 py-2">
    <h1 class="text-xs font-medium text-text-secondary">// usage</h1>
  </div>

  {#if error}
    <div class="px-4 py-2">
      <p class="text-xs text-danger">{error}</p>
    </div>
  {/if}

  <div class="flex-1 overflow-y-auto">
    {#if loading}
      <div class="flex h-full items-center justify-center">
        <p class="text-xs text-text-tertiary">loading...</p>
      </div>
    {:else if !stats}
      <div class="flex h-full items-center justify-center">
        <div class="text-center">
          <BarChart3 size={24} class="mx-auto mb-2 text-text-tertiary" />
          <p class="text-xs text-text-tertiary">// no usage data found</p>
          <p class="mt-1 text-[10px] text-text-tertiary">stats-cache.json will appear after using Claude</p>
        </div>
      </div>
    {:else}
      <div class="space-y-5 p-4">
        <div class="grid grid-cols-4 gap-2">
          {#each statCards as card}
            <div class="border border-border-primary bg-bg-secondary p-3">
              <div class="flex items-center gap-2 text-text-tertiary">
                <card.icon size={13} />
                <span class="text-[11px] uppercase tracking-wider">// {card.label}</span>
              </div>
              <p class="mt-1.5 text-xl font-medium text-text-primary">{card.value}</p>
            </div>
          {/each}
        </div>

        {#if dailyData.length > 0}
          <div>
            <h2 class="mb-2 text-[11px] uppercase tracking-wider text-text-tertiary">// daily activity (last 30 days)</h2>
            <div class="border border-border-primary bg-bg-secondary p-3">
              <BarChart data={dailyData} />
            </div>
          </div>
        {/if}

        {#if modelBreakdown.length > 0}
          <div>
            <h2 class="mb-2 text-[11px] uppercase tracking-wider text-text-tertiary">// model breakdown</h2>
            <div class="space-y-px">
              {#each modelBreakdown as model}
                <div class="flex items-center justify-between border border-border-primary bg-bg-secondary px-3 py-2">
                  <span class="text-xs text-text-primary">{model.name}</span>
                  <div class="flex items-center gap-3">
                    <span class="text-[10px] text-text-tertiary">in: {formatNumber(model.input)}</span>
                    <span class="text-[10px] text-text-tertiary">out: {formatNumber(model.output)}</span>
                    <span class="text-[10px] text-text-tertiary">cache: {formatNumber(model.cache_read)}</span>
                    <span class="text-[10px] text-text-secondary">{totalModelTokens > 0 ? ((model.total / totalModelTokens) * 100).toFixed(1) : 0}%</span>
                  </div>
                </div>
              {/each}
            </div>
          </div>
        {/if}

        {#if hourlyActivity.length > 0}
          <div>
            <h2 class="mb-2 text-[11px] uppercase tracking-wider text-text-tertiary">// hourly activity</h2>
            <div class="border border-border-primary bg-bg-secondary p-3">
              <HeatMap data={hourlyActivity} />
            </div>
          </div>
        {/if}

        {#if longestSession}
          <div>
            <h2 class="mb-2 text-[11px] uppercase tracking-wider text-text-tertiary">// longest session</h2>
            <div class="border border-border-primary bg-bg-secondary px-3 py-2">
              <div class="flex items-center gap-3">
                <span class="text-xs text-text-primary">{longestSession.duration_mins} mins</span>
                <span class="text-[10px] text-text-tertiary">{longestSession.message_count} messages</span>
                <span class="text-[10px] text-text-tertiary">{longestSession.date}</span>
              </div>
            </div>
          </div>
        {/if}
      </div>
    {/if}
  </div>
</div>
