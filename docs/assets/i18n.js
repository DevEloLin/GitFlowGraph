/* GitFlowGraph — Bilingual i18n (English first, Chinese available)
 * --------------------------------------------------------------
 * Element conventions:
 *   <el data-i18n="key">              → textContent = dict[key]
 *   <el data-i18n-html="key">         → innerHTML  = dict[key]   (allows <br>, <strong>, <code>)
 *   <el data-i18n-attr="title:key">   → setAttribute('title', dict[key])
 *   <html data-i18n-title="key">      → document.title shortcut
 */

const I18N_DICT = {
  en: {
    /* ───── Meta ───── */
    'meta.title': 'GitFlowGraph — Release-Ready Git Workspace for Zed',
    'meta.titlePricing': 'Pricing — GitFlowGraph',
    'meta.description': 'Interactive commit graph, semantic Smart Diff (YAML / JSON / Terraform), release management, multi-repo, file timeline and Hotfix Wizard — all embedded in Zed.',
    'meta.descriptionPricing': 'GitFlowGraph pricing. Start free, unlock release management, Smart Diff and multi-repo workflows with Pro.',

    /* ───── Nav ───── */
    'nav.features': 'Features',
    'nav.pricing': 'Pricing',
    'nav.docs': 'Docs',
    'nav.github': 'GitHub',
    'nav.installFree': 'Install Free',
    'nav.lang.en': 'EN',
    'nav.lang.zh': '中文',
    'nav.menuLabel': 'Primary',
    'nav.langLabel': 'Language',
    'nav.toggleLabel': 'Open menu',

    /* ───── Hero ───── */
    'hero.badge': 'Zed Extension · Release-ready',
    'hero.title': 'Master your Git history',
    'hero.titleHighlight': 'inside Zed',
    'hero.sub': 'Interactive commit graph, three-mode Smart Diff, release workspace and a guided Hotfix Wizard — all embedded in Zed. No editor switching, no cloud, ever.',
    'hero.installBtn': 'Install in Zed — Free',
    'hero.githubBtn': 'View on GitHub',
    'hero.chromeUrl': 'localhost:9876',
    'hero.imageAlt': 'GitFlowGraph commit graph',

    /* ───── Strip ───── */
    'strip.label': 'Smart Diff supports',
    'strip.tag.yaml': 'YAML',
    'strip.tag.json': 'JSON',
    'strip.tag.tf': 'Terraform / HCL',
    'strip.tag.terraform': 'Terraform',
    'strip.tag.hcl': 'HCL',
    'strip.tag.k8s': 'Kubernetes',
    'strip.tag.gha': 'GitHub Actions',
    'strip.runtime': '100% local · zero telemetry',

    /* ───── Features section heading ───── */
    'features.heading': 'Everything you need to ship with confidence',
    'features.sub': 'Built for engineers who live in the editor — answers, not context switches.',

    /* Git Graph */
    'feat.graph.h': 'Git Commit Graph',
    'feat.graph.lead': 'Render the full history as an interactive multi-lane graph: SVG curves, glowing HEAD ring, branch / tag / environment badges.',
    'feat.graph.l1': 'Per-branch lane colors with Bézier-curve merge edges',
    'feat.graph.l2': 'Inline branch &amp; tag pills, environment hex badges (⬡)',
    'feat.graph.l3': '⚠ marker on infra / terraform / helm / k8s commits',
    'feat.graph.l4': 'Filters: keyword, regex, author, branch, date range, path',
    'feat.graph.l5': 'Filter Presets — save and reload any combination (Pro)',
    'feat.graph.l6': 'History limit: 500 commits (Free) · 5,000 (Pro)',
    'feat.graph.imgAlt': 'Git graph close-up',
    'feat.graph.placeholder': 'Commit graph preview',

    /* Smart Diff */
    'feat.diff.h': 'Smart Diff — three context modes',
    'feat.diff.lead': 'Stop reading walls of text. Smart Diff understands the <em>structure</em> of your files and reports what actually changed, grouped by the work you are doing.',
    'feat.diff.l1': '<strong>Development Diff</strong> — working tree, staged, last commit, vs origin/main, custom refs',
    'feat.diff.l2': '<strong>Release Diff</strong> — any tag/branch pair, structured files trigger semantic analysis automatically',
    'feat.diff.l3': '<strong>Workflow Diff</strong> — auto-filtered to CI/CD &amp; infra files, grouped by GitHub Actions / Docker / Terraform / K8s / Helm',
    'feat.diff.l4': 'Side-by-side viewer with synchronized scroll and intra-line character highlighting (Pro)',
    'feat.diff.codePath1': 'spec.template.containers[0].image',
    'feat.diff.codeVal1': 'nginx:1.24',
    'feat.diff.codeVal1To': 'nginx:1.25',
    'feat.diff.codePath2': 'spec.template.containers[0].resources.limits.memory',
    'feat.diff.codeVal2': '128Mi',
    'feat.diff.codePath3': 'data.config.legacy_mode',
    'feat.diff.imgAlt': 'Smart Diff structural panel',
    'feat.diff.placeholder': 'Smart Diff preview',

    /* Commit Detail */
    'feat.detail.h': 'Commit Detail &amp; Diff Viewer',
    'feat.detail.lead': 'Click any commit to see full metadata, change list and diff in a side panel — never leave the graph.',
    'feat.detail.l1': 'Collapsible file tree with A / M / D / R badges and +/− line counts',
    'feat.detail.l2': 'Unified diff with token-level coloring',
    'feat.detail.l3': 'Side-by-side diff with synced scroll &amp; character-level highlights (Pro)',
    'feat.detail.l4': 'Per-file Smart toggle for YAML / JSON / Terraform (Pro)',
    'feat.detail.l5': 'Binary file detection — clean message, never garbled bytes',
    'feat.detail.imgAlt': 'Commit detail panel',
    'feat.detail.placeholder': 'Commit detail preview',

    /* Release Workspace */
    'feat.release.h': 'Release Workspace',
    'feat.release.lead': 'A six-tab dashboard built from your Git history alone — no external integrations needed.',
    'feat.release.l1': '<strong>Launchpad</strong> — one-click pre-flight check with status banner, risk score, sync table and a Markdown export (Pro)',
    'feat.release.l2': '<strong>Changelog</strong> — compare any two refs, classify by Conventional Commits, copy as Markdown (generation Pro)',
    'feat.release.l3': '<strong>Actions</strong> — hotfix sync detector across main/develop/master, lightweight-tag warnings (Pro)',
    'feat.release.l4': '<strong>Environments</strong> — pipeline grid with adjacent-pair ahead/behind, env hex badges in the graph (Pro)',
    'feat.release.l5': '<strong>Velocity</strong> — release frequency, hotfix rate, avg commits per release, scrollable history table (Pro)',
    'feat.release.l6': '<strong>Hotfix Wizard</strong> — 4-step guided flow: pick prod ref → name branch → cherry-pick → execute &amp; sync-check (Pro)',
    'feat.release.imgAlt': 'Release workspace',
    'feat.release.placeholder': 'Release dashboard preview',

    /* File Timeline */
    'feat.timeline.h': 'File Timeline',
    'feat.timeline.proTag': 'Pro',
    'feat.timeline.lead': 'Type a file path. See every commit that touched it. Click any two commits — get the side-by-side diff for that exact pair.',
    'feat.timeline.l1': 'Per-file commit history with branch/tag annotations and "most recent" marker',
    'feat.timeline.l2': 'Optional release-range filter — scope the timeline to between two refs',
    'feat.timeline.l3': 'Auto-orders the two picks by commit timestamp — click in any order',
    'feat.timeline.l4': 'Jump to Timeline straight from any Smart Diff file (Pro)',
    'feat.timeline.imgAlt': 'File timeline view',
    'feat.timeline.placeholder': 'File timeline preview',

    /* Multi-Repo */
    'feat.multirepo.h': 'Multi-Repo Release View',
    'feat.multirepo.proTag': 'Pro',
    'feat.multirepo.lead': 'Run one runtime per repository, then aggregate ahead / behind across all of them in a single table. Built for microservices and split-repo releases.',
    'feat.multirepo.l1': 'Add any number of endpoints — each pointing to a runtime + repo',
    'feat.multirepo.l2': 'Compare a base/head ref pair across every endpoint at once',
    'feat.multirepo.l3': 'Per-row status: ✓ ok · connected · ✗ error (one failure does not break the rest)',
    'feat.multirepo.l4': 'Endpoints persist in browser localStorage — re-open and the table is intact',
    'feat.multirepo.imgAlt': 'Multi-repo release view',
    'feat.multirepo.placeholder': 'Multi-repo preview',

    /* Worktrees & Profiles */
    'feat.wt.h': 'Worktrees &amp; Workspace Profiles',
    'feat.wt.lead': 'Manage parallel checkouts and switch repository contexts in seconds — no terminal hopping, no stash juggling.',
    'feat.wt.l1': '<strong>Worktrees</strong> — list, add and remove linked worktrees (Pro). MAIN / LOCKED status badges',
    'feat.wt.l2': '<strong>Workspace Profiles</strong> — name a folder once, reload as the active repo with a click',
    'feat.wt.l3': 'Branch-name validation enforces Git rules before any action runs',
    'feat.wt.l4': 'Active profile shown with a green indicator dot',
    'feat.wt.imgAlt': 'Worktrees panel',
    'feat.wt.placeholder': 'Worktrees preview',

    /* Remotes */
    'feat.remotes.h': 'Remotes &amp; Credentials',
    'feat.remotes.lead': 'A first-class credential manager for hosted Git platforms — fetch and push without ever leaving the editor.',
    'feat.remotes.l1': 'Auto-detected platform per remote — GitHub · GitLab · Self-hosted · Azure DevOps · Gitea',
    'feat.remotes.l2': 'PATs encrypted at rest with an HMAC-signed entry, masked in the UI',
    'feat.remotes.l3': 'One-click Fetch per remote · push uses the matching credential automatically',
    'feat.remotes.l4': 'Credential is never sent anywhere except the matching Git host',
    'feat.remotes.imgAlt': 'Remotes panel',
    'feat.remotes.placeholder': 'Remotes preview',

    /* ───── Install ───── */
    'install.heading': 'Up and running in 30 seconds',
    'install.s1.title': 'Open Zed Extensions',
    'install.s1.body': 'Press <kbd>⌘⇧X</kbd> (macOS) or <kbd>Ctrl+Shift+X</kbd> to open the Extensions panel.',
    'install.s2.title': 'Search GitFlowGraph',
    'install.s2.body': 'Type <strong>GitFlowGraph</strong> and click <strong>Install</strong>.',
    'install.s3.title': 'Launch',
    'install.s3.body': 'Run <code>/gitflowgraph</code> from the AI panel, then open <code>localhost:9876</code>.',

    /* ───── Privacy strip ───── */
    'privacy.local': '100% local',
    'privacy.noLeave': 'Data never leaves your machine',
    'privacy.noTel': 'Zero telemetry',
    'privacy.openSource': 'MIT-licensed extension wrapper',

    /* ───── Footer ───── */
    'footer.tagline': 'Release-ready Git workspace for Zed',
    'footer.product': 'Product',
    'footer.resources': 'Resources',
    'footer.legal': 'Legal',
    'footer.features': 'Features',
    'footer.pricing': 'Pricing',
    'footer.install': 'Install',
    'footer.docs': 'Documentation',
    'footer.github': 'GitHub',
    'footer.issues': 'Issues',
    'footer.marketplace': 'Zed Marketplace',
    'footer.terms': 'Terms',
    'footer.privacy': 'Privacy',
    'footer.soon': 'Coming soon',
    'footer.soonTooltip': 'Page is being drafted — check back soon.',
    'footer.copyright': '© 2026 GitFlowGraph. Zed extension wrapper licensed MIT.',
    'footer.contact': 'dev.elolin@gmail.com',

    /* ───── Pricing page ───── */
    'pricing.hero.badge': 'Simple, transparent pricing',
    'pricing.hero.title': 'Start free.',
    'pricing.hero.titleHighlight': 'Unlock release-ready when you need it.',
    'pricing.hero.sub': 'The Git Graph and unified diff are free forever. Smart Diff, the Release Workspace, multi-repo, file timeline and worktrees unlock with Pro.',
    'pricing.hero.note1': '30-day free trial',
    'pricing.hero.note2': 'No credit card required',
    'pricing.hero.note3': 'Cancel anytime',

    /* Plan cards */
    'plan.free.name': 'Free',
    'plan.free.price': '$0',
    'plan.free.cycle': 'forever',
    'plan.free.desc': 'Read your history. Inspect commits. Copy hashes. Forever, no account.',
    'plan.free.cta': 'Install Free',
    'plan.free.f1': 'Full Git commit graph (500 commits / request)',
    'plan.free.f2': 'Unified diff with token-level coloring',
    'plan.free.f3': 'Branch &amp; tag badges, environment hex badges',
    'plan.free.f4': 'Search — keyword, regex, author, branch',
    'plan.free.f5': 'Right-click: Copy hash / message / author · View Diff',
    'plan.free.f6': 'Read-only Release compare (ahead / behind)',
    'plan.free.f7': 'Workspace Profiles · Remotes &amp; credentials',

    'plan.pro.name': 'Pro',
    'plan.pro.priceMo': '$9.9',
    'plan.pro.priceYr': '$99',
    'plan.pro.perMo': '/mo',
    'plan.pro.perYr': '/yr',
    'plan.pro.cycle.annual': 'Annual billing',
    'plan.pro.cycle.save': 'Save 16%',
    'plan.pro.desc': 'For engineers who own the release. Everything in Free, plus the full release workspace.',
    'plan.pro.cta': 'Start 30-Day Trial',
    'plan.pro.popular': 'Most popular',
    'plan.pro.f1': 'Everything in Free, with 5,000 commits / request',
    'plan.pro.f2': 'Smart Diff — YAML / JSON / Terraform structural analysis',
    'plan.pro.f3': 'Side-by-side diff &amp; per-file Smart toggle',
    'plan.pro.f4': 'All right-click write actions (checkout, branch, tag, cherry-pick, reset, push, merge…)',
    'plan.pro.f5': 'Release Workspace — Launchpad, Changelog, Actions, Environments, Velocity, Hotfix Wizard',
    'plan.pro.f6': 'File Timeline · Multi-Repo Release View · Worktrees',
    'plan.pro.f7': 'Filter Presets &amp; advanced filters (date range / path)',

    /* Comparison table */
    'cmp.heading': 'Full feature comparison',
    'cmp.col.feature': 'Feature',
    'cmp.col.free': 'Free',
    'cmp.col.pro': 'Pro',

    'cmp.g.graph': 'Git Graph &amp; Navigation',
    'cmp.r.graphLanes': 'Commit graph with branch lanes',
    'cmp.r.commitTable': 'Hash · author · date · message columns',
    'cmp.r.headIndicator': 'HEAD glow ring &amp; row highlight',
    'cmp.r.branchTag': 'Branch &amp; tag badges',
    'cmp.r.envBadge': 'Environment hex badges (⬡)',
    'cmp.r.infraIcon': 'Infrastructure ⚠ marker',
    'cmp.r.searchBasic': 'Keyword / regex / author / branch search',
    'cmp.r.searchAdv': 'Date-range &amp; path filters',
    'cmp.r.presets': 'Filter Presets (save / reload)',
    'cmp.r.commitLimit': 'History limit per request',
    'cmp.v.500': '500',
    'cmp.v.5000': '5,000',

    'cmp.g.contextMenu': 'Right-click context menu',
    'cmp.r.copyHash': 'Copy Hash / Short Hash / Message / Author',
    'cmp.r.viewDiff': 'View Diff (open commit detail)',
    'cmp.r.checkout': 'Checkout commit / branch',
    'cmp.r.createBranchTag': 'Create Branch · Create Tag (annotated &amp; lightweight)',
    'cmp.r.cherryRevert': 'Cherry-pick · Revert',
    'cmp.r.mergePush': 'Merge Branch · Push Branch',
    'cmp.r.renameDelete': 'Rename · Delete branch · Delete tag',
    'cmp.r.reset': 'Soft / Mixed / Hard Reset',

    'cmp.g.diff': 'Diff Viewer',
    'cmp.r.unifiedDiff': 'Unified diff with token coloring',
    'cmp.r.fileTree': 'File tree with A/M/D/R badges &amp; +/− counts',
    'cmp.r.binDetect': 'Binary file detection',
    'cmp.r.sideBySide': 'Side-by-side diff &amp; intra-line highlight',
    'cmp.r.smartYaml': 'Smart Diff — YAML structural paths',
    'cmp.r.smartJson': 'Smart Diff — JSON key-path changes',
    'cmp.r.smartTf': 'Smart Diff — Terraform / HCL block diff',
    'cmp.r.smartToggle': 'Per-file Smart toggle in Commit Detail',
    'cmp.r.fileHistorySub': 'Commit Detail · File History sub-tab',

    'cmp.g.release': 'Release &amp; Multi-Repo',
    'cmp.r.releaseCompare': 'Release / Changelog Compare (read-only)',
    'cmp.r.changelog': 'Generate categorized Markdown changelog',
    'cmp.r.risk': 'Release Risk Score &amp; risk-factors panel',
    'cmp.r.launchpad': 'Release Launchpad — pre-flight checklist',
    'cmp.r.actions': 'Actions tab — hotfix sync &amp; tag check',
    'cmp.r.envs': 'Environments — pipeline comparison grid',
    'cmp.r.velocity': 'Velocity dashboard',
    'cmp.r.hotfix': 'Hotfix Wizard (4-step guided)',
    'cmp.r.fileTimeline': 'File Timeline (per-file history)',
    'cmp.r.multiRepo': 'Multi-Repo Release View',
    'cmp.r.worktrees': 'Worktrees panel',
    'cmp.r.profiles': 'Workspace Profiles',

    'cmp.g.platform': 'Runtime &amp; Platform',
    'cmp.r.local': 'Runs 100% locally — no cloud',
    'cmp.r.telemetry': 'Zero telemetry',
    'cmp.r.os': 'macOS arm64 · Linux x86_64/arm64 · Windows x86_64',
    'cmp.r.remotes': 'Remotes &amp; credential storage · Fetch',
    'cmp.r.support': 'Priority GitHub support',

    /* FAQ */
    'faq.heading': 'Frequently asked questions',
    'faq.q1': "What's included in the Free tier?",
    'faq.a1': 'The full Git commit graph (up to 500 commits per request), unified diff with token coloring, file tree, branch/tag/environment badges, keyword · regex · author · branch search, the read-only Release compare, Workspace Profiles, and the Remotes credential manager. Right-click is limited to the four Copy actions and View Diff. Forever free, no account.',
    'faq.q2': 'How does the 30-day trial work?',
    'faq.a2': 'On the License panel click <strong>Start Free Trial</strong>. Every Pro feature unlocks for 30 days, no credit card. When the trial ends, the editor reverts to Free — your data, environments, profiles and credentials all stay intact. Activate a Pro key any time to restore full access.',
    'faq.q3': 'Does my data ever leave my machine?',
    'faq.a3': 'No. The runtime is a local Rust binary on <code>localhost:9876</code>. Reading history, generating diffs, parsing YAML/JSON/Terraform — all of it runs on your machine. The only outbound network call is license activation against Lemon Squeezy (no repository data is ever sent).',
    'faq.q4': 'What is Smart Diff exactly?',
    'faq.a4': 'A semantic diff engine that understands the <em>structure</em> of YAML, JSON and Terraform/HCL files. Instead of "lines 14–15 changed" you get <code>spec.template.containers[0].image: nginx:1.24 → nginx:1.25</code> — the dot-path, old value and new value, with auto-categorisation for added / removed / modified / moved.',
    'faq.q5': 'Can I use GitFlowGraph with private repositories?',
    'faq.a5': 'Yes. The runtime reads any local Git repo from disk — public or private, online or fully offline. For network operations (push / fetch) configure a PAT in the Remotes panel; the credential is HMAC-signed and only ever sent to the matching host.',
    'faq.q6': 'What happens if I cancel Pro?',
    'faq.a6': 'You drop back to Free at the end of the billing period. The Git Graph, unified diff and Release compare keep working forever. Smart Diff, Release Workspace, File Timeline, Multi-Repo and Worktrees lock until a license is re-activated. No data is deleted.',

    /* CTA */
    'cta.heading': 'Ready to own your release flow?',
    'cta.sub': 'Install in seconds. Trial unlocks every Pro feature for 30 days — no card.',
    'cta.installFree': 'Install Free',
    'cta.startTrial': 'Start 30-Day Trial',
  },

  zh: {
    /* ───── Meta ───── */
    'meta.title': 'GitFlowGraph — 为 Zed 打造的发布级 Git 工作区',
    'meta.titlePricing': '定价 — GitFlowGraph',
    'meta.description': '交互式提交图谱、语义化 Smart Diff（YAML / JSON / Terraform）、发布管理、多仓库、文件时间线与 Hotfix 向导，全部内嵌于 Zed。',
    'meta.descriptionPricing': 'GitFlowGraph 定价方案。免费版永久可用，Pro 版解锁发布管理、Smart Diff 与多仓库工作流。',

    /* ───── Nav ───── */
    'nav.features': '功能',
    'nav.pricing': '定价',
    'nav.docs': '文档',
    'nav.github': 'GitHub',
    'nav.installFree': '免费安装',
    'nav.lang.en': 'EN',
    'nav.lang.zh': '中文',
    'nav.menuLabel': '主导航',
    'nav.langLabel': '语言切换',
    'nav.toggleLabel': '打开菜单',

    /* ───── Hero ───── */
    'hero.badge': 'Zed 扩展 · 发布就绪',
    'hero.title': '在 Zed 中完整掌握',
    'hero.titleHighlight': '你的 Git 历史',
    'hero.sub': '交互式提交图谱、三模式 Smart Diff、发布工作区与引导式 Hotfix 向导，全部内嵌于 Zed。无需切换编辑器，零云端依赖。',
    'hero.installBtn': '在 Zed 中安装 — 免费',
    'hero.githubBtn': '在 GitHub 查看',
    'hero.chromeUrl': 'localhost:9876',
    'hero.imageAlt': 'GitFlowGraph 提交图谱',

    /* ───── Strip ───── */
    'strip.label': 'Smart Diff 支持',
    'strip.tag.yaml': 'YAML',
    'strip.tag.json': 'JSON',
    'strip.tag.tf': 'Terraform / HCL',
    'strip.tag.terraform': 'Terraform',
    'strip.tag.hcl': 'HCL',
    'strip.tag.k8s': 'Kubernetes',
    'strip.tag.gha': 'GitHub Actions',
    'strip.runtime': '100% 本地运行 · 零遥测',

    /* ───── Features section heading ───── */
    'features.heading': '专注发布的工程师所需的一切',
    'features.sub': '为生活在编辑器里的工程师而生 — 要答案，不要上下文切换。',

    /* Git Graph */
    'feat.graph.h': 'Git 提交图谱',
    'feat.graph.lead': '以交互式多泳道图渲染完整历史：SVG 曲线、HEAD 发光环、分支 / Tag / 环境徽章一应俱全。',
    'feat.graph.l1': '每个分支独立泳道颜色，合并边以贝塞尔曲线连接',
    'feat.graph.l2': '内联展示分支与 Tag 胶囊、环境六边形徽章（⬡）',
    'feat.graph.l3': '⚠ 自动识别 infra / terraform / helm / k8s 提交',
    'feat.graph.l4': '过滤维度：关键字、正则、作者、分支、日期范围、路径',
    'feat.graph.l5': '过滤预设（Filter Presets）—— 保存任意组合并一键恢复（Pro）',
    'feat.graph.l6': '历史条数限制：免费 500 条 · Pro 5,000 条',
    'feat.graph.imgAlt': '提交图谱细节',
    'feat.graph.placeholder': '提交图谱示意',

    /* Smart Diff */
    'feat.diff.h': 'Smart Diff — 三种上下文模式',
    'feat.diff.lead': '不再阅读整面纯文本差异墙。Smart Diff 理解文件的<em>结构</em>，按你正在做的事分组报告真正变化的内容。',
    'feat.diff.l1': '<strong>开发 Diff</strong> —— 工作区、暂存区、最近提交、vs origin/main、自定义对比',
    'feat.diff.l2': '<strong>发布 Diff</strong> —— 任意 Tag / 分支对比，结构化文件自动触发语义分析',
    'feat.diff.l3': '<strong>工作流 Diff</strong> —— 自动筛 CI/CD 与基础设施文件，按 GitHub Actions / Docker / Terraform / K8s / Helm 分组',
    'feat.diff.l4': '左右并排视图 + 同步滚动 + 行内字符级高亮（Pro）',
    'feat.diff.codePath1': 'spec.template.containers[0].image',
    'feat.diff.codeVal1': 'nginx:1.24',
    'feat.diff.codeVal1To': 'nginx:1.25',
    'feat.diff.codePath2': 'spec.template.containers[0].resources.limits.memory',
    'feat.diff.codeVal2': '128Mi',
    'feat.diff.codePath3': 'data.config.legacy_mode',
    'feat.diff.imgAlt': 'Smart Diff 结构面板',
    'feat.diff.placeholder': 'Smart Diff 示意',

    /* Commit Detail */
    'feat.detail.h': '提交详情 &amp; Diff 查看器',
    'feat.detail.lead': '点击任意提交，立即在侧边面板看到完整元数据、变更列表与差异 —— 无需离开图谱。',
    'feat.detail.l1': '可折叠目录树，含 A / M / D / R 状态徽章及增减行数',
    'feat.detail.l2': '统一 Diff（Unified）含 token 级着色',
    'feat.detail.l3': '左右并排 Diff，同步滚动 + 字符级高亮（Pro）',
    'feat.detail.l4': '单文件 Smart 切换 —— YAML / JSON / Terraform（Pro）',
    'feat.detail.l5': '二进制文件检测，提示而非乱码',
    'feat.detail.imgAlt': '提交详情面板',
    'feat.detail.placeholder': '提交详情示意',

    /* Release Workspace */
    'feat.release.h': '发布工作区',
    'feat.release.lead': '基于 Git 历史的六个子标签页仪表板 —— 无需任何外部集成。',
    'feat.release.l1': '<strong>Launchpad</strong> —— 一键发布前自检：状态横幅、风险评分、同步表与 Markdown 导出（Pro）',
    'feat.release.l2': '<strong>变更日志</strong> —— 任意两 ref 对比，按 Conventional Commits 分类，可复制为 Markdown（生成需 Pro）',
    'feat.release.l3': '<strong>Actions</strong> —— Hotfix 同步检测（main / develop / master）+ 轻量 Tag 缺注释告警（Pro）',
    'feat.release.l4': '<strong>环境</strong> —— 流水线网格，相邻环境领先 / 落后实时显示，提交图谱叠加六边形徽章（Pro）',
    'feat.release.l5': '<strong>速度仪表板</strong> —— 发布频率、Hotfix 占比、平均提交数、最近发布滚动表（Pro）',
    'feat.release.l6': '<strong>Hotfix 向导</strong> —— 4 步引导：选生产 ref → 命名分支 → 挑选提交 → 执行并自动同步检查（Pro）',
    'feat.release.imgAlt': '发布工作区',
    'feat.release.placeholder': '发布管理示意',

    /* File Timeline */
    'feat.timeline.h': '文件时间线',
    'feat.timeline.proTag': 'Pro',
    'feat.timeline.lead': '输入文件路径，看到所有动过它的提交。点选任意两条提交，立刻获得这两点之间该文件的并排 Diff。',
    'feat.timeline.l1': '单文件提交历史，附分支 / Tag 注释与「最近一次」标记',
    'feat.timeline.l2': '可选发布区间过滤 —— 限定在两 ref 之间的范围',
    'feat.timeline.l3': '点选自动按时间排序 —— 任意先后皆可',
    'feat.timeline.l4': '从任意 Smart Diff 文件一键跳转 Timeline（Pro）',
    'feat.timeline.imgAlt': '文件时间线视图',
    'feat.timeline.placeholder': '文件时间线示意',

    /* Multi-Repo */
    'feat.multirepo.h': '多仓库发布视图',
    'feat.multirepo.proTag': 'Pro',
    'feat.multirepo.lead': '为每个仓库各跑一个运行时，然后在一张表里汇总所有仓库的领先 / 落后状态。专为微服务与拆库发布而设计。',
    'feat.multirepo.l1': '可添加任意数量端点 —— 每个端点指向一个运行时 + 仓库',
    'feat.multirepo.l2': '一次设定 base / head ref 对，跨所有端点同时对比',
    'feat.multirepo.l3': '逐行状态：✓ 正常 · 已连接 · ✗ 错误（单点失败不影响其他）',
    'feat.multirepo.l4': '端点持久化在浏览器 localStorage，重新打开即恢复',
    'feat.multirepo.imgAlt': '多仓库发布视图',
    'feat.multirepo.placeholder': '多仓库示意',

    /* Worktrees & Profiles */
    'feat.wt.h': 'Worktrees &amp; 工作区档案',
    'feat.wt.lead': '管理并行检出，秒切仓库上下文 —— 无需开终端，无需 stash 来回折腾。',
    'feat.wt.l1': '<strong>Worktrees</strong> —— 列出 / 添加 / 删除链接工作树（Pro），主 / 锁定状态徽章',
    'feat.wt.l2': '<strong>工作区档案</strong> —— 一次命名目录，一键作为活跃仓库重新加载',
    'feat.wt.l3': '分支命名校验在执行前强制 Git 命名规则',
    'feat.wt.l4': '当前活跃档案以绿色圆点标识',
    'feat.wt.imgAlt': 'Worktrees 面板',
    'feat.wt.placeholder': 'Worktrees 示意',

    /* Remotes */
    'feat.remotes.h': '远程仓库 &amp; 凭证管理',
    'feat.remotes.lead': '为各类托管平台准备的一等公民凭证管理器 —— 在编辑器内完成 Fetch 与 Push。',
    'feat.remotes.l1': '逐个 remote 自动识别平台 —— GitHub · GitLab · 自托管 · Azure DevOps · Gitea',
    'feat.remotes.l2': 'PAT 以 HMAC 签名持久化，UI 中始终掩码显示',
    'feat.remotes.l3': '每个 remote 一键 Fetch · Push 自动匹配对应凭证',
    'feat.remotes.l4': '凭证仅发往匹配的 Git 主机，绝不外传',
    'feat.remotes.imgAlt': 'Remotes 面板',
    'feat.remotes.placeholder': 'Remotes 示意',

    /* ───── Install ───── */
    'install.heading': '30 秒上手',
    'install.s1.title': '打开 Zed 扩展面板',
    'install.s1.body': '按 <kbd>⌘⇧X</kbd>（macOS）或 <kbd>Ctrl+Shift+X</kbd> 打开 Extensions 面板。',
    'install.s2.title': '搜索 GitFlowGraph',
    'install.s2.body': '输入 <strong>GitFlowGraph</strong> 并点击 <strong>Install</strong>。',
    'install.s3.title': '启动',
    'install.s3.body': '在 AI 面板执行 <code>/gitflowgraph</code>，然后打开 <code>localhost:9876</code>。',

    /* ───── Privacy strip ───── */
    'privacy.local': '100% 本地运行',
    'privacy.noLeave': '数据不离开你的机器',
    'privacy.noTel': '零遥测',
    'privacy.openSource': '扩展本体 MIT 开源',

    /* ───── Footer ───── */
    'footer.tagline': '为 Zed 打造的发布级 Git 工作区',
    'footer.product': '产品',
    'footer.resources': '资源',
    'footer.legal': '法律',
    'footer.features': '功能',
    'footer.pricing': '定价',
    'footer.install': '安装',
    'footer.docs': '文档',
    'footer.github': 'GitHub',
    'footer.issues': 'Issues',
    'footer.marketplace': 'Zed 插件市场',
    'footer.terms': '服务条款',
    'footer.privacy': '隐私政策',
    'footer.soon': '即将上线',
    'footer.soonTooltip': '页面正在起草中，敬请期待。',
    'footer.copyright': '© 2026 GitFlowGraph. Zed 扩展本体基于 MIT 许可证。',
    'footer.contact': 'dev.elolin@gmail.com',

    /* ───── Pricing page ───── */
    'pricing.hero.badge': '简单透明的定价',
    'pricing.hero.title': '免费即可开始。',
    'pricing.hero.titleHighlight': '需要时再解锁发布级。',
    'pricing.hero.sub': 'Git 提交图谱与统一 Diff 永久免费。Smart Diff、发布工作区、多仓库、文件时间线与 Worktrees 由 Pro 解锁。',
    'pricing.hero.note1': '30 天免费试用',
    'pricing.hero.note2': '无需信用卡',
    'pricing.hero.note3': '随时取消',

    /* Plan cards */
    'plan.free.name': '免费版',
    'plan.free.price': '¥0',
    'plan.free.cycle': '永久免费',
    'plan.free.desc': '阅读历史、检视提交、复制哈希。永久免费，无需账户。',
    'plan.free.cta': '免费安装',
    'plan.free.f1': '完整 Git 提交图谱（500 条 / 请求）',
    'plan.free.f2': '统一 Diff（Unified）+ token 级着色',
    'plan.free.f3': '分支与 Tag 徽章、环境六边形徽章',
    'plan.free.f4': '搜索 —— 关键词、正则、作者、分支',
    'plan.free.f5': '右键菜单：复制哈希 / 消息 / 作者 · 查看 Diff',
    'plan.free.f6': '只读发布对比（领先 / 落后）',
    'plan.free.f7': '工作区档案 · 远程凭证管理',

    'plan.pro.name': 'Pro',
    'plan.pro.priceMo': '¥69',
    'plan.pro.priceYr': '¥699',
    'plan.pro.perMo': ' / 月',
    'plan.pro.perYr': ' / 年',
    'plan.pro.cycle.annual': '年付',
    'plan.pro.cycle.save': '省 16%',
    'plan.pro.desc': '为对发布负责的工程师准备。包含免费版全部功能，外加完整发布工作区。',
    'plan.pro.cta': '开启 30 天试用',
    'plan.pro.popular': '最受欢迎',
    'plan.pro.f1': '免费版全部功能，单次请求 5,000 条历史',
    'plan.pro.f2': 'Smart Diff —— YAML / JSON / Terraform 结构化分析',
    'plan.pro.f3': '左右并排 Diff &amp; 单文件 Smart 切换',
    'plan.pro.f4': '右键全部写操作（Checkout、分支、Tag、Cherry-pick、Reset、Push、Merge…）',
    'plan.pro.f5': '发布工作区 —— Launchpad、变更日志、Actions、环境、速度、Hotfix 向导',
    'plan.pro.f6': '文件时间线 · 多仓库发布视图 · Worktrees',
    'plan.pro.f7': '过滤预设 &amp; 高级过滤（日期范围 / 路径）',

    /* Comparison table */
    'cmp.heading': '完整功能对比',
    'cmp.col.feature': '功能',
    'cmp.col.free': '免费版',
    'cmp.col.pro': 'Pro',

    'cmp.g.graph': '提交图谱与导航',
    'cmp.r.graphLanes': '带分支泳道的提交图',
    'cmp.r.commitTable': '哈希 · 作者 · 日期 · 消息 列',
    'cmp.r.headIndicator': 'HEAD 发光环与行高亮',
    'cmp.r.branchTag': '分支 &amp; Tag 徽章',
    'cmp.r.envBadge': '环境六边形徽章（⬡）',
    'cmp.r.infraIcon': '基础设施 ⚠ 标记',
    'cmp.r.searchBasic': '关键字 / 正则 / 作者 / 分支 搜索',
    'cmp.r.searchAdv': '日期范围 &amp; 路径过滤',
    'cmp.r.presets': '过滤预设（保存 / 重新载入）',
    'cmp.r.commitLimit': '单次请求历史条数上限',
    'cmp.v.500': '500',
    'cmp.v.5000': '5,000',

    'cmp.g.contextMenu': '右键菜单',
    'cmp.r.copyHash': '复制 哈希 / 短哈希 / 消息 / 作者',
    'cmp.r.viewDiff': '查看 Diff（打开提交详情）',
    'cmp.r.checkout': 'Checkout 提交 / 分支',
    'cmp.r.createBranchTag': '创建分支 · 创建 Tag（注释 / 轻量）',
    'cmp.r.cherryRevert': '挑选提交（Cherry-pick）· 回滚（Revert）',
    'cmp.r.mergePush': '合并分支 · 推送分支',
    'cmp.r.renameDelete': '重命名 · 删除分支 · 删除 Tag',
    'cmp.r.reset': '软 / 混合 / 硬重置（Soft / Mixed / Hard Reset）',

    'cmp.g.diff': 'Diff 查看器',
    'cmp.r.unifiedDiff': '统一 Diff + token 着色',
    'cmp.r.fileTree': '文件树（A/M/D/R 徽章 + 增减行数）',
    'cmp.r.binDetect': '二进制文件检测',
    'cmp.r.sideBySide': '左右并排 Diff &amp; 行内高亮',
    'cmp.r.smartYaml': 'Smart Diff —— YAML 结构路径',
    'cmp.r.smartJson': 'Smart Diff —— JSON 键路径变更',
    'cmp.r.smartTf': 'Smart Diff —— Terraform / HCL 块对比',
    'cmp.r.smartToggle': '提交详情中单文件 Smart 切换',
    'cmp.r.fileHistorySub': '提交详情 · 文件历史子页',

    'cmp.g.release': '发布与多仓库',
    'cmp.r.releaseCompare': '发布 / 变更日志对比（只读）',
    'cmp.r.changelog': '生成分类 Markdown 变更日志',
    'cmp.r.risk': '发布风险评分 &amp; 风险因素面板',
    'cmp.r.launchpad': '发布 Launchpad —— 发布前自检清单',
    'cmp.r.actions': 'Actions 标签 —— Hotfix 同步 &amp; Tag 检查',
    'cmp.r.envs': '环境 —— 流水线对比网格',
    'cmp.r.velocity': '速度仪表板',
    'cmp.r.hotfix': 'Hotfix 向导（4 步引导）',
    'cmp.r.fileTimeline': '文件时间线（单文件历史）',
    'cmp.r.multiRepo': '多仓库发布视图',
    'cmp.r.worktrees': 'Worktrees 面板',
    'cmp.r.profiles': '工作区档案',

    'cmp.g.platform': '运行时与平台',
    'cmp.r.local': '100% 本地运行 —— 无云端依赖',
    'cmp.r.telemetry': '零遥测',
    'cmp.r.os': 'macOS arm64 · Linux x86_64/arm64 · Windows x86_64',
    'cmp.r.remotes': '远程凭证管理 · Fetch',
    'cmp.r.support': 'GitHub 优先支持',

    /* FAQ */
    'faq.heading': '常见问题',
    'faq.q1': '免费版具体包含什么？',
    'faq.a1': '完整 Git 提交图谱（单次最多 500 条）、统一 Diff（含 token 着色）、文件树、分支 / Tag / 环境徽章，关键字 · 正则 · 作者 · 分支 搜索，只读发布对比，工作区档案，以及远程凭证管理。右键菜单仅包含 4 个复制操作与"查看 Diff"。永久免费，无需注册。',
    'faq.q2': '30 天试用如何工作？',
    'faq.a2': '在 License 面板点击 <strong>Start Free Trial</strong>。所有 Pro 功能立刻解锁 30 天，无需信用卡。试用结束后退回免费版 —— 你的数据、环境配置、档案与凭证都会原样保留。任何时候激活 Pro Key 都能恢复完整访问。',
    'faq.q3': '我的数据会离开机器吗？',
    'faq.a3': '不会。运行时是位于 <code>localhost:9876</code> 的本地 Rust 二进制。读取历史、生成 Diff、解析 YAML / JSON / Terraform，全部在你机器上完成。唯一的对外网络请求是向 Lemon Squeezy 校验 License（任何仓库数据都不会被发送）。',
    'faq.q4': 'Smart Diff 究竟是什么？',
    'faq.a4': '一个理解 YAML / JSON / Terraform / HCL 文件<em>结构</em>的语义化 Diff 引擎。不再是"第 14–15 行变了"，而是 <code>spec.template.containers[0].image: nginx:1.24 → nginx:1.25</code> —— 输出键路径、旧值、新值，并自动分类为 新增 / 删除 / 修改 / 移动。',
    'faq.q5': '可以用于私有仓库吗？',
    'faq.a5': '可以。运行时直接从磁盘读取任何本地 Git 仓库 —— 无论公共还是私有，无论联网与否。Push / Fetch 等网络操作可在 Remotes 面板配置 PAT；凭证经 HMAC 签名，仅会被发往对应的 Git 主机。',
    'faq.q6': '取消 Pro 后会怎样？',
    'faq.a6': '当前计费周期结束后退回免费版。Git 图谱、统一 Diff、发布对比永久可用。Smart Diff、发布工作区、文件时间线、多仓库与 Worktrees 在重新激活 License 之前不可用。所有数据保留。',

    /* CTA */
    'cta.heading': '准备好掌控你的发布流了吗？',
    'cta.sub': '秒级安装。试用解锁所有 Pro 功能 30 天 —— 无需信用卡。',
    'cta.installFree': '免费安装',
    'cta.startTrial': '开启 30 天试用',
  },
};

/* ───────────── Runtime ───────────── */
const I18N_KEY = 'gitflowgraph.lang';

function detectInitialLang() {
  const saved = (() => {
    try { return localStorage.getItem(I18N_KEY); } catch (_) { return null; }
  })();
  if (saved && I18N_DICT[saved]) return saved;
  // English first by request, regardless of browser locale.
  return 'en';
}

function resolveValue(dict, key) {
  const v = dict[key];
  if (typeof v === 'string') return v;
  // Surface missing translations during development without breaking
  // production. Each missing key is warned at most once per page load.
  if (!resolveValue._warned) resolveValue._warned = new Set();
  if (!resolveValue._warned.has(key)) {
    resolveValue._warned.add(key);
    if (typeof console !== 'undefined') {
      console.warn('[i18n] missing key:', key);
    }
  }
  return null;
}

function applyLang(lang) {
  if (!I18N_DICT[lang]) lang = 'en';
  const dict = I18N_DICT[lang];
  const htmlEl = document.documentElement;
  htmlEl.setAttribute('lang', lang === 'zh' ? 'zh-CN' : 'en');

  /* textContent */
  document.querySelectorAll('[data-i18n]').forEach((el) => {
    const val = resolveValue(dict, el.getAttribute('data-i18n'));
    if (val !== null) el.textContent = val;
  });

  /* innerHTML — explicit opt-in via data-i18n-html */
  document.querySelectorAll('[data-i18n-html]').forEach((el) => {
    const val = resolveValue(dict, el.getAttribute('data-i18n-html'));
    if (val !== null) el.innerHTML = val;
  });

  /* attribute mapping: data-i18n-attr="title:key1,placeholder:key2" */
  document.querySelectorAll('[data-i18n-attr]').forEach((el) => {
    const spec = el.getAttribute('data-i18n-attr');
    spec.split(',').forEach((pair) => {
      const [attr, key] = pair.split(':').map((s) => s.trim());
      const val = resolveValue(dict, key);
      if (attr && val !== null) el.setAttribute(attr, val);
    });
  });

  /* document title shortcut */
  const titleKey = htmlEl.getAttribute('data-i18n-title');
  if (titleKey) {
    const t = resolveValue(dict, titleKey);
    if (t !== null) document.title = t;
  }

  /* meta description + Open Graph: keep social previews localized */
  const metaTargets = [
    { selector: 'meta[name="description"]', attr: 'content' },
    { selector: 'meta[property="og:title"]', attr: 'content' },
    { selector: 'meta[property="og:description"]', attr: 'content' },
    { selector: 'meta[name="twitter:title"]', attr: 'content' },
    { selector: 'meta[name="twitter:description"]', attr: 'content' },
  ];
  metaTargets.forEach(({ selector, attr }) => {
    document.querySelectorAll(selector).forEach((el) => {
      const key = el.getAttribute('data-i18n-content');
      if (!key) return;
      const v = resolveValue(dict, key);
      if (v !== null) el.setAttribute(attr, v);
    });
  });

  /* Reflect active state on the language switch buttons */
  document.querySelectorAll('[data-lang-switch]').forEach((btn) => {
    const target = btn.getAttribute('data-lang-switch');
    btn.classList.toggle('lang-active', target === lang);
    btn.setAttribute('aria-pressed', target === lang ? 'true' : 'false');
  });

  /* Re-apply pricing toggle state (annual/monthly) so labels stay coherent */
  if (typeof window.refreshBillingLabels === 'function') {
    window.refreshBillingLabels();
  }

  try { localStorage.setItem(I18N_KEY, lang); } catch (_) { /* noop */ }

  // Reveal the page (works in tandem with the pre-paint inline script that
  // hides body when the saved preference is `zh`).
  htmlEl.classList.add('i18n-ready');
  htmlEl.classList.remove('lang-pending-zh');

  window.dispatchEvent(new CustomEvent('i18n:changed', { detail: { lang } }));
}

function setupLangSwitch() {
  document.querySelectorAll('[data-lang-switch]').forEach((btn) => {
    btn.addEventListener('click', (e) => {
      e.preventDefault();
      const lang = btn.getAttribute('data-lang-switch');
      applyLang(lang);
    });
  });
}

function setupSoonLinks() {
  // Anchors marked with aria-disabled="true" point at "#" today; without an
  // explicit guard they would scroll the page back to the top on click,
  // which is jarring. Intercept clicks site-wide.
  document.addEventListener('click', (e) => {
    const target = e.target instanceof Element
      ? e.target.closest('a[aria-disabled="true"]')
      : null;
    if (target) e.preventDefault();
  });
}

function setupNavToggle() {
  const toggle = document.querySelector('.nav-toggle');
  const nav = document.getElementById('primary-nav');
  if (!toggle || !nav) return;

  function setOpen(open) {
    toggle.setAttribute('aria-expanded', open ? 'true' : 'false');
    document.body.classList.toggle('nav-open', open);
  }

  toggle.addEventListener('click', () => {
    const isOpen = toggle.getAttribute('aria-expanded') === 'true';
    setOpen(!isOpen);
  });

  // Close when a nav link is followed
  nav.addEventListener('click', (e) => {
    if (e.target instanceof HTMLAnchorElement) setOpen(false);
  });

  // Close on Escape (a11y)
  document.addEventListener('keydown', (e) => {
    if (e.key === 'Escape') setOpen(false);
  });

  // Close when the viewport grows past the mobile breakpoint, otherwise the
  // open-state class can leak and visually corrupt the desktop nav.
  const mql = window.matchMedia('(min-width: 769px)');
  if (mql.addEventListener) mql.addEventListener('change', (e) => { if (e.matches) setOpen(false); });
}

// Safety net: if applyLang() never runs (script error, browser blocking),
// reveal the page anyway after 700ms so the user is never stuck on a blank
// screen.
setTimeout(() => {
  document.documentElement.classList.add('i18n-ready');
  document.documentElement.classList.remove('lang-pending-zh');
}, 700);

document.addEventListener('DOMContentLoaded', () => {
  setupLangSwitch();
  setupNavToggle();
  setupSoonLinks();
  applyLang(detectInitialLang());
});

/* Public helper exposed for the pricing page billing toggle */
window.GFG_I18N = {
  current: () => document.documentElement.getAttribute('lang') === 'zh-CN' ? 'zh' : 'en',
  t: (key) => {
    const lang = window.GFG_I18N.current();
    return (I18N_DICT[lang] && I18N_DICT[lang][key]) || key;
  },
};
