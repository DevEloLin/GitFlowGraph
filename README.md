<div align="center">

<img src="docs/assets/logo.png" alt="GitFlowGraph" width="88" />

# GitFlowGraph

**A Git visualisation workspace built for Zed**

Understand your repository's history without leaving the editor — commit graph, smart diff, release management — all in one panel.

**English** &nbsp;·&nbsp; [简体中文](README.zh-CN.md)

[![Zed Extension](https://img.shields.io/badge/Zed-Extension-1f6feb?labelColor=0d1117)](https://zed.dev/extensions?q=gitflowgraph)
[![Version](https://img.shields.io/badge/version-0.1.0-238636?labelColor=0d1117)](https://github.com/DevEloLin/GitFlowGraph/releases)
[![License: MIT](https://img.shields.io/badge/license-MIT-1f6feb?labelColor=0d1117)](LICENSE)
[![Platform](https://img.shields.io/badge/platform-macOS%20%7C%20Linux%20%7C%20Windows-6e7681?labelColor=0d1117)](https://github.com/DevEloLin/GitFlowGraph/releases)

<br/>

<img src="docs/assets/screenshot-graph.png" alt="GitFlowGraph — Git commit graph" width="900" />

<sub>Lane-aware commit graph with branch labels and inline diff panel</sub>

<br/>

</div>

---

## Overview

GitFlowGraph is a [Zed](https://zed.dev) extension that embeds a complete Git visualisation workspace into the editor. It replaces the constant context-switching between terminal, browser Git UIs, and a separate IDE — you browse history, understand changes, and manage releases inside one panel.

The extension itself (this repository) is published under the **MIT License**. The runtime engine that drives the commit graph, Smart Diff, and release analytics ships as a precompiled binary distributed separately.

---

## Commit Graph

An interactive multi-lane commit graph that visualises the full repository history.

- **SVG lane rendering** — each branch has its own colour, merge commits drawn as Bézier curves
- **HEAD highlight** — the current checkout sits inside a glowing ring
- **Branch and tag labels** — coloured capsules pinned to the relevant commits
- **Environment deployment badges (⬡)** — marks commits deployed to named environments
- **Infrastructure-change markers (⚠️)** — auto-detects `infra:`, `chore(k8s):`, `terraform:`, `helm:` prefixes
- **Skeleton loading state** — placeholder while data streams in

### Filters

- Plain-text or regular-expression search of commit messages
- Filter by author, branch, date range, or file path
- Named filter presets — save and recall any combination of the above

### Commit context menu

| Action | What it does |
|---|---|
| Copy hash / message / author | Writes to the clipboard |
| Checkout | Switch to the commit or branch |
| Create branch | Branch from any commit |
| Create tag | Annotated or lightweight |
| Cherry-pick | Apply to the current branch |
| Revert | Generate a revert commit |
| Merge | Merge a branch into HEAD |
| Push | Push branch to remote |
| Rename / delete branch | Branch management |
| Delete tag | Tag management |
| Reset (soft / mixed / hard) | Move HEAD |

---

## Commit Detail Panel

Click any commit to open a side panel with full context.

- Complete metadata: hash, author, email, time, parent IDs
- File list with A / M / D / R badges and ± line counts
- Collapsible directory tree of changed files
- Binary file detection
- **Unified diff** — standard row-level view
- **Side-by-side diff** — synced scrolling with character-level inline highlights
- **File history** — every commit that touched the selected file (up to 50)

---

## Smart Diff

Three context-aware diff modes, all supporting side-by-side view and character-level inline highlighting.

### Development Diff

Review work in progress.

| Preset | What it shows |
|---|---|
| Working tree | Unstaged changes vs. staged |
| Staged | Staged changes vs. HEAD |
| Last commit | HEAD~1 → HEAD |
| vs origin/* | Branch divergence from base (three-dot merge-base) |
| Custom | Any base → head ref |

### Release Diff

Review what is about to ship.

- Pick any tag, branch, or commit as From / To
- Structured files (`.yaml`, `.yml`, `.json`, `.tf`, `.tfvars`) automatically show a **semantic analysis sidebar** — which keys were added, removed, modified, or moved, with old and new values

### Workflow Diff

Review CI/CD and infrastructure-automation changes.

- Filters automatically to automation-relevant files only
- Files grouped by category:

| Category | Match rules |
|---|---|
| GitHub Actions | `.github/workflows/`, `.github/actions/` |
| Docker | `Dockerfile*`, `docker-compose*` |
| Terraform | `*.tf`, `*.tfvars` |
| Kubernetes | `k8s/`, `kubernetes/`, `manifests/` |
| Helm | `helm/`, `charts/` |
| CI pipelines | `.gitlab-ci.yml`, `Jenkinsfile`, `.circleci/`, `bitbucket-pipelines*` |

Structured files inside Workflow Diff also surface the semantic analysis sidebar.

---

## Release Workspace

A release-focused dashboard with five sub-tabs.

### Changelog

- Compare any two refs: ahead / behind counts
- **Release-risk score** (0–100, low / medium / high) with per-file risk factors
- Changelog grouped by category: Breaking Changes · Features · Bug Fixes · Infrastructure · Other
- Toggle between structured view and raw Markdown

### Actions

Automated pre-flight checks:

- **Hotfix sync** — scan every `hotfix/*` branch and show whether it has merged back into `main`, `develop`, `master`
- **Release-note check** — flag recent lightweight tags missing annotation messages

### Environments

Map named environments to a branch or tag, with:

- Side-by-side pipeline grid (e.g. Dev → Staging → Production)
- Live ahead / behind counts between adjacent environments

### Velocity Dashboard

Metrics computed directly from Git history — no external integrations.

| Metric | What it measures |
|---|---|
| Total releases | Number of version tags |
| Avg days between releases | Average release cadence |
| Hotfix count and rate | `hotfix/*` branches as a percentage of releases |
| Avg commits per release | Change volume per release |
| Recent releases table | Tag, date, commit count, days since previous |

### Hotfix Wizard

Four-step guided emergency-fix workflow:

1. **Pick production ref** — what's live right now
2. **Name the branch** — validated against Git naming rules
3. **Cherry-pick commits** — from any source branch
4. **Execute** — shows the generated script, runs it step by step with live status, then auto-runs the sync check on completion

---

## Worktrees

Manage Git worktrees inside the editor.

- List all worktrees: path, branch, HEAD commit, status (main / linked / locked)
- Add a worktree (any path, any branch)
- Remove a worktree

---

## Workspace Profiles

Named configurations for fast repo-context switching.

- Save repository paths as named profiles
- Switch profiles — notifies the host extension to reload the active repository
- The active profile is marked with a green indicator

---

## Installation

### Zed Extension Marketplace (recommended)

1. Open Zed
2. Press `⌘⇧X` to open **Extensions**
3. Search for **GitFlowGraph**
4. Click **Install**

The runtime binary (~15 MB) downloads automatically on first use.

### Manual / development mode

```bash
git clone https://github.com/DevEloLin/GitFlowGraph
cd GitFlowGraph
zed --install-extension .
```

---

## Using GitFlowGraph inside Zed — 28 slash commands

Zed's extension API doesn't expose webviews, so we can't embed the React UI directly in the editor. Instead, **every feature is reachable as a slash command** in the Assistant panel: tables, code blocks, badges, links render correctly, and that includes **all write operations** (`checkout`, `cherry-pick`, `merge`, `push`, etc. execute directly).

Open the Assistant panel with `⌘?` and start typing.

### Entry points

| Command | Purpose |
|---|---|
| `/gitflowgraph` or `/gitflowgraph-help` | Overview + the complete slash-command list |

### Read views

| Command | Purpose |
|---|---|
| `/gitflowgraph-status` | Working tree status (staged / unstaged / untracked) |
| `/gitflowgraph-graph [n]` | Most recent N commits + branch / tag annotations |
| `/gitflowgraph-branches` | All branches with HEAD marker |
| `/gitflowgraph-tags` | All tags + the commits they point at |
| `/gitflowgraph-worktrees` | Git worktree list |
| `/gitflowgraph-remotes` | Configured remotes |
| `/gitflowgraph-credentials` | Stored credentials (masked) |
| `/gitflowgraph-license` | License tier / machine usage / expiry |

### Range / release analytics

| Command | Purpose |
|---|---|
| `/gitflowgraph-diff <range>` | File-level diff summary |
| `/gitflowgraph-changelog <range>` | Auto-generated changelog |
| `/gitflowgraph-risk <range>` | Release-risk score + factors |
| `/gitflowgraph-launchpad <range>` | Compare + risk + checklist + changelog in one shot |
| `/gitflowgraph-velocity` | Release-cadence dashboard |
| `/gitflowgraph-file-history <path>` | Every commit that touched a file |

### Write operations (execute directly, **no browser needed**)

| Command | Purpose |
|---|---|
| `/gitflowgraph-checkout <ref>` | Switch branch / tag / commit |
| `/gitflowgraph-cherry-pick <sha>` | Cherry-pick |
| `/gitflowgraph-revert <sha>` | Create a revert commit |
| `/gitflowgraph-merge <branch>` | Merge into current HEAD |
| `/gitflowgraph-reset <soft\|mixed\|hard> <sha>` | Reset HEAD |
| `/gitflowgraph-fetch [remote]` | Fetch (default `origin`) |
| `/gitflowgraph-push <branch> [remote]` | Push |
| `/gitflowgraph-create-branch <name> [from]` | Create branch |
| `/gitflowgraph-delete-branch <name> [--force]` | Delete branch |
| `/gitflowgraph-create-tag <name> [from] [--annotated <msg>]` | Create tag |
| `/gitflowgraph-delete-tag <name>` | Delete tag |
| `/gitflowgraph-trial-start` | Start the free 30-day trial |

Each write command surfaces a `✓ <action>` success message (with a hint to `⌘R` the browser view) or an error explaining the failure cause.

### Optional full visual UI

For the lane graph, side-by-side diff, and Hotfix Wizard visual walkthrough, open **http://localhost:9876**. The slash commands talk to the same runtime, so the data is always consistent.

Drop the snippets from [`examples/tasks.json`](examples/tasks.json) and [`examples/keymap.json`](examples/keymap.json) into your `~/.config/zed/{tasks,keymap}.json` to one-shot the browser:

| Shortcut | Action |
|---|---|
| `⌘⇧G` | Open GitFlowGraph in the browser |
| `⌘⇧D` | Jump straight to the Smart Diff tab |
| `⌘⇧R` | Jump straight to the Release tab |

---

## LSP — in-editor git context

A built-in language server attaches to common source files (Rust, TypeScript, Python, Go, Java, Markdown, YAML, Terraform, and 30 more) and surfaces git context **inside the editor view**:

- **Hover** — git blame markdown for the line under the cursor: commit SHA, author, date, summary, link to drill into history
- **Inlay hints** — end-of-line ghost text per blame hunk (`3d ago · Alice — feat: add payment`); full tooltip resolved lazily on hover
- **Code lens** — file-header links to File History and Smart Diff

---

## Privacy

- **All Git operations run locally** — commit data, diff content, and file contents never leave your machine
- **Zero telemetry** — no analytics, no crash reports, no usage tracking

---

## Contributing

The extension itself (`src/`) is MIT-licensed and contributions are welcome.

```bash
git clone https://github.com/DevEloLin/GitFlowGraph
cd GitFlowGraph
cargo build --target wasm32-wasip1
```

Please open an issue to discuss any large changes before submitting a pull request.

---

## Support

| | Channel |
|---|---|
| 🐛 | [GitHub Issues](https://github.com/DevEloLin/GitFlowGraph/issues) — bug reports and feature requests |
| 📧 | [dev.elolin@gmail.com](mailto:dev.elolin@gmail.com) — direct contact |

---

## License

The Zed extension in this repository is published under the **[MIT License](LICENSE)**.

---

<div align="center">

Built for the Zed community &nbsp;·&nbsp; [develolin.github.io/GitFlowGraph](https://develolin.github.io/GitFlowGraph/) &nbsp;·&nbsp; [dev.elolin@gmail.com](mailto:dev.elolin@gmail.com)

</div>
