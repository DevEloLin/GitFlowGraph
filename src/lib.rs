use serde::Deserialize;
use std::fs;
use zed_extension_api::{
    self as zed,
    http_client::{fetch, HttpMethod, HttpRequest, RedirectPolicy},
    LanguageServerId, Result, SlashCommand, SlashCommandOutput,
};

const RUNTIME_VERSION: &str = "0.1.0-beta.1";
const RUNTIME_PORT: u16 = 9876;
const BINARY_STEM: &str = "gitflowgraph";
const GITHUB_REPO: &str = "DevEloLin/GitFlowGraph";

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

        Ok(zed::Command {
            command: binary,
            args: vec![
                "--port".to_string(),
                RUNTIME_PORT.to_string(),
                "--lsp".to_string(),
                "--repo".to_string(),
                root,
            ],
            env: vec![],
        })
    }

    fn run_slash_command(
        &self,
        command: SlashCommand,
        args: Vec<String>,
        _worktree: Option<&zed::Worktree>,
    ) -> Result<SlashCommandOutput, String> {
        match command.name.as_str() {
            "gitflowgraph" => render_overview(),
            "gitflowgraph-status" => render_status(),
            "gitflowgraph-graph" => {
                let n = args
                    .first()
                    .and_then(|s| s.parse::<u32>().ok())
                    .unwrap_or(20)
                    .clamp(1, 200);
                render_graph(n)
            }
            "gitflowgraph-diff" => {
                let range = args
                    .first()
                    .cloned()
                    .unwrap_or_else(|| "HEAD~1..HEAD".to_string());
                if !is_valid_ref_range(&range) {
                    return Err(format!(
                        "GitFlowGraph: invalid ref range `{range}`. \
                         Examples: `HEAD~1..HEAD`, `main..feature/my-branch`, `abc1234`."
                    ));
                }
                render_diff(&range)
            }
            "gitflowgraph-changelog" => {
                let range = args
                    .first()
                    .cloned()
                    .unwrap_or_else(|| "HEAD~10..HEAD".to_string());
                if !is_valid_ref_range(&range) {
                    return Err(format!(
                        "GitFlowGraph: invalid ref range `{range}`. \
                         Examples: `v1.0..HEAD`, `HEAD~10..HEAD`."
                    ));
                }
                render_changelog(&range)
            }
            "gitflowgraph-risk" => {
                let range = args
                    .first()
                    .cloned()
                    .unwrap_or_else(|| "HEAD~10..HEAD".to_string());
                if !is_valid_ref_range(&range) {
                    return Err(format!(
                        "GitFlowGraph: invalid ref range `{range}`."
                    ));
                }
                render_risk(&range)
            }
            other => Err(format!(
                "GitFlowGraph: unknown command `{other}`. \
                 Available: `/gitflowgraph`, `/gitflowgraph-status`, \
                 `/gitflowgraph-graph [n]`, `/gitflowgraph-diff <range>`, \
                 `/gitflowgraph-changelog <range>`, `/gitflowgraph-risk <range>`."
            )),
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
    range.chars().all(|c| {
        c.is_alphanumeric()
            || matches!(c, '.' | '/' | '-' | '_' | '~' | '^' | '@' | '{' | '}')
    })
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

fn fetch_local_get(path: &str) -> std::result::Result<String, String> {
    // Build a GET against the runtime's loopback address. Localhost
    // requests inherit Zed's HTTP client, which permits private-IP
    // destinations — this is how other extensions reach LSP sidecars.
    let req = HttpRequest {
        method: HttpMethod::Get,
        url: format!("http://127.0.0.1:{port}{path}", port = RUNTIME_PORT),
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
             project — make sure the workspace contains a `.git` directory.",
            port = RUNTIME_PORT
        )
    })?;
    Ok(String::from_utf8_lossy(&resp.body).into_owned())
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
         More views: `/gitflowgraph-graph 20` · `/gitflowgraph-status` · \
         `/gitflowgraph-diff HEAD~1..HEAD` · `/gitflowgraph-changelog v1.0..HEAD` · \
         `/gitflowgraph-risk v1.0..HEAD`\n\n\
         Full interactive UI (lane graph, structural diff, hotfix wizard) is at \
         [http://localhost:{port}](http://localhost:{port}).\n",
        port = RUNTIME_PORT
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
        port = RUNTIME_PORT
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

zed::register_extension!(GitFlowGraphExtension);
