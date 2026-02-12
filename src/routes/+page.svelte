<script lang="ts">
  import { onMount } from "svelte";
  import { listProjects, deleteProject, type ProjectInfo } from "$lib/commands/projects";
  import { listConversations, type ConversationMeta } from "$lib/commands/history";
  import { listEntities } from "$lib/commands/entities";
  import { readStatsCache, type StatsCache } from "$lib/commands/usage";
  import { startWatching, onFileChange } from "$lib/commands/watcher";
  import { formatTimestamp, formatNumber, decodeProject } from "$lib/utils/format";
  import BarChart from "$lib/components/charts/BarChart.svelte";
  import HeatMap from "$lib/components/charts/HeatMap.svelte";
  import {
    FolderOpen,
    MessageSquare,
    Bot,
    FileText,
    Settings as SettingsIcon,
    BarChart3,
    Zap,
    Calendar,
    Trash2,
  } from "lucide-svelte";

  let projects = $state<ProjectInfo[]>([]);
  let recentConversations = $state<ConversationMeta[]>([]);
  let entityCount = $state(0);
  let stats = $state<StatsCache | null>(null);
  let loading = $state(true);
  let error = $state<string | null>(null);

  let dailyData = $derived(
    stats?.dailyActivity
      ? stats.dailyActivity
          .map((d) => ({ date: d.date, value: d.messageCount }))
          .sort((a, b) => a.date.localeCompare(b.date))
          .slice(-30)
      : []
  );

  let modelBreakdown = $derived(
    stats?.modelUsage
      ? Object.entries(stats.modelUsage)
          .map(([name, usage]) => {
            const total =
              usage.inputTokens + usage.outputTokens + usage.cacheReadInputTokens + usage.cacheCreationInputTokens;
            return { name, input: usage.inputTokens, output: usage.outputTokens, cache_read: usage.cacheReadInputTokens, total };
          })
          .sort((a, b) => b.total - a.total)
      : []
  );

  let totalModelTokens = $derived(modelBreakdown.reduce((sum, m) => sum + m.total, 0));

  let hourlyActivity = $derived.by(() => {
    if (!stats?.hourCounts) return [];
    const arr = new Array(24).fill(0);
    for (const [hour, count] of Object.entries(stats.hourCounts)) {
      arr[Number(hour)] = count;
    }
    return arr;
  });

  async function handleDeleteProject(encodedPath: string): Promise<void> {
    try {
      await deleteProject(encodedPath);
      await loadAll();
    } catch (e) {
      error = String(e);
    }
  }

  async function loadAll(): Promise<void> {
    const [p, c, agents, rules, cmds, skills, hooks, s] = await Promise.all([
      listProjects(),
      listConversations(),
      listEntities("agents"),
      listEntities("rules"),
      listEntities("commands"),
      listEntities("skills"),
      listEntities("hooks"),
      readStatsCache(),
    ]);
    projects = p;
    recentConversations = c.slice(0, 5);
    entityCount = agents.length + rules.length + cmds.length + skills.length + hooks.length;
    stats = s;
  }

  onMount(() => {
    let unlisteners: (() => void)[] = [];

    startWatching().catch(() => {});

    (async () => {
      try {
        await loadAll();
      } catch (e) {
        error = String(e);
      } finally {
        loading = false;
      }

      unlisteners = await Promise.all([
        onFileChange("claude-md-changed", loadAll),
        onFileChange("settings-changed", loadAll),
        onFileChange("entity-changed", loadAll),
        onFileChange("history-changed", loadAll),
      ]);
    })();

    return () => unlisteners.forEach((fn) => fn());
  });

  interface StatCard {
    label: string;
    value: string;
    icon: typeof FolderOpen;
    href?: string;
  }

  let statCards = $derived<StatCard[]>([
    { label: "projects", value: String(projects.length), icon: FolderOpen, href: "/" },
    { label: "sessions", value: formatNumber(stats?.totalSessions ?? 0), icon: BarChart3 },
    { label: "messages", value: formatNumber(stats?.totalMessages ?? 0), icon: MessageSquare, href: "/history" },
    { label: "tokens", value: formatNumber(totalModelTokens), icon: Zap },
    { label: "entities", value: String(entityCount), icon: Bot, href: "/entities" },
    { label: "days active", value: String(stats?.dailyActivity?.length ?? 0), icon: Calendar },
  ]);
</script>

<div class="flex h-full flex-col">
  <div class="border-b border-border-primary px-4 py-2">
    <h1 class="text-xs font-medium text-text-secondary">// dashboard</h1>
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
    {:else}
      <div class="space-y-5 p-4">
        {#snippet statCardContent(stat: StatCard)}
          <div class="flex items-center gap-2 text-text-tertiary">
            <stat.icon size={13} />
            <span class="text-[11px] uppercase tracking-wider">// {stat.label}</span>
          </div>
          <p class="mt-1.5 text-xl font-medium text-text-primary">{stat.value}</p>
        {/snippet}

        <div class="grid grid-cols-3 gap-2">
          {#each statCards as stat}
            {#if stat.href}
              <a
                href={stat.href}
                class="border border-border-primary bg-bg-secondary p-3 transition-colors hover:bg-bg-hover"
              >
                {@render statCardContent(stat)}
              </a>
            {:else}
              <div class="border border-border-primary bg-bg-secondary p-3">
                {@render statCardContent(stat)}
              </div>
            {/if}
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

        <div>
          <h2 class="mb-2 text-[11px] uppercase tracking-wider text-text-tertiary">// quick links</h2>
          <div class="grid grid-cols-2 gap-1.5">
            <a
              href="/claude-md"
              class="flex items-center gap-2.5 border border-border-primary bg-bg-secondary px-3 py-2 transition-colors hover:bg-bg-hover"
            >
              <FileText size={14} class="text-text-tertiary" />
              <div class="flex-1">
                <p class="text-xs text-text-primary">> CLAUDE.md</p>
                <p class="text-[10px] text-text-tertiary">edit instructions</p>
              </div>
            </a>
            <a
              href="/settings"
              class="flex items-center gap-2.5 border border-border-primary bg-bg-secondary px-3 py-2 transition-colors hover:bg-bg-hover"
            >
              <SettingsIcon size={14} class="text-text-tertiary" />
              <div class="flex-1">
                <p class="text-xs text-text-primary">> settings</p>
                <p class="text-[10px] text-text-tertiary">manage permissions</p>
              </div>
            </a>
            <a
              href="/entities"
              class="flex items-center gap-2.5 border border-border-primary bg-bg-secondary px-3 py-2 transition-colors hover:bg-bg-hover"
            >
              <Bot size={14} class="text-text-tertiary" />
              <div class="flex-1">
                <p class="text-xs text-text-primary">> entities</p>
                <p class="text-[10px] text-text-tertiary">agents, rules, hooks</p>
              </div>
            </a>
            <a
              href="/history"
              class="flex items-center gap-2.5 border border-border-primary bg-bg-secondary px-3 py-2 transition-colors hover:bg-bg-hover"
            >
              <MessageSquare size={14} class="text-text-tertiary" />
              <div class="flex-1">
                <p class="text-xs text-text-primary">> history</p>
                <p class="text-[10px] text-text-tertiary">browse conversations</p>
              </div>
            </a>
          </div>
        </div>

        {#if recentConversations.length > 0}
          <div>
            <div class="mb-2 flex items-center justify-between">
              <h2 class="text-[11px] uppercase tracking-wider text-text-tertiary">// recent conversations</h2>
              <a href="/history" class="text-[11px] text-accent hover:text-accent-hover">view all</a>
            </div>
            <div class="space-y-px">
              {#each recentConversations as conv}
                <a
                  href="/history"
                  class="flex items-center gap-2.5 border border-border-primary bg-bg-secondary px-3 py-1.5 transition-colors hover:bg-bg-hover"
                >
                  <span class="text-[10px] text-accent">></span>
                  <div class="min-w-0 flex-1">
                    <p class="truncate text-xs text-text-primary">{conv.first_message_preview}</p>
                    <div class="flex items-center gap-2">
                      <span class="text-[10px] text-text-tertiary">{decodeProject(conv.project)}</span>
                      <span class="text-[10px] text-text-tertiary">{conv.message_count} msgs</span>
                      {#if conv.timestamp}
                        <span class="text-[10px] text-text-tertiary">{formatTimestamp(conv.timestamp)}</span>
                      {/if}
                    </div>
                  </div>
                </a>
              {/each}
            </div>
          </div>
        {/if}

        {#if projects.length > 0}
          <div>
            <h2 class="mb-2 text-[11px] uppercase tracking-wider text-text-tertiary">// projects ({projects.length})</h2>
            <div class="space-y-px">
              {#each projects as project}
                <div class="flex items-center justify-between border border-border-primary bg-bg-secondary px-3 py-1.5">
                  <div class="min-w-0">
                    <p class="text-xs text-text-primary">{project.name}</p>
                    <p class="truncate text-[10px] text-text-tertiary">{project.decoded_path}</p>
                  </div>
                  <div class="flex shrink-0 items-center gap-1.5">
                    {#if project.has_claude_md}
                      <span class="bg-accent-muted px-1.5 py-0.5 text-[10px] text-accent">CLAUDE.md</span>
                    {/if}
                    {#if project.has_settings}
                      <span class="bg-accent-muted px-1.5 py-0.5 text-[10px] text-accent">settings</span>
                    {/if}
                    {#if !project.is_root}
                      <button
                        onclick={() => handleDeleteProject(project.encoded_path)}
                        class="p-1 text-text-tertiary transition-colors hover:text-danger"
                      >
                        <Trash2 size={12} />
                      </button>
                    {/if}
                  </div>
                </div>
              {/each}
            </div>
          </div>
        {/if}
      </div>
    {/if}
  </div>
</div>
