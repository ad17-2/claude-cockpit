<script lang="ts">
  import { onMount } from "svelte";
  import { listProjects, type ProjectInfo } from "$lib/commands/projects";
  import { listConversations, type ConversationMeta } from "$lib/commands/history";
  import { listEntities } from "$lib/commands/entities";
  import { startWatching, onFileChange } from "$lib/commands/watcher";
  import {
    FolderOpen,
    MessageSquare,
    Bot,
    FileText,
    Settings as SettingsIcon,
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

  onMount(() => {
    let unlisteners: (() => void)[] = [];

    startWatching().catch(() => {});

    (async () => {
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

      const loadAll = async () => {
        try {
          const [p, c, agents, rules, cmds, skills, hooks] = await Promise.all([
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
          entityCount = agents.length + rules.length + cmds.length + skills.length + hooks.length;
        } catch {}
      };

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
    value: number;
    icon: typeof FolderOpen;
    href: string;
  }

  let stats = $derived<StatCard[]>([
    { label: "projects", value: projects.length, icon: FolderOpen, href: "/" },
    { label: "conversations", value: recentConversations.length, icon: MessageSquare, href: "/history" },
    { label: "entities", value: entityCount, icon: Bot, href: "/entities" },
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
        <div class="grid grid-cols-3 gap-2">
          {#each stats as stat}
            <a
              href={stat.href}
              class="border border-border-primary bg-bg-secondary p-3 transition-colors hover:bg-bg-hover"
            >
              <div class="flex items-center gap-2 text-text-tertiary">
                <stat.icon size={13} />
                <span class="text-[11px] uppercase tracking-wider">// {stat.label}</span>
              </div>
              <p class="mt-1.5 text-xl font-medium text-text-primary">{stat.value}</p>
            </a>
          {/each}
        </div>

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
                  <div class="flex shrink-0 gap-1.5">
                    {#if project.has_claude_md}
                      <span class="bg-accent-muted px-1.5 py-0.5 text-[10px] text-accent">CLAUDE.md</span>
                    {/if}
                    {#if project.has_settings}
                      <span class="bg-accent-muted px-1.5 py-0.5 text-[10px] text-accent">settings</span>
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
