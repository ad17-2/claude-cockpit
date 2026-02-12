# Claude Cockpit

A desktop dashboard for [Claude Code](https://docs.anthropic.com/en/docs/claude-code). Reads from `~/.claude/` and provides a unified GUI to browse, monitor, edit, and manage projects, sessions, conversations, settings, and entities.

Built with Tauri + SvelteKit.

## Features

### Dashboard (`/`)
- Stats overview — projects, sessions, messages, tokens, entities, days active
- Daily activity chart (last 30 days)
- Token usage breakdown by model (input/output/cache)
- Hourly activity heatmap
- Recent conversations and projects list
- Project management with delete support

### Sessions (`/sessions`)
- Live view of active Claude Code sessions (auto-refreshes every 3s)
- Expandable message tail per session (polls every 2s)
- Activity indicator for sessions active within the last 30s
- Shows role, timestamp, token counts, and full message content

### CLAUDE.md Editor (`/claude-md`)
- CodeMirror markdown editor with syntax highlighting and One Dark theme
- Global (`~/.claude/CLAUDE.md`) and per-project scope switching
- Unsaved changes tracking, `Cmd/Ctrl+S` to save
- Auto-reload on external file changes

### Settings (`/settings`)
- Manage `settings.json` and `settings.local.json` (global + per-project)
- Effective settings view — see computed merged settings per project
- Permissions manager (allow/deny/ask rules) with add/remove
- MCP servers list with command, args, and env details
- Raw JSON view for non-permission settings

### Entities (`/entities`)
- Browse, create, edit, and delete across all entity types: agents, rules, commands, skills, hooks
- YAML frontmatter + markdown body editing
- Global and per-project scope support

### History (`/history`)
- **Conversations** — full-text search, project filtering, expandable message previews, inline delete
- **Commands** — terminal command execution history with timestamps

### Command Palette (`Cmd/Ctrl+K`)
- Fuzzy search across pages, projects, entities, and conversations
- Keyboard navigation (arrows, enter, escape)

### System Tray
- Minimize to tray with show/quit menu
- Desktop notifications on session completion

### Keyboard Shortcuts

| Shortcut | Action |
|----------|--------|
| `Cmd/Ctrl+1-6` | Jump to page |
| `Cmd/Ctrl+K` | Command palette |
| `Cmd/Ctrl+S` | Save in editors |
| `Esc` | Close palette/dialogs |

## Tech Stack

| Layer | Technology |
|-------|-----------|
| Desktop | Tauri 2 (Rust) |
| Frontend | SvelteKit 2, Svelte 5, TypeScript |
| Styling | Tailwind CSS 4 |
| Editor | CodeMirror 6 |
| Icons | Lucide Svelte |
| Build | Vite 6, Cargo |

## Prerequisites

- **Rust** 1.70+ — [install](https://rustup.rs/)
- **Node.js** 18+
- **pnpm** — `npm install -g pnpm`
- Platform-specific Tauri dependencies — [see Tauri docs](https://v2.tauri.app/start/prerequisites/)

## Setup

```bash
pnpm install
```

## Development

```bash
pnpm tauri dev
```

Starts the Vite dev server on port 1420 and launches the Tauri window.

## Build

```bash
pnpm tauri build
```

Produces platform-specific binaries in `src-tauri/target/release/bundle/`.

## Other Scripts

```bash
pnpm dev            # Frontend-only dev server (no Tauri window)
pnpm build          # Frontend-only build
pnpm check          # TypeScript + Svelte type checking
pnpm check:watch    # Type checking in watch mode
```

## Project Structure

```
src/
  routes/             # SvelteKit pages
    +page.svelte      #   Dashboard
    sessions/          #   Live sessions
    claude-md/         #   CLAUDE.md editor
    settings/          #   Settings manager
    entities/          #   Entities manager
    history/           #   Conversation & command history
  lib/
    components/       # Shared UI (Sidebar, CommandPalette, editors, etc.)
    commands/         # Tauri command wrappers
    utils/            # Shared formatting and helpers
  app.css             # Tailwind theme + custom styles

src-tauri/
  src/
    commands/         # Rust backend (projects, sessions, history, entities, settings, etc.)
    lib.rs            # Tauri setup + command registration
    main.rs
  tauri.conf.json     # App config (1200x800 window, tray, bundle settings)
```

## Data

All data is read from the standard Claude Code directory (`~/.claude/`):

| Data | Path |
|------|------|
| Projects | `~/.claude/projects/` |
| Conversations | `~/.claude/projects/*/*.jsonl` |
| Global instructions | `~/.claude/CLAUDE.md` |
| Settings | `~/.claude/settings.json`, `settings.local.json` |
| Entities | `~/.claude/{agents,rules,commands,skills,hooks}/` |
| Usage stats | `~/.claude/statsig-cache.json` |
| Command history | `~/.claude/.history` |

## License

MIT
