use std::fs;
use zed_extension_api::{self as zed, LanguageServerId, Result, SlashCommand, SlashCommandOutput};

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
            "gitflowgraph" => Ok(SlashCommandOutput {
                text: format!(
                    "## GitFlowGraph\n\n\
                     The interactive Git graph is available at \
                     [http://localhost:{port}](http://localhost:{port}).\n\n\
                     **Tips**\n\
                     - The runtime starts automatically when you open a Git project.\n\
                     - Press `Cmd+R` inside the panel to refresh the graph.\n\
                     - Click any commit node to view its diff and metadata.",
                    port = RUNTIME_PORT
                ),
                sections: vec![],
            }),

            "gitflowgraph-diff" => {
                let range = args.first().cloned().unwrap_or_else(|| "HEAD~1..HEAD".to_string());

                if !is_valid_ref_range(&range) {
                    return Err(format!(
                        "GitFlowGraph: invalid ref range `{range}`. \
                         Examples: `HEAD~1..HEAD`, `main..feature/my-branch`, `abc1234`."
                    ));
                }

                let encoded = url_encode_range(&range);
                Ok(SlashCommandOutput {
                    text: format!(
                        "## GitFlowGraph Smart Diff\n\n\
                         **Range:** `{range}`\n\n\
                         Open the semantic diff viewer: \
                         [http://localhost:{port}/diff?range={encoded}](http://localhost:{port}/diff?range={encoded})\n\n\
                         > Smart Diff highlights structural changes in YAML, JSON, and Terraform — \
                         not just raw line deltas.",
                        range = range,
                        port = RUNTIME_PORT,
                        encoded = encoded,
                    ),
                    sections: vec![],
                })
            }

            other => Err(format!(
                "GitFlowGraph: unknown command `{other}`. \
                 Available: `/gitflowgraph`, `/gitflowgraph-diff <range>`."
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
    if !fs::metadata(local_binary).map_or(false, |m| m.is_file()) {
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

zed::register_extension!(GitFlowGraphExtension);
