# GitFlowGraph

**Modern Git Graph & Smart Diff Workspace for Zed**

> Understand every change.

GitFlowGraph is a [Zed](https://zed.dev) extension that brings advanced Git visualization and intelligent diff analysis directly into your editor.

## Features

### Free
- Interactive Git commit graph (up to 500 commits)
- Branch & tag visualization
- Basic diff viewer
- Commit details panel

### Pro ($9.9/month or $99/year)
- **Smart Diff** — understands YAML, Terraform, JSON structure semantically
- **Semantic Diff** — AST-level code understanding via tree-sitter
- **Release Flow** — visualize how commits flow to production environments
- **Large Repo Optimization** — handles 100k+ commit repositories
- **Unlimited commits** — no 500-commit cap
- **Multi-repo support** & saved views
- 30-day free trial included, no credit card required

## Installation

1. Open Zed
2. Press `Cmd+Shift+X` to open Extensions
3. Search for **GitFlowGraph**
4. Click Install

The runtime binary (~15 MB) downloads automatically on first use.

## Usage

### Open the Git Graph
Type in the AI assistant panel:
```
/gitflowgraph
```
Then open `http://localhost:9876` in your browser.

### Smart Diff
```
/gitflowgraph-diff HEAD~3..HEAD
```

### License Activation
Navigate to `http://localhost:9876/license` and enter your Pro license key.

## Architecture

This extension uses a **marketplace loader + closed-source runtime** design:

| Component | Source | Role |
|-----------|--------|------|
| This extension | Open source (MIT) | Downloads & launches the runtime |
| `gitflowgraph-core` binary | Closed source | Git graph UI, Smart Diff engine, license validation |

The core algorithms are protected while the Zed marketplace integration remains fully open source.

## Privacy

- All Git operations run **locally** — no data leaves your machine
- License validation is **offline** via RSA cryptography — no server calls
- Zero telemetry

## Support

- Issues: https://github.com/gitflowgraph/gitflowgraph/issues

## License

MIT — see [LICENSE](LICENSE)
