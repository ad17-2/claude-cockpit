# Claude Cockpit

A desktop dashboard for managing Claude AI projects, conversations, settings, and entities. Built with Tauri + SvelteKit.

Claude Cockpit reads from `~/.claude/` — the local directory where Claude Code stores its projects, conversation history, settings, and configuration files — and provides a unified GUI to browse, edit, and manage everything in one place.

## Features

### Dashboard (`/`)
Overview with stats (total projects, conversations, entities), recent conversation list, and quick navigation links.

### CLAUDE.md Editor (`/claude-md`)
Edit Claude system prompts and instructions with a CodeMirror-powered markdown editor. Supports both global (`~/.claude/CLAUDE.md`) and per-project scopes. Tracks unsaved changes.

### Settings Manager (`/settings`)
View and manage Claude permissions (allow/deny/ask rules). Switch between `settings.json` and `settings.local.json`. Raw JSON editing for full control.

### Entities Manager (`/entities`)
Browse, create, edit, and delete Claude entities across all types:
- **Agents** — custom agent definitions
- **Rules** — instruction rules
- **Commands** — slash commands
- **Skills** — skill definitions
- **Hooks** — event hooks

Each entity supports YAML frontmatter + markdown body editing.

### Conversation History (`/history`)
Browse past conversations with full-text search, project filtering, and inline delete. Lists conversations with preview, message count, and timestamp.

### Navigation
Keyboard shortcuts `Cmd/Ctrl + 1-5` for quick page switching. Collapsible sidebar.

## Tech Stack

| Layer | Technology |
|-------|-----------|
| Desktop | Tauri 2.x (Rust) |
| Frontend | SvelteKit 5, Svelte 5, TypeScript |
| Styling | Tailwind CSS 4, custom dark theme |
| Editors | CodeMirror 6 (code), TipTap 3 (rich text) |
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
  routes/             # SvelteKit pages (dashboard, claude-md, settings, entities, history)
  lib/
    components/       # Shared UI (Sidebar, MarkdownEditor, CodeMirrorEditor, etc.)
    commands/         # Tauri command wrappers (projects, history, entities, settings, etc.)
    navigation.svelte.ts
    tauri.ts          # Tauri invoke helper
  app.css             # Tailwind theme + custom styles
  app.html

src-tauri/
  src/
    commands/         # Rust backend (projects, history, entities, settings, claude_md, watcher)
    lib.rs            # Tauri setup + command registration
    main.rs
  tauri.conf.json     # App config (window 1200x800, bundle settings)
  Cargo.toml
```

## Data Locations

All data is read from the standard Claude Code directory:

| Data | Path |
|------|------|
| Projects | `~/.claude/projects/` |
| Conversations | `~/.claude/projects/*/` (`.jsonl` files) |
| Global instructions | `~/.claude/CLAUDE.md` |
| Settings | `~/.claude/settings.json`, `~/.claude/settings.local.json` |
| Entities | `~/.claude/{agents,rules,commands,skills,hooks}/` |

## IDE Setup

[VS Code](https://code.visualstudio.com/) with:
- [Svelte](https://marketplace.visualstudio.com/items?itemName=svelte.svelte-vscode)
- [Tauri](https://marketplace.visualstudio.com/items?itemName=tauri-apps.tauri-vscode)
- [rust-analyzer](https://marketplace.visualstudio.com/items?itemName=rust-lang.rust-analyzer)
