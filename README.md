<div align="center">

<img src="docs/assets/logo.png" alt="GitFlowGraph" width="88" />

# GitFlowGraph

**专为 Zed 打造的 Git 可视化工作区**

在编辑器内完整了解你的代码库历史——提交图谱、智能 Diff、发布管理，无需离开 Zed。

[![Zed Extension](https://img.shields.io/badge/Zed-Extension-1f6feb?labelColor=0d1117)](https://zed.dev/extensions?q=gitflowgraph)
[![Version](https://img.shields.io/badge/version-0.1.2-238636?labelColor=0d1117)](https://github.com/DevEloLin/GitFlowGraph/releases)
[![License: MIT](https://img.shields.io/badge/license-MIT-1f6feb?labelColor=0d1117)](LICENSE)
[![Platform](https://img.shields.io/badge/platform-macOS%20%7C%20Linux-6e7681?labelColor=0d1117)](https://github.com/DevEloLin/GitFlowGraph/releases)

<br/>

<img src="docs/assets/screenshot-graph.png" alt="GitFlowGraph — Git 提交图谱" width="900" />

<sub>带颜色编码泳道、分支标签和内联 Diff 面板的 Git 提交图谱</sub>

<br/>

</div>

---

## 简介

GitFlowGraph 是一个 [Zed](https://zed.dev) 扩展，将完整的 Git 可视化工作区嵌入编辑器。它取代了在终端、浏览器 Git UI 与 IDE 之间来回切换的工作方式，让你在同一个面板内浏览历史、理解变更、管理发布。

扩展本体（此仓库）采用 **MIT 开源协议**。驱动提交图谱、Smart Diff 和发布管理的运行时引擎以预编译二进制形式单独分发。

---

## Git 提交图谱

交互式多泳道提交图，可视化完整的仓库历史。

- **SVG 泳道渲染** — 每条分支独立颜色，合并提交以贝塞尔曲线连接
- **HEAD 高亮** — 当前检出位置标注发光圆环
- **分支与 Tag 标签** — 每个相关提交上以彩色胶囊标注
- **环境部署徽章（⬡）** — 在提交旁标记已部署到的命名环境
- **基础设施变更标记（⚠️）** — 自动识别 `infra:`、`chore(k8s):`、`terraform:`、`helm:` 等前缀的提交
- **骨架屏加载动画** — 数据加载期间显示占位骨架

### 过滤器

- 纯文本或正则表达式搜索提交消息
- 按作者、分支、日期范围、文件路径过滤
- 命名过滤预设 — 保存并快速恢复任意过滤组合

### 提交右键菜单

| 操作 | 说明 |
|------|------|
| 复制哈希 / 消息 / 作者 | 写入剪贴板 |
| Checkout | 切换到该提交或分支 |
| 创建分支 | 从任意提交新建分支 |
| 创建 Tag | 注释型或轻量型 |
| Cherry-pick | 应用到当前分支 |
| Revert | 生成回滚提交 |
| Merge | 将分支合入当前分支 |
| Push | 推送分支到远程 |
| 重命名 / 删除分支 | 分支管理 |
| 删除 Tag | Tag 管理 |
| Reset（Soft / Mixed / Hard） | 移动 HEAD |

---

## 提交详情面板

点击任意提交打开侧边详情面板。

- 完整元数据：哈希、作者、邮箱、时间、父提交 ID
- 文件列表：含 A / M / D / R 状态徽章及增减行数
- 目录树：可展开折叠的变更文件结构
- 二进制文件检测
- **统一 Diff** — 标准行级差异视图
- **左右并排 Diff** — 同步滚动，支持字符级行内高亮
- **文件历史** — 显示选中文件被修改的所有提交（最多 50 条）

---

## Smart Diff（智能差异对比）

三种上下文感知 Diff 模式，全部支持并排视图与字符级行内高亮。

### 开发 Diff（Development Diff）

审查进行中的工作变更。

| 预设 | 说明 |
|------|------|
| 工作区 | 未暂存变更 vs 暂存区 |
| 已暂存 | 暂存区变更 vs HEAD |
| 最近提交 | HEAD~1 → HEAD |
| vs origin/\* | 分支与基准的分歧（三点 merge-base） |
| 自定义 | 任意 base → head ref 对比 |

### 发布 Diff（Release Diff）

审查即将发布的内容。

- 选择任意 Tag、分支或提交作为 From / To
- 结构化文件（`.yaml`、`.yml`、`.json`、`.tf`、`.tfvars`）自动显示**语义分析**侧边栏 — 展示哪些键被添加、删除、修改或移动，并附旧值 / 新值

### 工作流 Diff（Workflow Diff）

审查 CI/CD 和基础设施自动化变更。

- 自动过滤，仅展示自动化相关文件
- 文件按类别分组：

| 类别 | 匹配规则 |
|------|---------|
| GitHub Actions | `.github/workflows/`、`.github/actions/` |
| Docker | `Dockerfile*`、`docker-compose*` |
| Terraform | `*.tf`、`*.tfvars` |
| Kubernetes | `k8s/`、`kubernetes/`、`manifests/` |
| Helm | `helm/`、`charts/` |
| CI 流水线 | `.gitlab-ci.yml`、`Jenkinsfile`、`.circleci/`、`bitbucket-pipelines*` |

工作流 Diff 中的结构化文件同样自动显示语义分析侧边栏。

---

## 发布管理（Release）

包含五个子标签页的发布专属仪表板。

### 变更日志（Changelog）

- 对比任意两个 ref：显示领先 / 落后提交数
- **发布风险评分**（0–100，低 / 中 / 高三级）及文件级风险因素明细
- 变更日志按类别整理：Breaking Changes · Features · Bug Fixes · Infrastructure · Other
- 支持结构化视图与 Markdown 格式切换

### 操作（Actions）

自动化发布前检查：

- **Hotfix 同步** — 扫描所有 `hotfix/*` 分支，显示每个分支是否已合入 `main`、`develop`、`master`
- **发布说明检查** — 列出近期缺少注释消息的轻量 Tag

### 环境（Environments）

将命名环境映射到分支或 Tag，并显示：

- 并排流水线网格（如 Dev → Staging → Production）
- 相邻环境间的实时领先 / 落后提交数

### 速度仪表板（Velocity Dashboard）

从 Git 历史计算指标，无需任何外部集成。

| 指标 | 说明 |
|------|------|
| 总发布次数 | 版本 Tag 数量统计 |
| 平均发布间隔 | 平均发布周期（天） |
| Hotfix 数量及占比 | `hotfix/*` 分支占总发布的百分比 |
| 每次发布平均提交数 | 单次发布的变更体量 |
| 最近发布记录表 | Tag、日期、提交数、距上次天数 |

### Hotfix 向导（Hotfix Wizard）

4 步引导式紧急修复工作流：

1. **选择生产 ref** — 当前上线状态
2. **命名分支** — 遵循 Git 命名规则校验
3. **挑选提交** — 从任意来源分支 Cherry-pick
4. **执行** — 显示生成的脚本，逐步运行并实时展示状态，执行完成后自动运行同步检查

---

## Worktrees（工作树管理）

在编辑器内管理 Git 工作树。

- 列出所有工作树：路径、分支、HEAD 提交及状态（主 / 链接 / 锁定）
- 添加工作树（指定任意路径和分支）
- 删除工作树

---

## 工作区配置档案（Workspace Profiles）

快速切换仓库上下文的命名配置方案。

- 将仓库路径保存为命名配置档案
- 切换配置档案 — 通知宿主扩展重新加载活跃仓库
- 活跃档案以绿色指示器标注

---

## 安装

### Zed 插件市场（推荐）

1. 打开 Zed
2. 按 `⌘⇧X` 打开 **Extensions**
3. 搜索 **GitFlowGraph**
4. 点击 **Install**

运行时二进制（约 15 MB）首次使用时自动下载。

### 手动 / 开发模式

```bash
git clone https://github.com/DevEloLin/GitFlowGraph
cd GitFlowGraph
zed --install-extension .
```

---

## 在 Zed 里使用 — 完整功能 24+ 个 slash command

Zed 扩展 API 不开放 webview，所以无法把 React UI 嵌进编辑器；但**所有功能都通过 slash command 在 Assistant 面板里可达**：表格、代码块、徽章、链接全部正确渲染，**包括所有写操作**（checkout/cherry-pick/merge/push 等都直接执行）。

按 `⌘?` 打开 Assistant 面板，输入命令：

### 入口

| 命令 | 用途 |
|---|---|
| `/gitflowgraph` 或 `/gitflowgraph-help` | 概览 + 所有 slash command 列表 |

### 读视图

| 命令 | 用途 |
|---|---|
| `/gitflowgraph-status` | 工作树状态（staged / unstaged / untracked） |
| `/gitflowgraph-graph [n]` | 最近 N 笔提交 + 分支 / tag 标注 |
| `/gitflowgraph-branches` | 所有分支 + HEAD 标记 |
| `/gitflowgraph-tags` | 所有 tag + 指向的 commit |
| `/gitflowgraph-worktrees` | git worktree 列表 |
| `/gitflowgraph-license` | 授权 tier / 机器使用量 / expires_at |

### 范围 / release 分析（Pro）

| 命令 | 用途 |
|---|---|
| `/gitflowgraph-diff <range>` | 文件级 diff 摘要 |
| `/gitflowgraph-changelog <range>` | 自动 changelog |
| `/gitflowgraph-risk <range>` | 发布风险评分 + 因子 |
| `/gitflowgraph-launchpad <range>` | compare + risk + checklist + changelog 一次出全 |
| `/gitflowgraph-velocity` | release 频率仪表盘 |
| `/gitflowgraph-file-history <path>` | 文件所有提交 |

### 写操作（直接执行，**无需切到浏览器**）

| 命令 | 用途 |
|---|---|
| `/gitflowgraph-checkout <ref>` | 切分支 / tag / commit |
| `/gitflowgraph-cherry-pick <sha>` | Cherry-pick |
| `/gitflowgraph-revert <sha>` | 创建反向 commit |
| `/gitflowgraph-merge <branch>` | 合并分支到当前 HEAD |
| `/gitflowgraph-reset <soft\|mixed\|hard> <sha>` | 重置 HEAD |
| `/gitflowgraph-fetch [remote]` | Fetch（默认 origin） |
| `/gitflowgraph-push <branch> [remote]` | Push |
| `/gitflowgraph-create-branch <name> [from]` | 创建分支 |
| `/gitflowgraph-delete-branch <name> [--force]` | 删除分支 |
| `/gitflowgraph-create-tag <name> [from] [--annotated <msg>]` | 创建 tag |
| `/gitflowgraph-delete-tag <name>` | 删除 tag |
| `/gitflowgraph-trial-start` | 启动 30 天 Pro 免费试用 |

每个写命令的成功反馈：`✓ <action>` + 提示浏览器侧 ⌘R 刷新；失败：含原因（LS 402 会被翻译为 "requires Pro"）。

> Free 用户：`/gitflowgraph-graph` 上限 500 条；写操作会被服务端 402 拒绝，slash 命令会显示 "Open the License tab to activate"。

### 完整可视化 UI（可选）

需要泳道图、并排 diff、Hotfix Wizard 可视化向导时，访问 **http://localhost:9876**。所有 slash command 调用的就是同一个 runtime，数据完全一致。

把 [`examples/tasks.json`](examples/tasks.json) + [`examples/keymap.json`](examples/keymap.json) 内容追加到 `~/.config/zed/{tasks,keymap}.json` 后，可一键调起浏览器：

| 快捷键 | 行为 |
|---|---|
| `⌘⇧G` | 在浏览器打开 GitFlowGraph 主面板 |
| `⌘⇧D` | 直接打开 Smart Diff 标签 |
| `⌘⇧R` | 直接打开 Release 标签 |

---

## 隐私

- **所有 Git 操作均在本地运行** — 提交数据、Diff 内容、文件内容均不会离开你的机器
- **零遥测** — 无分析统计、无崩溃上报、无使用追踪

---

## 贡献

扩展本体（`src/`）采用 MIT 协议，欢迎贡献。

```bash
git clone https://github.com/DevEloLin/GitFlowGraph
cd GitFlowGraph
cargo build
```

提交大型变更前请先开 Issue 讨论。

---

## 支持

| | 渠道 |
|---|------|
| 🐛 | [GitHub Issues](https://github.com/DevEloLin/GitFlowGraph/issues) — Bug 反馈与功能建议 |

---

## 许可证

此仓库中的 Zed 扩展本体基于 **[MIT 许可证](LICENSE)** 发布。

---

<div align="center">

为 Zed 社区构建 &nbsp;·&nbsp; [gitflowgraph.dev](https://github.com/DevEloLin/GitFlowGraph)

</div>
