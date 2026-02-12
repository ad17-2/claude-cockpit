<script lang="ts">
  import { onMount } from "svelte";
  import { listProjects, type ProjectInfo } from "$lib/commands/projects";
  import { listConversations, type ConversationMeta } from "$lib/commands/history";
  import { listEntities } from "$lib/commands/entities";
  import { startWatching } from "$lib/commands/watcher";
  import {
    FolderOpen,
    MessageSquare,
    Bot,
    FileText,
    Settings as SettingsIcon,
    ArrowRight,
  } from "lucide-svelte";

  let projects = $state<ProjectInfo[]>([]);
  let recentConversations = $state<ConversationMeta[]>([]);
  let entityCount = $state(0);
  let loading = $state(true);
  let error = $state<string | null>(null);

  function formatTimestamp(ts: string): string {
    if (!ts) return "";
    try {
      const d = new Date(ts);
      return d.toLocaleDateString(undefined, {
        month: "short",
        day: "numeric",
        hour: "2-digit",
        minute: "2-digit",
      });
    } catch {
      return ts;
    }
  }

  function decodeProject(encoded: string): string {
    if (!encoded || !encoded.startsWith("-")) return encoded;
    return encoded.substring(1).replace(/-/g, "/");
  }

  onMount(async () => {
    startWatching().catch(() => {});

    try {
      const [p, c, agents, rules, commands, skills, hooks] = await Promise.all([
        listProjects(),
        listConversations(),
        listEntities("agents"),
        listEntities("rules"),
        listEntities("commands"),
        listEntities("skills"),
        listEntities("hooks"),
      ]);
      projects = p;
      recentConversations = c.slice(0, 5);
      entityCount = agents.length + rules.length + commands.length + skills.length + hooks.length;
    } catch (e) {
      error = String(e);
    } finally {
      loading = false;
    }
  });

  interface StatCard {
    label: string;
    value: number;
    icon: typeof FolderOpen;
    href: string;
  }

  let stats = $derived<StatCard[]>([
    { label: "Projects", value: projects.length, icon: FolderOpen, href: "/" },
    { label: "Conversations", value: recentConversations.length, icon: MessageSquare, href: "/history" },
    { label: "Entities", value: entityCount, icon: Bot, href: "/entities" },
  ]);
</script>

<div class="flex h-full flex-col">
  <div class="border-b border-border-primary px-4 py-2">
    <h1 class="text-sm font-semibold text-text-primary">Dashboard</h1>
  </div>

  {#if error}
    <div class="px-4 py-3">
      <p class="text-sm text-danger">{error}</p>
    </div>
  {/if}

  <div class="flex-1 overflow-y-auto">
    {#if loading}
      <div class="flex h-full items-center justify-center">
        <p class="text-sm text-text-tertiary">Loading...</p>
      </div>
    {:else}
      <div class="space-y-6 p-4">
        <div class="grid grid-cols-3 gap-3">
          {#each stats as stat}
            <a
              href={stat.href}
              class="rounded-lg border border-border-primary bg-bg-secondary p-4 transition-colors hover:bg-bg-hover"
            >
              <div class="flex items-center gap-2 text-text-tertiary">
                <stat.icon size={14} />
                <span class="text-xs font-medium uppercase tracking-wider">{stat.label}</span>
              </div>
              <p class="mt-2 text-2xl font-semibold text-text-primary">{stat.value}</p>
            </a>
          {/each}
        </div>

        <div>
          <div class="mb-3 flex items-center justify-between">
            <h2 class="text-xs font-semibold uppercase tracking-wider text-text-tertiary">Quick Links</h2>
          </div>
          <div class="grid grid-cols-2 gap-2">
            <a
              href="/claude-md"
              class="flex items-center gap-3 rounded-md border border-border-primary bg-bg-secondary px-3 py-2.5 transition-colors hover:bg-bg-hover"
            >
              <FileText size={16} class="text-text-tertiary" />
              <div class="flex-1">
                <p class="text-sm font-medium text-text-primary">CLAUDE.md</p>
                <p class="text-xs text-text-tertiary">Edit instructions</p>
              </div>
              <ArrowRight size={14} class="text-text-tertiary" />
            </a>
            <a
              href="/settings"
              class="flex items-center gap-3 rounded-md border border-border-primary bg-bg-secondary px-3 py-2.5 transition-colors hover:bg-bg-hover"
            >
              <SettingsIcon size={16} class="text-text-tertiary" />
              <div class="flex-1">
                <p class="text-sm font-medium text-text-primary">Settings</p>
                <p class="text-xs text-text-tertiary">Manage permissions</p>
              </div>
              <ArrowRight size={14} class="text-text-tertiary" />
            </a>
            <a
              href="/entities"
              class="flex items-center gap-3 rounded-md border border-border-primary bg-bg-secondary px-3 py-2.5 transition-colors hover:bg-bg-hover"
            >
              <Bot size={16} class="text-text-tertiary" />
              <div class="flex-1">
                <p class="text-sm font-medium text-text-primary">Entities</p>
                <p class="text-xs text-text-tertiary">Agents, rules, hooks</p>
              </div>
              <ArrowRight size={14} class="text-text-tertiary" />
            </a>
            <a
              href="/history"
              class="flex items-center gap-3 rounded-md border border-border-primary bg-bg-secondary px-3 py-2.5 transition-colors hover:bg-bg-hover"
            >
              <MessageSquare size={16} class="text-text-tertiary" />
              <div class="flex-1">
                <p class="text-sm font-medium text-text-primary">History</p>
                <p class="text-xs text-text-tertiary">Browse conversations</p>
              </div>
              <ArrowRight size={14} class="text-text-tertiary" />
            </a>
          </div>
        </div>

        {#if recentConversations.length > 0}
          <div>
            <div class="mb-3 flex items-center justify-between">
              <h2 class="text-xs font-semibold uppercase tracking-wider text-text-tertiary">Recent Conversations</h2>
              <a href="/history" class="text-xs text-accent hover:text-accent-hover">View all</a>
            </div>
            <div class="space-y-1">
              {#each recentConversations as conv}
                <a
                  href="/history"
                  class="flex items-center gap-3 rounded-md border border-border-primary bg-bg-secondary px-3 py-2 transition-colors hover:bg-bg-hover"
                >
                  <MessageSquare size={14} class="shrink-0 text-text-tertiary" />
                  <div class="min-w-0 flex-1">
                    <p class="truncate text-sm text-text-primary">{conv.first_message_preview}</p>
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
            <h2 class="mb-3 text-xs font-semibold uppercase tracking-wider text-text-tertiary">Projects ({projects.length})</h2>
            <div class="space-y-1">
              {#each projects as project}
                <div class="flex items-center justify-between rounded-md border border-border-primary bg-bg-secondary px-3 py-2">
                  <div class="min-w-0">
                    <p class="text-sm font-medium text-text-primary">{project.name}</p>
                    <p class="truncate font-mono text-xs text-text-tertiary">{project.decoded_path}</p>
                  </div>
                  <div class="flex shrink-0 gap-2">
                    {#if project.has_claude_md}
                      <span class="rounded bg-accent-muted px-1.5 py-0.5 text-[11px] text-accent">CLAUDE.md</span>
                    {/if}
                    {#if project.has_settings}
                      <span class="rounded bg-accent-muted px-1.5 py-0.5 text-[11px] text-accent">settings</span>
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
