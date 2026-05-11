use serde::Deserialize;
use std::fs;
use std::sync::atomic::{AtomicU16, Ordering};
use zed_extension_api::{
    self as zed,
    http_client::{fetch, HttpMethod, HttpRequest, RedirectPolicy},
    LanguageServerId, Result, SlashCommand, SlashCommandArgumentCompletion, SlashCommandOutput,
};

const RUNTIME_VERSION: &str = "0.1.0-beta.3";
/// Last-resort port — used only when the language-server launch hasn't
/// happened yet AND no per-worktree `port` file is on disk. The real
/// port for the live runtime is discovered via the `--port-file` the
/// extension passes to the binary; multi-window Zed must use that
/// instead of a fixed value or two windows would collide on the same
/// port and serve the wrong repo's data.
const FALLBACK_PORT: u16 = 9876;
const BINARY_STEM: &str = "gitflowgraph";
const GITHUB_REPO: &str = "DevEloLin/GitFlowGraph";

/// Most-recently-known port. Updated whenever `language_server_command`
/// fires (which is per-worktree) and read by `fetch_local_*`. The
/// completion entrypoint has no worktree context, so this static is
/// the only thing that lets it talk to the right runtime in
/// single-window use; multi-window users can still hand-type refs.
static CURRENT_PORT: AtomicU16 = AtomicU16::new(0);

/// Environment variables we forward from the user's shell to the
/// runtime child process. libgit2 reads `HOME` / `XDG_CONFIG_HOME`
/// (etc.) to locate `~/.gitconfig` — without these it loses
/// `user.name`, GPG signers, credential helpers, and aliases. PATH is
/// needed because libgit2 (and the runtime's CLI shells) invoke `git`,
/// `ssh`, `gpg`, `hostname`, and so on.
const PROPAGATED_ENV_KEYS: &[&str] = &[
    "HOME",
    "USER",
    "USERNAME",
    "PATH",
    "APPDATA",
    "LOCALAPPDATA",
    "XDG_CONFIG_HOME",
    "XDG_DATA_HOME",
    "SHELL",
    "LANG",
    "LC_ALL",
    "TZ",
    "SSH_AUTH_SOCK",
    "GIT_CONFIG",
    "GIT_CONFIG_GLOBAL",
    "GIT_CONFIG_SYSTEM",
];

struct GitFlowGraphExtension;

impl zed::Extension for GitFlowGraphExtension {
    fn new() -> Self {
        Self
    }

    fn language_server_command(
        &mut self,
        language_server_id: &LanguageServerId,
        worktree: &zed::Worktree,
    ) -> Result<zed::Command> {
        let root = worktree.root_path();

        if !is_git_repo(&root) {
            return Err(format!(
                "GitFlowGraph: no Git repository found at `{root}`. \
                 Open a folder that contains a `.git` directory."
            ));
        }

        let binary = self.ensure_runtime_binary(language_server_id)?;

        // Per-worktree state directory. Two Zed windows opening two
        // different repos must NOT share a port file or a config dir —
        // doing so was the cause of multi-repo data crossover (one
        // window's slash commands hitting the other window's runtime).
        let state_dir = worktree_state_dir(&root);
        let _ = fs::create_dir_all(&state_dir);
        let port_file = format!("{state_dir}/port");
        let config_dir = format!("{state_dir}/config");
        // Stale port-file from a previous crash would otherwise mislead
        // slash commands until the new runtime overwrites it.
        let _ = fs::remove_file(&port_file);

        // Prime CURRENT_PORT so single-window slash commands work
        // immediately even before the runtime writes its port-file.
        // Once the runtime starts and replaces the file, subsequent
        // reads pick up the actual ephemeral port.
        CURRENT_PORT.store(0, Ordering::Relaxed);

        Ok(zed::Command {
            command: binary,
            args: vec![
                // `--port 0` lets the OS pick a free port for *this*
                // worktree, so two simultaneous runtimes never fight
                // for `:9876`.
                "--port".to_string(),
                "0".to_string(),
                "--port-file".to_string(),
                port_file,
                "--config-dir".to_string(),
                config_dir,
                "--lsp".to_string(),
                "--repo".to_string(),
                root,
            ],
            // Forward only the env vars libgit2/git need (gitconfig,
            // ssh agent, PAGER, …). Empty env was the cause of "commits
            // are authored as <unknown>" in shipped builds.
            env: worktree_env(worktree),
        })
    }

    fn run_slash_command(
        &self,
        command: SlashCommand,
        args: Vec<String>,
        worktree: Option<&zed::Worktree>,
    ) -> Result<SlashCommandOutput, String> {
        // Resolve the runtime port for *this* worktree before any HTTP
        // calls — needed for multi-window users where each window's
        // runtime is on its own ephemeral port.
        if let Some(wt) = worktree {
            if let Some(p) = worktree_port(&wt.root_path()) {
                CURRENT_PORT.store(p, Ordering::Relaxed);
            }
        }
        // Args helpers — every command parses its inputs the same way:
        //   `arg(0)`     → optional first arg
        //   `range_arg(default)` → "from..to" with default fallback
        let arg = |i: usize| args.get(i).cloned();

        match command.name.as_str() {
            // ── Help / overview ──────────────────────────────────────
            "gitflowgraph" | "gitflowgraph-help" => render_overview(),

            // ── Read commands ────────────────────────────────────────
            "gitflowgraph-status" => render_status(),
            "gitflowgraph-graph" => {
                let n = arg(0)
                    .and_then(|s| s.parse::<u32>().ok())
                    .unwrap_or(20)
                    .clamp(1, 200);
                render_graph(n)
            }
            "gitflowgraph-branches" => render_branches(),
            "gitflowgraph-tags" => render_tags(),
            "gitflowgraph-worktrees" => render_worktrees(),
            "gitflowgraph-license" => render_license(),
            "gitflowgraph-remotes" => render_remotes(),
            "gitflowgraph-credentials" => render_credentials(),

            "gitflowgraph-diff" => {
                let range = arg(0).unwrap_or_else(|| "HEAD~1..HEAD".to_string());
                if !is_valid_ref_range(&range) {
                    return Err(invalid_range_error(&range));
                }
                render_diff(&range)
            }
            "gitflowgraph-changelog" => {
                let range = arg(0).unwrap_or_else(|| "HEAD~10..HEAD".to_string());
                if !is_valid_ref_range(&range) {
                    return Err(invalid_range_error(&range));
                }
                render_changelog(&range)
            }
            "gitflowgraph-risk" => {
                let range = arg(0).unwrap_or_else(|| "HEAD~10..HEAD".to_string());
                if !is_valid_ref_range(&range) {
                    return Err(invalid_range_error(&range));
                }
                render_risk(&range)
            }
            "gitflowgraph-launchpad" => {
                // Launchpad shows compare + risk + changelog + sync + checklist
                // for a release range. Default: "what would we ship if we cut
                // a tag now from HEAD against the most recent tag-shaped ref?"
                let range = arg(0).unwrap_or_else(|| "HEAD~10..HEAD".to_string());
                if !is_valid_ref_range(&range) {
                    return Err(invalid_range_error(&range));
                }
                render_launchpad(&range)
            }
            "gitflowgraph-velocity" => render_velocity(),
            "gitflowgraph-file-history" => {
                let path = arg(0).ok_or_else(|| {
                    "Usage: /gitflowgraph-file-history <path> — \
                     show every commit that touched <path>."
                        .to_string()
                })?;
                if !is_valid_file_path(&path) {
                    return Err(format!(
                        "GitFlowGraph: invalid path `{path}`. Path must be \
                         non-empty and contain no NUL bytes."
                    ));
                }
                render_file_history(&path)
            }

            // ── Write / mutating commands ────────────────────────────
            "gitflowgraph-checkout" => {
                let r = arg(0).ok_or_else(|| {
                    "Usage: /gitflowgraph-checkout <ref> — \
                     ref can be a branch, tag, or commit SHA."
                        .to_string()
                })?;
                if !is_valid_ref(&r) {
                    return Err(invalid_ref_error(&r));
                }
                run_action_post("/api/git/checkout", &serde_json::json!({"ref": r}),
                    &format!("Checked out `{r}`"), "Checkout")
            }
            "gitflowgraph-cherry-pick" => {
                let c = arg(0).ok_or_else(|| {
                    "Usage: /gitflowgraph-cherry-pick <commit-sha>".to_string()
                })?;
                if !is_valid_commit_id(&c) {
                    return Err(format!(
                        "GitFlowGraph: invalid commit SHA `{c}`. \
                         Pass a 7-40 character hex string."
                    ));
                }
                run_action_post("/api/git/cherry-pick", &serde_json::json!({"commit_id": c}),
                    &format!("Cherry-picked `{}`", short(&c)), "Cherry-pick")
            }
            "gitflowgraph-revert" => {
                let c = arg(0).ok_or_else(|| {
                    "Usage: /gitflowgraph-revert <commit-sha>".to_string()
                })?;
                if !is_valid_commit_id(&c) {
                    return Err(format!("GitFlowGraph: invalid commit SHA `{c}`."));
                }
                run_action_post("/api/git/revert", &serde_json::json!({"commit_id": c}),
                    &format!("Reverted `{}` (created revert commit on HEAD)", short(&c)), "Revert")
            }
            "gitflowgraph-merge" => {
                let r = arg(0).ok_or_else(|| {
                    "Usage: /gitflowgraph-merge <branch> — \
                     merges <branch> INTO the currently checked-out branch."
                        .to_string()
                })?;
                if !is_valid_ref(&r) {
                    return Err(invalid_ref_error(&r));
                }
                run_action_post("/api/git/merge", &serde_json::json!({"refname": r}),
                    &format!("Merged `{r}` into current branch"), "Merge")
            }
            "gitflowgraph-fetch" => {
                let remote = arg(0).unwrap_or_else(|| "origin".to_string());
                if !is_valid_ref(&remote) {
                    return Err(format!("GitFlowGraph: invalid remote name `{remote}`."));
                }
                run_action_post("/api/git/fetch", &serde_json::json!({"remote": remote}),
                    &format!("Fetched from `{remote}`"), "Fetch")
            }
            "gitflowgraph-push" => {
                let branch = arg(0).ok_or_else(|| {
                    "Usage: /gitflowgraph-push <branch>".to_string()
                })?;
                if !is_valid_ref(&branch) {
                    return Err(invalid_ref_error(&branch));
                }
                let remote = arg(1).unwrap_or_else(|| "origin".to_string());
                run_action_post(
                    "/api/git/push",
                    &serde_json::json!({"branch": branch, "remote": remote}),
                    &format!("Pushed `{branch}` to `{remote}`"),
                    "Push",
                )
            }
            "gitflowgraph-create-branch" => {
                let name = arg(0).ok_or_else(|| {
                    "Usage: /gitflowgraph-create-branch <name> [from-ref]".to_string()
                })?;
                let from = arg(1).unwrap_or_else(|| "HEAD".to_string());
                if !is_valid_ref(&name) || !is_valid_ref(&from) {
                    return Err(invalid_ref_error(&name));
                }
                run_action_post(
                    "/api/git/branch/create",
                    &serde_json::json!({"name": name, "commit_id": from}),
                    &format!("Branch `{name}` created at `{from}`"),
                    "Create branch",
                )
            }
            "gitflowgraph-delete-branch" => {
                let name = arg(0).ok_or_else(|| {
                    "Usage: /gitflowgraph-delete-branch <name> [--force]".to_string()
                })?;
                let force = args.iter().any(|a| a == "--force" || a == "-f");
                if !is_valid_ref(&name) {
                    return Err(invalid_ref_error(&name));
                }
                run_action_post(
                    "/api/git/branch/delete",
                    &serde_json::json!({"name": name, "force": force}),
                    &format!("Branch `{name}` deleted"),
                    "Delete branch",
                )
            }
            "gitflowgraph-create-tag" => {
                let name = arg(0).ok_or_else(|| {
                    "Usage: /gitflowgraph-create-tag <name> [from-ref] [--annotated <message>]"
                        .to_string()
                })?;
                let from = arg(1).filter(|s| !s.starts_with("--"))
                    .unwrap_or_else(|| "HEAD".to_string());
                // Annotated: caller passes `--annotated <message>` after positional args.
                let message = args.iter().position(|a| a == "--annotated")
                    .and_then(|i| args.get(i + 1).cloned());
                if !is_valid_ref(&name) || !is_valid_ref(&from) {
                    return Err(invalid_ref_error(&name));
                }
                run_action_post(
                    "/api/git/tag/create",
                    &serde_json::json!({
                        "name": name,
                        "commit_id": from,
                        "message": message,
                    }),
                    &format!("Tag `{name}` created at `{from}`"),
                    "Create tag",
                )
            }
            "gitflowgraph-delete-tag" => {
                let name = arg(0).ok_or_else(|| {
                    "Usage: /gitflowgraph-delete-tag <name>".to_string()
                })?;
                if !is_valid_ref(&name) {
                    return Err(invalid_ref_error(&name));
                }
                run_action_post(
                    "/api/git/tag/delete",
                    &serde_json::json!({"name": name}),
                    &format!("Tag `{name}` deleted"),
                    "Delete tag",
                )
            }
            "gitflowgraph-reset" => {
                let mode = arg(0).ok_or_else(|| {
                    "Usage: /gitflowgraph-reset <soft|mixed|hard> <commit-sha>".to_string()
                })?;
                let commit_id = arg(1).ok_or_else(|| {
                    "Usage: /gitflowgraph-reset <soft|mixed|hard> <commit-sha>".to_string()
                })?;
                if !matches!(mode.as_str(), "soft" | "mixed" | "hard") {
                    return Err("Mode must be one of: soft, mixed, hard.".to_string());
                }
                if !is_valid_commit_id(&commit_id) {
                    return Err(format!("GitFlowGraph: invalid commit SHA `{commit_id}`."));
                }
                run_action_post(
                    "/api/git/reset",
                    &serde_json::json!({"commit_id": commit_id, "mode": mode}),
                    &format!("{} reset to `{}`", capitalize(&mode), short(&commit_id)),
                    "Reset",
                )
            }
            "gitflowgraph-trial-start" => {
                run_action_post("/api/trial/start", &serde_json::json!({}),
                    "30-day Pro trial started — all Pro features unlocked.", "Start trial")
            }

            other => Err(format!(
                "GitFlowGraph: unknown command `{other}`.\n\
                 Run `/gitflowgraph` (or `/gitflowgraph-help`) for the full list."
            )),
        }
    }

    /// Auto-complete the *argument* of a slash command. Zed calls this as
    /// the user types — for ref-shaped commands we hit the local runtime
    /// for `/api/branches` + `/api/tags` and surface them as completion
    /// items. Without this the user has to know the exact branch / tag /
    /// SHA before invoking the command, which is the worst part of the
    /// slash-command UX. With this, typing
    ///   /gitflowgraph-checkout fea<Tab>
    /// drops down the matching branches, the user picks one, and the
    /// command runs.
    ///
    /// Failure mode: if the runtime isn't reachable (Zed extension is
    /// activated before the LSP server starts on a fresh project),
    /// returning an empty list is the right thing — Zed silently falls
    /// back to free-text and the user can still type the ref by hand.
    fn complete_slash_command_argument(
        &self,
        command: SlashCommand,
        args: Vec<String>,
    ) -> Result<Vec<SlashCommandArgumentCompletion>, String> {
        // Only completing the FIRST argument for now. Position-aware
        // multi-arg completion (e.g. `/gitflowgraph-create-branch <name>
        // <here-want-tag-completion>`) is more complex and Zed's
        // current API doesn't tell us which arg index is being typed.
        if args.len() > 1 {
            return Ok(Vec::new());
        }

        match command.name.as_str() {
            // Refs: branch + tag + a small set of useful pseudo-refs.
            "gitflowgraph-checkout"
            | "gitflowgraph-merge"
            | "gitflowgraph-push"
            | "gitflowgraph-fetch" => Ok(complete_ref_argument(true)),

            // Branch-only (renaming / deleting a tag would be wrong).
            "gitflowgraph-delete-branch" => Ok(complete_branches_only()),

            // Tag-only.
            "gitflowgraph-delete-tag" => Ok(complete_tags_only()),

            // Range-shaped commands: prefix the user's typed
            // "from.." with each tag/branch as the "to" suggestion.
            // E.g. typing `/gitflowgraph-changelog v1.0..` then Tab
            // suggests v1.1, v1.2, main, etc.
            "gitflowgraph-diff"
            | "gitflowgraph-changelog"
            | "gitflowgraph-risk"
            | "gitflowgraph-launchpad" => Ok(complete_range_argument(args.first())),

            // Reset: arg(0) is the mode (soft/mixed/hard), not a ref.
            "gitflowgraph-reset" => Ok(vec![
                completion("soft", "soft"),
                completion("mixed", "mixed"),
                completion("hard", "hard"),
            ]),

            // No completion for cherry-pick/revert/file-history/etc. —
            // SHAs and paths don't fit the "small enumerable list" model.
            _ => Ok(Vec::new()),
        }
    }
}

impl GitFlowGraphExtension {
    fn ensure_runtime_binary(&mut self, language_server_id: &LanguageServerId) -> Result<String> {
        let (os, arch) = zed::current_platform();
        let (asset_name, local_dir, local_binary, file_type) = platform_paths(&os, &arch)?;

        if binary_is_current(&local_binary, &local_dir) {
            return Ok(local_binary);
        }

        zed::set_language_server_installation_status(
            language_server_id,
            &zed::LanguageServerInstallationStatus::Downloading,
        );

        let release = zed::github_release_by_tag_name(
            GITHUB_REPO,
            &format!("v{RUNTIME_VERSION}"),
        )
        .map_err(|e| {
            format!(
                "GitFlowGraph: failed to fetch release v{RUNTIME_VERSION} from \
                 github.com/{GITHUB_REPO} — {e}."
            )
        })?;

        let asset = release
            .assets
            .iter()
            .find(|a| a.name == asset_name)
            .ok_or_else(|| {
                format!(
                    "GitFlowGraph: asset `{asset_name}` not found in release v{RUNTIME_VERSION}. \
                     Available: {}",
                    release.assets.iter().map(|a| a.name.as_str()).collect::<Vec<_>>().join(", ")
                )
            })?;

        zed::download_file(&asset.download_url, &local_dir, file_type)
            .map_err(|e| format!("GitFlowGraph: download of `{asset_name}` failed — {e}."))?;

        // TODO(integrity): publish a `<asset>.sha256` next to each release
        // asset and verify it here before chmod+x. Computing SHA-256 in
        // the WASM extension requires adding a crypto crate (or a hand-
        // rolled implementation) — deferred until we have the release
        // pipeline producing the sidecar files.
        zed::make_file_executable(&local_binary)
            .map_err(|e| format!("GitFlowGraph: failed to make binary executable — {e}"))?;

        write_installed_version(&local_dir);

        zed::set_language_server_installation_status(
            language_server_id,
            &zed::LanguageServerInstallationStatus::None,
        );

        Ok(local_binary)
    }
}

// ── helpers ──────────────────────────────────────────────────────────────────

fn platform_paths(
    os: &zed::Os,
    arch: &zed::Architecture,
) -> Result<(String, String, String, zed::DownloadedFileType)> {
    let (asset, dir, file_type) = match (os, arch) {
        (zed::Os::Mac, zed::Architecture::Aarch64) => (
            format!("{BINARY_STEM}-darwin-arm64.tar.gz"),
            format!("{BINARY_STEM}-darwin-arm64"),
            zed::DownloadedFileType::GzipTar,
        ),
        (zed::Os::Linux, zed::Architecture::X8664) => (
            format!("{BINARY_STEM}-linux-x86_64.tar.gz"),
            format!("{BINARY_STEM}-linux-x86_64"),
            zed::DownloadedFileType::GzipTar,
        ),
        (zed::Os::Linux, zed::Architecture::Aarch64) => (
            format!("{BINARY_STEM}-linux-arm64.tar.gz"),
            format!("{BINARY_STEM}-linux-arm64"),
            zed::DownloadedFileType::GzipTar,
        ),
        (zed::Os::Windows, zed::Architecture::X8664) => (
            format!("{BINARY_STEM}-windows-x86_64.zip"),
            format!("{BINARY_STEM}-windows-x86_64"),
            zed::DownloadedFileType::Zip,
        ),
        _ => {
            return Err(format!(
                "GitFlowGraph: unsupported platform {os:?}/{arch:?}. \
                 Supported: macOS (arm64/x86_64), Linux (arm64/x86_64), Windows (x86_64)."
            ))
        }
    };

    let binary = if matches!(os, zed::Os::Windows) {
        format!("{dir}/{BINARY_STEM}.exe")
    } else {
        format!("{dir}/{BINARY_STEM}")
    };

    Ok((asset, dir, binary, file_type))
}

fn binary_is_current(local_binary: &str, local_dir: &str) -> bool {
    if !fs::metadata(local_binary).is_ok_and(|m| m.is_file()) {
        return false;
    }
    let version_file = format!("{local_dir}/binary.version");
    fs::read_to_string(&version_file)
        .map(|v| v.trim() == RUNTIME_VERSION)
        .unwrap_or(false)
}

fn write_installed_version(local_dir: &str) {
    let version_file = format!("{local_dir}/binary.version");
    let _ = fs::write(&version_file, RUNTIME_VERSION);
}

fn is_git_repo(root: &str) -> bool {
    let git_path = format!("{root}/.git");
    fs::metadata(&git_path).is_ok()
}

fn is_valid_ref_range(range: &str) -> bool {
    if range.is_empty() {
        return false;
    }
    // Defence in depth: a malformed range like `../../etc/passwd` made
    // it through the per-char check because `.` and `/` are allowed. The
    // runtime would reject it on its own, but the extension should
    // never *send* such a string. Split on `..` (or `...`) and validate
    // each side as a standalone ref.
    let (left, right) = match split_range(range) {
        Some(pair) => pair,
        None => return false,
    };
    is_valid_ref(left) && is_valid_ref(right)
}

/// Split `a..b` / `a...b` into `(a, b)`. Returns `None` for inputs that
/// look like a single ref (those go through `is_valid_ref` directly).
fn split_range(range: &str) -> Option<(&str, &str)> {
    if let Some(idx) = range.find("...") {
        return Some((&range[..idx], &range[idx + 3..]));
    }
    range.find("..").map(|idx| (&range[..idx], &range[idx + 2..]))
}

/// Validate a single ref / branch / tag / SHA argument. Mirrors the
/// runtime's `valid_ref` (no leading `-`, no whitespace/control chars,
/// no `..`, no `\0`); the runtime will revparse it for real, so we
/// only need to filter out arg-injection-shaped inputs here.
fn is_valid_ref(s: &str) -> bool {
    if s.is_empty() || s.len() > 256 {
        return false;
    }
    // No leading `-` (would be misread as a CLI flag), no NUL, no
    // `..` (would be reinterpreted as a range), no whitespace or
    // control chars.
    if s.starts_with('-') || s.contains('\0') || s.contains("..") {
        return false;
    }
    !s.chars().any(|c| c.is_whitespace() || c.is_control())
}

/// Validate a 7-40 hex commit SHA. The runtime's mutating endpoints
/// (cherry-pick / revert / reset) only accept this format.
fn is_valid_commit_id(s: &str) -> bool {
    let len = s.len();
    (7..=40).contains(&len) && s.chars().all(|c| c.is_ascii_hexdigit())
}

/// Validate a file path passed to `/gitflowgraph-file-history`. The
/// runtime caps at 4096 bytes; we mirror that here so giant payloads
/// don't even reach the network.
fn is_valid_file_path(s: &str) -> bool {
    !s.is_empty() && s.len() <= 4096 && !s.contains('\0')
}

// ── Per-worktree state directory + port discovery ──────────────────────────
//
// One Zed window per worktree means one runtime process per worktree.
// Hard-coding `:9876` made every additional window collide with the
// first, silently serving the wrong repo's data. To fix this we let
// the runtime pick its port (`--port 0`) and write the actual port
// into a per-worktree file we hash from the worktree's absolute root.
// `language_server_command` configures the file path; slash commands
// read it back.

fn worktree_state_dir(root: &str) -> String {
    format!("state/{}", fnv1a_hex(root))
}

fn worktree_port(root: &str) -> Option<u16> {
    let p = format!("{}/port", worktree_state_dir(root));
    fs::read_to_string(&p)
        .ok()?
        .trim()
        .parse::<u16>()
        .ok()
}

/// Best-known port for the runtime serving the current call. Reads
/// from `CURRENT_PORT` (set by `run_slash_command` from the worktree)
/// and falls back to the legacy `9876` so a freshly-installed
/// extension whose runtime hasn't booted yet at least emits a usable
/// (if technically inaccurate) error message instead of `:0`.
fn current_port() -> u16 {
    let p = CURRENT_PORT.load(Ordering::Relaxed);
    if p == 0 {
        FALLBACK_PORT
    } else {
        p
    }
}

/// 64-bit FNV-1a over a UTF-8 string, rendered as 16 hex chars. Used
/// to derive a per-worktree filesystem slot without dragging in a
/// real crypto hash — collisions for the small set of paths a single
/// user opens are not a concern, and even if they happened the only
/// consequence is two worktrees sharing one state slot.
fn fnv1a_hex(s: &str) -> String {
    let mut h: u64 = 0xcbf2_9ce4_8422_2325;
    for b in s.bytes() {
        h ^= b as u64;
        h = h.wrapping_mul(0x0000_0100_0000_01b3);
    }
    format!("{h:016x}")
}

/// Forward a curated set of env vars from the user's shell to the
/// runtime child process. libgit2 needs HOME / XDG_CONFIG_HOME to find
/// `~/.gitconfig` (user.name, signingkey, credential helpers, aliases);
/// PATH is needed because the runtime shells out to `git`, `ssh`,
/// `gpg`, `hostname`. Empty env — the previous shipped value — caused
/// commits/tags created via the runtime to be authored as the libgit2
/// default (blank).
fn worktree_env(worktree: &zed::Worktree) -> Vec<(String, String)> {
    let shell = worktree.shell_env();
    shell
        .into_iter()
        .filter(|(k, _)| PROPAGATED_ENV_KEYS.contains(&k.as_str()))
        .collect()
}

fn invalid_range_error(range: &str) -> String {
    format!(
        "GitFlowGraph: invalid ref range `{range}`. \
         Examples: `HEAD~1..HEAD`, `main..feature/x`, `v1.0..HEAD`."
    )
}

fn invalid_ref_error(r: &str) -> String {
    format!(
        "GitFlowGraph: invalid ref `{r}`. \
         Refs must be non-empty, contain no whitespace, no `..`, and not start with `-`."
    )
}

fn short(sha: &str) -> &str {
    &sha[..7.min(sha.len())]
}

fn capitalize(s: &str) -> String {
    let mut chars = s.chars();
    match chars.next() {
        Some(c) => c.to_uppercase().chain(chars).collect(),
        None => String::new(),
    }
}

fn url_encode_range(range: &str) -> String {
    let mut out = String::with_capacity(range.len() + 8);
    for c in range.chars() {
        match c {
            ' ' => out.push_str("%20"),
            '^' => out.push_str("%5E"),
            '{' => out.push_str("%7B"),
            '}' => out.push_str("%7D"),
            _ => out.push(c),
        }
    }
    out
}

// ── In-Zed slash command renderers ──────────────────────────────────────────
//
// Zed's public extension API does not (yet) let extensions add status-bar
// icons, embedded webviews, or a docked panel. The closest we can get to an
// "in-Zed view" of GitFlowGraph is to fetch live data from the local runtime
// over HTTP and render it as Markdown into the Assistant panel via a slash
// command — Zed renders the returned `text` as Markdown, including code
// blocks (with syntax highlighting), tables, headers and links.
//
// Each slash command below calls `http://localhost:9876/api/...`, parses
// the JSON response, and formats a focused view (status, graph, diff,
// changelog, risk). The /gitflowgraph overview ties the most important
// snippets together so the user gets a one-screen summary without
// switching to the browser.

// ── Slash-command argument-completion helpers ──────────────────────────────
//
// Each helper queries the local runtime for the live ref / tag list and
// turns the response into `SlashCommandArgumentCompletion` items. Failures
// (runtime not started yet, etc.) return an empty list so Zed silently
// falls back to free-text — never blocks the user from typing a ref by hand.

fn completion(label: &str, new_text: &str) -> SlashCommandArgumentCompletion {
    SlashCommandArgumentCompletion {
        label: label.to_string(),
        new_text: new_text.to_string(),
        // We don't auto-run on completion — branch / tag / SHA picks are
        // usually one of multiple args (e.g. /push <branch> [remote]) so
        // the user should be able to keep typing.
        run_command: false,
    }
}

/// Branches + tags + a small set of always-useful pseudo-refs. The `head_first`
/// flag puts HEAD at the top because that's what users type most.
fn complete_ref_argument(head_first: bool) -> Vec<SlashCommandArgumentCompletion> {
    let mut out = Vec::new();
    if head_first {
        out.push(completion("HEAD", "HEAD"));
    }
    out.extend(branches_for_completion());
    out.extend(tags_for_completion());
    out
}

fn complete_branches_only() -> Vec<SlashCommandArgumentCompletion> {
    branches_for_completion()
}

fn complete_tags_only() -> Vec<SlashCommandArgumentCompletion> {
    tags_for_completion()
}

/// Range-arg completion: handles both the bare-ref case (suggest `<ref>`)
/// and the partial-range case (`from..` → suggest `from..<ref>` for each
/// candidate ref). Lets `/gitflowgraph-changelog v1.0..<Tab>` drop down
/// every newer tag without the user retyping the prefix.
fn complete_range_argument(typed: Option<&String>) -> Vec<SlashCommandArgumentCompletion> {
    let prefix = typed
        .and_then(|s| s.rfind("..").map(|idx| &s[..idx + 2]))
        .unwrap_or("");
    let mut out = Vec::new();
    if prefix.is_empty() {
        // No `..` typed yet — suggest each ref as a starting point.
        out.push(completion("HEAD", "HEAD"));
    }
    for c in branches_for_completion().into_iter().chain(tags_for_completion()) {
        let new_text = if prefix.is_empty() {
            c.new_text
        } else {
            format!("{prefix}{}", c.new_text)
        };
        out.push(SlashCommandArgumentCompletion {
            label: c.label,
            new_text,
            run_command: false,
        });
    }
    out
}

fn branches_for_completion() -> Vec<SlashCommandArgumentCompletion> {
    let body = match fetch_local_get("/api/branches") {
        Ok(b) => b,
        Err(_) => return Vec::new(),
    };
    let branches: Vec<BranchDto> = serde_json::from_str(&body).unwrap_or_default();
    branches
        .into_iter()
        .map(|b| {
            let label = if b.is_head {
                format!("⭐ {} (HEAD)", b.name)
            } else {
                b.name.clone()
            };
            completion(&label, &b.name)
        })
        .collect()
}

fn tags_for_completion() -> Vec<SlashCommandArgumentCompletion> {
    let body = match fetch_local_get("/api/tags") {
        Ok(b) => b,
        Err(_) => return Vec::new(),
    };
    let tags: Vec<TagDto> = serde_json::from_str(&body).unwrap_or_default();
    tags.into_iter()
        .map(|t| {
            let label = format!("🏷️ {}", t.name);
            completion(&label, &t.name)
        })
        .collect()
}

fn fetch_local_get(path: &str) -> std::result::Result<String, String> {
    // Build a GET against the runtime's loopback address. Localhost
    // requests inherit Zed's HTTP client, which permits private-IP
    // destinations — this is how other extensions reach LSP sidecars.
    let port = current_port();
    let req = HttpRequest {
        method: HttpMethod::Get,
        url: format!("http://127.0.0.1:{port}{path}"),
        headers: Vec::new(),
        body: None,
        redirect_policy: RedirectPolicy::FollowAll,
    };
    // The WIT-level HttpResponse doesn't expose status_code (only headers +
    // body). `fetch` returns Err on transport failures and on non-2xx, so
    // any Ok response can be treated as a successful body.
    let resp = fetch(&req).map_err(|e| {
        format!(
            "GitFlowGraph: could not reach the runtime on port {port} ({e}). \
             The runtime is started automatically when you open a Git \
             project — make sure the workspace contains a `.git` directory."
        )
    })?;
    Ok(String::from_utf8_lossy(&resp.body).into_owned())
}

/// POST a JSON body to a local runtime endpoint and return the raw
/// response text. Used by every mutating slash command (checkout,
/// cherry-pick, …). Like `fetch_local_get`, the wit-level HttpResponse
/// has no status_code so Err = transport failure or non-2xx, Ok = 2xx.
fn fetch_local_post(path: &str, body: &serde_json::Value) -> std::result::Result<String, String> {
    let body_bytes = serde_json::to_vec(body).map_err(|e| format!("serialize body: {e}"))?;
    let port = current_port();
    let req = HttpRequest {
        method: HttpMethod::Post,
        url: format!("http://127.0.0.1:{port}{path}"),
        headers: vec![("Content-Type".into(), "application/json".into())],
        body: Some(body_bytes),
        redirect_policy: RedirectPolicy::FollowAll,
    };
    // The runtime returns the LS-side error JSON for 4xx (license gating,
    // invalid input, etc.). The fetch host wraps non-2xx as Err with the
    // body as the error string, which is exactly what we want for action
    // commands — bubble the runtime's `error` field up to the user.
    let resp = fetch(&req).map_err(|e| {
        // `e` is already a string from the host; runtime errors come
        // through here verbatim ("invalid commit id", "402 Pro
        // required: …"). Trim a generic prefix if present so the user
        // sees the message rather than the wrapper.
        strip_status_prefix(&e).to_string()
    })?;
    Ok(String::from_utf8_lossy(&resp.body).into_owned())
}

/// Some Zed builds prefix transport errors with `HTTP <code>: ` — strip
/// that so the slash-command output reads naturally.
fn strip_status_prefix(s: &str) -> &str {
    if let Some(rest) = s.strip_prefix("HTTP ") {
        if let Some(idx) = rest.find(": ") {
            return &rest[idx + 2..];
        }
    }
    s
}

/// Run a POST action and render success / failure as Markdown for the
/// Assistant panel. Used by every write slash command. On success the
/// success message is shown verbatim and a brief footer reminds the
/// user to refresh the GitGraph (the in-Zed slash output and the
/// browser UI's TanStack Query caches are independent).
fn run_action_post(
    path: &str,
    body: &serde_json::Value,
    success_msg: &str,
    action_label: &str,
) -> Result<SlashCommandOutput, String> {
    match fetch_local_post(path, body) {
        Ok(_) => Ok(SlashCommandOutput {
            text: format!(
                "## ✓ {success_msg}\n\n\
                 _If you have the GitFlowGraph panel open in a browser, \
                 press **⌘R** to refresh the graph view._",
            ),
            sections: vec![],
        }),
        Err(e) => {
            // Try to parse JSON `{error: "..."}` from the body. The runtime
            // returns this shape for both 4xx and gating 402.
            let detail = serde_json::from_str::<serde_json::Value>(&e)
                .ok()
                .and_then(|v| v.get("error").and_then(|s| s.as_str().map(String::from)))
                .unwrap_or(e);
            Err(format!("GitFlowGraph: {action_label} failed — {detail}"))
        }
    }
}

// ── Response shapes — only the fields the slash commands render. The
// runtime's JSON includes more than this; serde silently drops the rest.

#[derive(Deserialize)]
struct LicenseStatus {
    is_pro: bool,
    trial_active: bool,
    days_remaining: Option<i32>,
}

#[derive(Deserialize)]
struct CommitDto {
    id: String,
    message: String,
    author: String,
    // `timestamp` and `email` exist in the runtime's CommitsResponse but
    // we don't render them here — keep the struct narrow so changes to
    // those fields don't break parsing.
    #[serde(default)]
    parent_ids: Vec<String>,
}

#[derive(Deserialize)]
struct CommitsResponse {
    commits: Vec<CommitDto>,
    #[serde(default)]
    truncated: bool,
}

#[derive(Deserialize)]
struct BranchDto {
    name: String,
    commit_id: String,
    #[serde(default)]
    is_head: bool,
}

#[derive(Deserialize)]
struct TagDto {
    name: String,
    commit_id: String,
}

#[derive(Deserialize)]
struct DiffFileDto {
    path: String,
    status: String,
    additions: u32,
    deletions: u32,
}

#[derive(Deserialize)]
struct ChangelogResponse {
    markdown: String,
}

#[derive(Deserialize)]
struct RiskFactorDto {
    file: String,
    reason: String,
    score: f64,
}

#[derive(Deserialize)]
struct RiskResponse {
    level: String,
    score: f64,
    changed_files: u32,
    #[serde(default)]
    factors: Vec<RiskFactorDto>,
}

#[derive(Deserialize)]
struct RemoteInfoDto {
    name: String,
    url: String,
    platform_display: String,
    has_credential: bool,
}

#[derive(Deserialize)]
struct CredentialEntryDto {
    host: String,
    platform: String,
    token_hint: String,
    username: Option<String>,
}

#[derive(Deserialize)]
struct WorktreeDto {
    path: String,
    head: Option<String>,
    branch: Option<String>,
    is_main: bool,
    is_locked: bool,
}

#[derive(Deserialize)]
struct VelocityResponse {
    release_count: u32,
    avg_days_between_releases: Option<f64>,
    hotfix_count: u32,
    hotfix_rate: f64,
    avg_commits_per_release: f64,
    #[serde(default)]
    recent_releases: Vec<ReleaseStatsDto>,
}

#[derive(Deserialize)]
struct ReleaseStatsDto {
    tag: String,
    date: i64,
    commit_count: u32,
    days_since_prev: Option<f64>,
}

#[derive(Deserialize)]
struct CompareResultDto {
    ahead_by: u32,
    behind_by: u32,
    #[serde(default)]
    commits: Vec<CommitDto>,
}

#[derive(Deserialize)]
struct ChecklistItemDto {
    title: String,
    detail: String,
    severity: String,
}

#[derive(Deserialize)]
struct ReleaseAnalysisResponse {
    compare: CompareResultDto,
    risk: RiskResponse,
    changelog: ChangelogResponse,
    #[serde(default)]
    checklist: Vec<ChecklistItemDto>,
}

#[derive(Deserialize)]
struct LicenseStatusFull {
    is_pro: bool,
    trial_active: bool,
    days_remaining: Option<i32>,
    license: Option<LicenseDetailsDto>,
    buy_url: String,
    portal_url: String,
    buy_url_is_configured: bool,
}

#[derive(Deserialize)]
struct LicenseDetailsDto {
    key: String,
    status: Option<String>,
    email: Option<String>,
    activation_limit: Option<i64>,
    activation_usage: Option<i64>,
    expires_at: Option<i64>,
    validated_at: i64,
}

// ── Renderers ──────────────────────────────────────────────────────────────

/// Default `/gitflowgraph` view. Pulls a one-shot summary intended to fit on
/// a single screen of the Assistant panel: license tier, working-tree
/// status counts (so the user knows whether they have anything uncommitted),
/// the most recent five commits as a table, plus links to drill down.
fn render_overview() -> Result<SlashCommandOutput, String> {
    let mut out = String::new();
    out.push_str("## GitFlowGraph\n\n");

    // License/trial badge — gives the user immediate signal about which
    // features will be available when they invoke other slash commands.
    match fetch_local_get("/api/license")
        .and_then(|s| serde_json::from_str::<LicenseStatus>(&s).map_err(|e| e.to_string()))
    {
        Ok(lic) => {
            let badge = if lic.is_pro {
                "**PRO**"
            } else if lic.trial_active {
                let d = lic.days_remaining.unwrap_or(0);
                return Ok(SlashCommandOutput {
                    text: render_overview_body(&format!(
                        "**TRIAL** · {d} day{} remaining",
                        if d == 1 { "" } else { "s" }
                    ))?,
                    sections: vec![],
                });
            } else {
                "FREE (500-commit cap)"
            };
            out.push_str(&render_overview_body(badge)?);
        }
        Err(e) => {
            // Even when the license endpoint fails (e.g. runtime just
            // started), the rest of the overview is still useful — render
            // it without the badge rather than erroring out entirely.
            out.push_str(&format!("> ⚠ License status unavailable: {e}\n\n"));
            out.push_str(&render_overview_body("…")?);
        }
    }

    Ok(SlashCommandOutput {
        text: out,
        sections: vec![],
    })
}

fn render_overview_body(license_badge: &str) -> std::result::Result<String, String> {
    let mut out = String::new();
    out.push_str(&format!("**Tier**: {license_badge}\n\n"));

    // Working-tree summary — counts only, full status is /gitflowgraph-status.
    if let Ok(body) = fetch_local_get("/api/diff/files?base=WORKING&head=HEAD") {
        if let Ok(files) = serde_json::from_str::<Vec<DiffFileDto>>(&body) {
            if !files.is_empty() {
                let added = files.iter().filter(|f| f.status == "added").count();
                let modified = files.iter().filter(|f| f.status == "modified").count();
                let deleted = files.iter().filter(|f| f.status == "deleted").count();
                out.push_str(&format!(
                    "**Working tree**: {added} added · {modified} modified · {deleted} deleted (`/gitflowgraph-status` for details)\n\n",
                ));
            } else {
                out.push_str("**Working tree**: clean ✓\n\n");
            }
        }
    }

    // Recent commits table — small enough to read in the Assistant pane,
    // bigger views via `/gitflowgraph-graph N`.
    out.push_str("**Recent commits**\n\n");
    out.push_str(&render_commits_table(5)?);

    out.push_str(&format!(
        "\n---\n\n\
         ### Read views\n\
         `/gitflowgraph-status` · `/gitflowgraph-graph [n]` · \
         `/gitflowgraph-branches` · `/gitflowgraph-tags` · \
         `/gitflowgraph-worktrees` · `/gitflowgraph-license`\n\n\
         ### Range / release\n\
         `/gitflowgraph-diff <range>` · `/gitflowgraph-changelog <range>` · \
         `/gitflowgraph-risk <range>` · `/gitflowgraph-launchpad <range>` · \
         `/gitflowgraph-velocity` · `/gitflowgraph-file-history <path>`\n\n\
         ### Write actions\n\
         `/gitflowgraph-checkout <ref>` · `/gitflowgraph-cherry-pick <sha>` · \
         `/gitflowgraph-revert <sha>` · `/gitflowgraph-merge <branch>` · \
         `/gitflowgraph-reset <soft|mixed|hard> <sha>` · \
         `/gitflowgraph-fetch [remote]` · `/gitflowgraph-push <branch> [remote]`\n\n\
         `/gitflowgraph-create-branch <name> [from]` · \
         `/gitflowgraph-delete-branch <name> [--force]` · \
         `/gitflowgraph-create-tag <name> [from] [--annotated <message>]` · \
         `/gitflowgraph-delete-tag <name>`\n\n\
         `/gitflowgraph-trial-start` (free 30-day Pro trial)\n\n\
         ---\n\n\
         For the full interactive UI (lane graph, structural diff, hotfix \
         wizard) open [http://localhost:{port}](http://localhost:{port}) in \
         a browser. Zed's extension API doesn't yet expose a webview — \
         every feature here is reachable as a slash command.\n",
        port = current_port()
    ));
    Ok(out)
}

fn render_status() -> Result<SlashCommandOutput, String> {
    let working = fetch_local_get("/api/diff/files?base=WORKING&head=HEAD")?;
    let staged = fetch_local_get("/api/diff/files?base=STAGED&head=HEAD")?;
    let working: Vec<DiffFileDto> =
        serde_json::from_str(&working).map_err(|e| format!("parse working: {e}"))?;
    let staged: Vec<DiffFileDto> =
        serde_json::from_str(&staged).map_err(|e| format!("parse staged: {e}"))?;

    let mut out = String::from("## Working tree status\n\n");
    if working.is_empty() && staged.is_empty() {
        out.push_str("Working tree is clean — no uncommitted changes.\n");
        return Ok(SlashCommandOutput {
            text: out,
            sections: vec![],
        });
    }

    if !staged.is_empty() {
        out.push_str("### Staged\n\n");
        out.push_str("| Status | File | + / − |\n|---|---|---|\n");
        for f in &staged {
            out.push_str(&format!(
                "| {} | `{}` | +{} −{} |\n",
                status_badge(&f.status),
                escape_md(&f.path),
                f.additions,
                f.deletions
            ));
        }
        out.push('\n');
    }
    if !working.is_empty() {
        out.push_str("### Working tree (unstaged)\n\n");
        out.push_str("| Status | File | + / − |\n|---|---|---|\n");
        for f in &working {
            out.push_str(&format!(
                "| {} | `{}` | +{} −{} |\n",
                status_badge(&f.status),
                escape_md(&f.path),
                f.additions,
                f.deletions
            ));
        }
        out.push('\n');
    }

    Ok(SlashCommandOutput {
        text: out,
        sections: vec![],
    })
}

fn render_graph(n: u32) -> Result<SlashCommandOutput, String> {
    let mut out = String::from("## Recent commits\n\n");
    out.push_str(&render_commits_table(n as usize)?);
    Ok(SlashCommandOutput {
        text: out,
        sections: vec![],
    })
}

fn render_commits_table(limit: usize) -> std::result::Result<String, String> {
    let qs = format!("/api/commits?limit={limit}");
    let body = fetch_local_get(&qs)?;
    let resp: CommitsResponse =
        serde_json::from_str(&body).map_err(|e| format!("parse commits: {e}"))?;

    // Branch + tag annotations so the table actually reflects the graph
    // semantics, not just a flat log. /branches and /tags are cheap and
    // already-cached by the runtime.
    let branches: Vec<BranchDto> = serde_json::from_str(
        &fetch_local_get("/api/branches").unwrap_or_else(|_| "[]".into()),
    )
    .unwrap_or_default();
    let tags: Vec<TagDto> = serde_json::from_str(
        &fetch_local_get("/api/tags").unwrap_or_else(|_| "[]".into()),
    )
    .unwrap_or_default();

    let mut out = String::new();
    out.push_str("| | Hash | Message | Author |\n|---|---|---|---|\n");
    for c in &resp.commits {
        let mut tags_for: Vec<String> = tags
            .iter()
            .filter(|t| t.commit_id == c.id)
            .map(|t| format!("🏷️ `{}`", t.name))
            .collect();
        let branches_for: Vec<String> = branches
            .iter()
            .filter(|b| b.commit_id == c.id)
            .map(|b| {
                if b.is_head {
                    format!("**HEAD →** `{}`", b.name)
                } else {
                    format!("`{}`", b.name)
                }
            })
            .collect();
        tags_for.extend(branches_for);
        let prefix = if tags_for.is_empty() {
            "•".to_string()
        } else {
            tags_for.join(" ")
        };
        let short = &c.id[..7.min(c.id.len())];
        let first_line = c.message.lines().next().unwrap_or("").to_string();
        let merge = if c.parent_ids.len() > 1 { " (merge)" } else { "" };
        out.push_str(&format!(
            "| {} | `{}` | {}{} | {} |\n",
            prefix,
            short,
            escape_md(&first_line),
            merge,
            escape_md(&c.author),
        ));
    }
    if resp.truncated {
        out.push_str(&format!(
            "\n> ⚠ Showing first {limit} commits. Activate Pro for unlimited history.\n"
        ));
    }
    Ok(out)
}

fn render_diff(range: &str) -> Result<SlashCommandOutput, String> {
    // Parse `from..to` into base / head. Single ref means "from = ref^,
    // head = ref" so the user can write `/gitflowgraph-diff abc1234`
    // and see what that commit changed.
    let (base, head) = if let Some(idx) = range.find("..") {
        (&range[..idx], &range[idx + 2..])
    } else {
        return Ok(SlashCommandOutput {
            text: format!(
                "## Smart Diff — `{range}`\n\n\
                 _Single-ref form (showing what `{range}` changed against its parent)_\n\n\
                 {}",
                render_diff_body(&format!("{range}^"), range)?
            ),
            sections: vec![],
        });
    };
    let body = render_diff_body(base, head)?;
    Ok(SlashCommandOutput {
        text: format!("## Smart Diff — `{base}…{head}`\n\n{body}"),
        sections: vec![],
    })
}

fn render_diff_body(base: &str, head: &str) -> std::result::Result<String, String> {
    let qs = format!(
        "/api/diff/files?base={}&head={}",
        url_encode_range(base),
        url_encode_range(head)
    );
    let body = fetch_local_get(&qs)?;
    let files: Vec<DiffFileDto> =
        serde_json::from_str(&body).map_err(|e| format!("parse diff/files: {e}"))?;
    if files.is_empty() {
        return Ok("No file changes between these refs.\n".into());
    }
    let mut out = String::new();
    let total_add: u32 = files.iter().map(|f| f.additions).sum();
    let total_del: u32 = files.iter().map(|f| f.deletions).sum();
    out.push_str(&format!(
        "**{n} file{s} changed** · +{total_add} −{total_del}\n\n",
        n = files.len(),
        s = if files.len() == 1 { "" } else { "s" }
    ));
    out.push_str("| Status | File | + / − |\n|---|---|---|\n");
    for f in &files {
        out.push_str(&format!(
            "| {} | `{}` | +{} −{} |\n",
            status_badge(&f.status),
            escape_md(&f.path),
            f.additions,
            f.deletions
        ));
    }
    out.push_str(&format!(
        "\n_Open the structural diff (YAML/JSON/Terraform-aware) at \
         http://localhost:{port}_\n",
        port = current_port()
    ));
    Ok(out)
}

fn render_changelog(range: &str) -> Result<SlashCommandOutput, String> {
    let (from, to) = parse_range(range);
    let qs = format!(
        "/api/release/changelog?from={}&to={}",
        url_encode_range(&from),
        url_encode_range(&to)
    );
    let body = fetch_local_get(&qs)?;
    let resp: ChangelogResponse =
        serde_json::from_str(&body).map_err(|e| format!("parse changelog: {e}"))?;
    Ok(SlashCommandOutput {
        text: format!(
            "## Changelog — `{from}…{to}`\n\n{}",
            resp.markdown
        ),
        sections: vec![],
    })
}

fn render_risk(range: &str) -> Result<SlashCommandOutput, String> {
    let (from, to) = parse_range(range);
    let qs = format!(
        "/api/release/risk?from={}&to={}",
        url_encode_range(&from),
        url_encode_range(&to)
    );
    let body = fetch_local_get(&qs)?;
    let resp: RiskResponse =
        serde_json::from_str(&body).map_err(|e| format!("parse risk: {e}"))?;
    let badge = match resp.level.as_str() {
        "HIGH" => "🔴 **HIGH**",
        "MEDIUM" => "🟡 **MEDIUM**",
        _ => "🟢 **LOW**",
    };
    let mut out = format!(
        "## Release Risk — `{from}…{to}`\n\n\
         **Level**: {badge}  \n\
         **Score**: {score:.1}  \n\
         **Files changed**: {fc}\n\n",
        score = resp.score,
        fc = resp.changed_files,
    );
    if resp.factors.is_empty() {
        out.push_str("_No risk factors flagged._\n");
    } else {
        out.push_str("### Top risk factors\n\n");
        out.push_str("| File | Reason | Score |\n|---|---|---|\n");
        for f in resp.factors.iter().take(15) {
            out.push_str(&format!(
                "| `{}` | {} | {:.1} |\n",
                escape_md(&f.file),
                escape_md(&f.reason),
                f.score,
            ));
        }
    }
    Ok(SlashCommandOutput {
        text: out,
        sections: vec![],
    })
}

fn parse_range(range: &str) -> (String, String) {
    if let Some(idx) = range.find("..") {
        (range[..idx].to_string(), range[idx + 2..].to_string())
    } else {
        // Bare ref → "from this ref's parent to this ref" — same convention
        // used by render_diff.
        (format!("{range}^"), range.to_string())
    }
}

fn status_badge(s: &str) -> &'static str {
    match s {
        "added" => "🟢 +",
        "deleted" => "🔴 −",
        "renamed" => "🔵 →",
        _ => "🟡 ~",
    }
}

/// Markdown-escape characters that would otherwise turn into formatting
/// inside a table cell. The set is conservative — only what actually
/// breaks GFM tables.
fn escape_md(s: &str) -> String {
    let mut out = String::with_capacity(s.len());
    for c in s.chars() {
        match c {
            '|' => out.push_str("\\|"),
            '\n' | '\r' => out.push(' '),
            '`' => out.push_str("\\`"),
            _ => out.push(c),
        }
    }
    out
}

// ── Read renderers (continued) ─────────────────────────────────────────────

fn render_branches() -> Result<SlashCommandOutput, String> {
    let body = fetch_local_get("/api/branches")?;
    let branches: Vec<BranchDto> =
        serde_json::from_str(&body).map_err(|e| format!("parse branches: {e}"))?;
    if branches.is_empty() {
        return Ok(SlashCommandOutput {
            text: "## Branches\n\n_No branches in this repository._\n".into(),
            sections: vec![],
        });
    }
    let mut out = format!(
        "## Branches ({n})\n\n| | Branch | At commit |\n|---|---|---|\n",
        n = branches.len()
    );
    for b in &branches {
        let mark = if b.is_head { "**HEAD →**" } else { "•" };
        out.push_str(&format!(
            "| {} | `{}` | `{}` |\n",
            mark,
            escape_md(&b.name),
            short(&b.commit_id),
        ));
    }
    out.push_str("\n_Switch with `/gitflowgraph-checkout <branch>` · \
                 push with `/gitflowgraph-push <branch>`._\n");
    Ok(SlashCommandOutput { text: out, sections: vec![] })
}

fn render_tags() -> Result<SlashCommandOutput, String> {
    let body = fetch_local_get("/api/tags")?;
    let tags: Vec<TagDto> =
        serde_json::from_str(&body).map_err(|e| format!("parse tags: {e}"))?;
    if tags.is_empty() {
        return Ok(SlashCommandOutput {
            text: "## Tags\n\n_No tags in this repository._\n".into(),
            sections: vec![],
        });
    }
    let mut out = format!(
        "## Tags ({n})\n\n| Tag | At commit |\n|---|---|\n",
        n = tags.len()
    );
    for t in &tags {
        out.push_str(&format!(
            "| 🏷️ `{}` | `{}` |\n",
            escape_md(&t.name),
            short(&t.commit_id),
        ));
    }
    out.push_str("\n_Create with `/gitflowgraph-create-tag <name> [from]` · \
                 delete with `/gitflowgraph-delete-tag <name>`._\n");
    Ok(SlashCommandOutput { text: out, sections: vec![] })
}

fn render_remotes() -> Result<SlashCommandOutput, String> {
    let body = fetch_local_get("/api/remotes")?;
    let remotes: Vec<RemoteInfoDto> =
        serde_json::from_str(&body).map_err(|e| format!("parse remotes: {e}"))?;
    if remotes.is_empty() {
        return Ok(SlashCommandOutput {
            text: "## Remotes\n\n_No remotes configured. Add one with `git remote add` first._\n"
                .into(),
            sections: vec![],
        });
    }
    let mut out = String::from("## Remotes\n\n| Name | URL | Platform | Credential |\n|---|---|---|---|\n");
    for r in &remotes {
        out.push_str(&format!(
            "| `{}` | `{}` | {} | {} |\n",
            escape_md(&r.name),
            escape_md(&r.url),
            escape_md(&r.platform_display),
            if r.has_credential { "✓ stored" } else { "⚠ none" },
        ));
    }
    out.push_str("\n_Fetch with `/gitflowgraph-fetch <name>` (default: origin)._\n");
    Ok(SlashCommandOutput { text: out, sections: vec![] })
}

fn render_credentials() -> Result<SlashCommandOutput, String> {
    let body = fetch_local_get("/api/settings/credentials")?;
    let creds: Vec<CredentialEntryDto> =
        serde_json::from_str(&body).map_err(|e| format!("parse credentials: {e}"))?;
    if creds.is_empty() {
        let port = current_port();
        return Ok(SlashCommandOutput {
            text: format!(
                "## Stored credentials\n\n_No credentials stored._\n\n\
                 To add one, open the **Remotes** tab in the browser UI \
                 (http://localhost:{port}) — slash commands don't accept \
                 plaintext PATs to keep them out of Assistant chat history.\n"
            ),
            sections: vec![],
        });
    }
    let mut out = String::from(
        "## Stored credentials\n\n| Host | Platform | Username | Token (masked) |\n|---|---|---|---|\n",
    );
    for c in &creds {
        out.push_str(&format!(
            "| `{}` | {} | {} | `{}` |\n",
            escape_md(&c.host),
            escape_md(&c.platform),
            c.username.as_deref().map(escape_md).unwrap_or_else(|| "—".into()),
            escape_md(&c.token_hint),
        ));
    }
    out.push_str(
        "\n_Add / edit / delete in the browser **Remotes** tab — \
        plaintext PATs would otherwise leak into Assistant chat history._\n",
    );
    Ok(SlashCommandOutput { text: out, sections: vec![] })
}

fn render_worktrees() -> Result<SlashCommandOutput, String> {
    let body = fetch_local_get("/api/worktrees")?;
    let worktrees: Vec<WorktreeDto> =
        serde_json::from_str(&body).map_err(|e| format!("parse worktrees: {e}"))?;
    if worktrees.is_empty() {
        return Ok(SlashCommandOutput {
            text: "## Worktrees\n\n_No worktrees registered._\n".into(),
            sections: vec![],
        });
    }
    let mut out = String::from("## Worktrees\n\n| | Path | Branch | HEAD |\n|---|---|---|---|\n");
    for w in &worktrees {
        let mark = if w.is_main {
            "🏠 main"
        } else if w.is_locked {
            "🔒 locked"
        } else {
            "🌿"
        };
        out.push_str(&format!(
            "| {} | `{}` | {} | `{}` |\n",
            mark,
            escape_md(&w.path),
            w.branch.as_deref().map(escape_md).unwrap_or_else(|| "(detached)".into()),
            w.head.as_deref().map(short).unwrap_or("?"),
        ));
    }
    Ok(SlashCommandOutput { text: out, sections: vec![] })
}

fn render_license() -> Result<SlashCommandOutput, String> {
    let body = fetch_local_get("/api/license")?;
    let lic: LicenseStatusFull =
        serde_json::from_str(&body).map_err(|e| format!("parse license: {e}"))?;

    let mut out = String::from("## License\n\n");
    if lic.is_pro {
        out.push_str("**PRO** · all features unlocked.\n\n");
        if let Some(d) = lic.license.as_ref() {
            if let Some(email) = &d.email {
                out.push_str(&format!("**Email**: {email}  \n"));
            }
            if let Some(status) = &d.status {
                out.push_str(&format!("**Status**: {status}  \n"));
            }
            let key_masked = if d.key.len() > 8 {
                format!("{}…{}", &d.key[..4], &d.key[d.key.len() - 4..])
            } else {
                "****".into()
            };
            out.push_str(&format!("**Key**: `{key_masked}`  \n"));
            if let (Some(usage), Some(limit)) = (d.activation_usage, d.activation_limit) {
                out.push_str(&format!("**Machines**: {usage} of {limit}  \n"));
            } else if let Some(usage) = d.activation_usage {
                out.push_str(&format!("**Machines**: {usage} (unlimited)  \n"));
            }
            if let Some(exp) = d.expires_at {
                out.push_str(&format!("**Expires**: epoch {exp}  \n"));
            }
            out.push_str(&format!("**Last validated**: epoch {}  \n", d.validated_at));
        }
        out.push_str(&format!(
            "\n[Customer portal →]({})\n",
            lic.portal_url
        ));
    } else if lic.trial_active {
        let d = lic.days_remaining.unwrap_or(0);
        let port = current_port();
        out.push_str(&format!(
            "**TRIAL** · {d} day{plural} remaining\n\n\
             Activate a paid key with `/gitflowgraph-activate-key <key>` (TODO) \
             or open the License tab in the browser at \
             `http://localhost:{port}` and click **🔑 Activate License**.\n\n\
             [Buy License →]({buy})\n",
            plural = if d == 1 { "" } else { "s" },
            buy = lic.buy_url
        ));
    } else {
        out.push_str(
            "**FREE** · 500-commit limit, no write operations.\n\n\
             Start a free 30-day trial with `/gitflowgraph-trial-start` \
             — no payment, no auto-renewal.\n\n",
        );
        out.push_str(&format!(
            "[{} →]({})\n",
            if lic.buy_url_is_configured { "Buy License" } else { "Visit Pricing" },
            lic.buy_url,
        ));
    }
    Ok(SlashCommandOutput { text: out, sections: vec![] })
}

fn render_velocity() -> Result<SlashCommandOutput, String> {
    let body = fetch_local_get("/api/release/velocity")?;
    let v: VelocityResponse =
        serde_json::from_str(&body).map_err(|e| format!("parse velocity: {e}"))?;
    let mut out = String::from("## Release Velocity\n\n");
    out.push_str(&format!(
        "- **Releases**: {}\n\
         - **Hotfixes**: {} ({:.0}% of releases)\n\
         - **Avg days between releases**: {}\n\
         - **Avg commits per release**: {:.1}\n\n",
        v.release_count,
        v.hotfix_count,
        v.hotfix_rate * 100.0,
        v.avg_days_between_releases
            .map(|d| format!("{d:.1}"))
            .unwrap_or_else(|| "—".into()),
        v.avg_commits_per_release,
    ));
    if !v.recent_releases.is_empty() {
        out.push_str("### Recent releases\n\n| Tag | Date | Commits | Days since previous |\n|---|---|---|---|\n");
        for r in &v.recent_releases {
            let date = format_epoch_date(r.date);
            let gap = r
                .days_since_prev
                .map(|d| format!("{d:.0}"))
                .unwrap_or_else(|| "—".into());
            out.push_str(&format!(
                "| 🏷️ `{}` | {} | {} | {} |\n",
                escape_md(&r.tag),
                date,
                r.commit_count,
                gap,
            ));
        }
    }
    Ok(SlashCommandOutput { text: out, sections: vec![] })
}

fn render_launchpad(range: &str) -> Result<SlashCommandOutput, String> {
    let (from, to) = parse_range(range);
    // /api/release/analyze returns the bundle of compare + risk +
    // changelog + sync + checklist that the browser Launchpad shows
    // as five separate cards. We render them stacked.
    let qs = format!(
        "/api/release/analyze?from={}&to={}",
        url_encode_range(&from),
        url_encode_range(&to)
    );
    let body = fetch_local_get(&qs)?;
    let resp: ReleaseAnalysisResponse =
        serde_json::from_str(&body).map_err(|e| format!("parse analyze: {e}"))?;

    let mut out = format!("## Release Launchpad — `{from}…{to}`\n\n");

    // Compare summary
    out.push_str(&format!(
        "**↑ {} ahead**, **↓ {} behind**, {} commit{} in range\n\n",
        resp.compare.ahead_by,
        resp.compare.behind_by,
        resp.compare.commits.len(),
        if resp.compare.commits.len() == 1 { "" } else { "s" },
    ));

    // Risk
    let risk_badge = match resp.risk.level.as_str() {
        "HIGH" => "🔴 **HIGH**",
        "MEDIUM" => "🟡 **MEDIUM**",
        _ => "🟢 **LOW**",
    };
    out.push_str(&format!(
        "### Risk\n\n{badge} · score {score:.1} · {fc} files changed\n\n",
        badge = risk_badge,
        score = resp.risk.score,
        fc = resp.risk.changed_files,
    ));

    // Checklist
    if !resp.checklist.is_empty() {
        out.push_str("### Pre-flight checklist\n\n");
        for item in &resp.checklist {
            let mark = match item.severity.as_str() {
                "blocker" => "🔴",
                "warning" => "🟡",
                _ => "ℹ️",
            };
            out.push_str(&format!(
                "- {} **{}** — {}\n",
                mark,
                escape_md(&item.title),
                escape_md(&item.detail),
            ));
        }
        out.push('\n');
    }

    // Changelog (the runtime already renders it as Markdown)
    out.push_str("### Changelog\n\n");
    out.push_str(&resp.changelog.markdown);

    Ok(SlashCommandOutput { text: out, sections: vec![] })
}

fn render_file_history(path: &str) -> Result<SlashCommandOutput, String> {
    let qs = format!("/api/file/history?path={}&limit=50", url_encode_range(path));
    let body = fetch_local_get(&qs)?;
    let resp: CommitsResponse =
        serde_json::from_str(&body).map_err(|e| format!("parse file/history: {e}"))?;
    let mut out = format!("## File history — `{}`\n\n", escape_md(path));
    if resp.commits.is_empty() {
        out.push_str("_No commits touched this file._\n");
        return Ok(SlashCommandOutput { text: out, sections: vec![] });
    }
    out.push_str("| Hash | Message | Author |\n|---|---|---|\n");
    for c in &resp.commits {
        out.push_str(&format!(
            "| `{}` | {} | {} |\n",
            short(&c.id),
            escape_md(c.message.lines().next().unwrap_or("")),
            escape_md(&c.author),
        ));
    }
    Ok(SlashCommandOutput { text: out, sections: vec![] })
}

fn format_epoch_date(epoch: i64) -> String {
    // We don't have chrono in the extension dep tree; do a minimal
    // YYYY-MM-DD conversion via days-since-epoch. Good enough for
    // "Recent releases" — release-day precision is what the user wants.
    let days = epoch / 86_400;
    let (y, m, d) = days_to_ymd(days);
    format!("{y:04}-{m:02}-{d:02}")
}

/// Convert "days since 1970-01-01" into (year, month, day). Adapted from
/// the civil-from-days algorithm by Howard Hinnant. No leap-year /
/// off-by-one issues; works for any 64-bit epoch we'll ever see.
fn days_to_ymd(days: i64) -> (i32, u32, u32) {
    let z = days + 719_468;
    let era = if z >= 0 { z } else { z - 146_096 } / 146_097;
    let doe = (z - era * 146_097) as u64;
    let yoe = (doe - doe / 1460 + doe / 36524 - doe / 146_096) / 365;
    let y = yoe as i64 + era * 400;
    let doy = doe - (365 * yoe + yoe / 4 - yoe / 100);
    let mp = (5 * doy + 2) / 153;
    let d = (doy - (153 * mp + 2) / 5 + 1) as u32;
    let m = (if mp < 10 { mp + 3 } else { mp - 9 }) as u32;
    let y = (y + if m <= 2 { 1 } else { 0 }) as i32;
    (y, m, d)
}

zed::register_extension!(GitFlowGraphExtension);
