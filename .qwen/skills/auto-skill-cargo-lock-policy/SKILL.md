---
name: cargo-lock-policy
description: >
  Dependency lock and Cargo.lock conflict policy for Rust workspaces.
  Covers when to update, when to regenerate, manual conflict handling, and
  merge/sync guidelines to prevent recurring Cargo.lock merge conflicts.
source: auto-skill
extracted_at: '2026-06-22T02:23:54.746Z'
---

# Cargo.lock / Dependency Lock Policy

## 1. 设计意图

Cargo.lock 要纳入 git 版本控制，放在 workspace root。它的作用是锁定可复现构建，因此任何会导致依赖树变化的变更，都应同步更新 lock 文件。

## 2. 何时必须更新 Cargo.lock

- 修改 `Cargo.toml`（增/删/升级依赖，改 features）
- 更新 `Cargo.toml` 中的 `reqwest`、`pyo3`、`rusqlite` 等主要 crate
- 执行 `cargo update`、`cargo upgrade` 后
- PR 中源码已改 dependency，但 lock 还未与代码同时提交

## 3. 更新 lock 的标准命令

在修改完 `Cargo.toml` 后，执行：

```bash
cargo update --workspace
cargo check --workspace
cargo test --workspace
```

## 4. 多平台下的更新说明

- Termux / Android：`cargo update --workspace` 后确认缓存正常，再提交 `Cargo.lock`
- CI pipeline：强制 lock 校验（`cargo metadata --lockfile-version` 或等价 gated check）
- Dependabot：锁定版本文件只允许 Dependabot 修改，人工变更同样提交 lock

## 5. Merge / Sync 规范

### 5.1 不要 rebase main 与 origin/main 的冲突

不要对 main 做 `git rebase` 与远端强制同步。正确做法：

1. `git fetch origin`
2. 分析双方差异（`git log --oneline origin/main..HEAD`）
3. 若仅向前合并：`git merge origin/main --no-commit --no-ff`
4. 遇 conflict：
   - 对 `Cargo.toml`：手工抉择应保留的版本（通常跟随 changelog / commit 意图）
   - 对 `Cargo.lock`：取较新那份完全有效的 lock，或者用 `cargo update --workspace` 一次性重新生成，再 `git add Cargo.lock`
5. `git commit` 完成后，再 push

### 5.2 Cargo.lock 冲突的优先策略

| 情况 | 处理 |
|------|------|
| 仅有 format change | 保留当前工作树 + checkout 的 `Cargo.lock` |
| 仅有 version bump | 保留较新锁文件；re-run cargo update 再提交 |
| 大量依赖 inserted/removed | 删除冲突部分，用 `cargo update --workspace` 重新生成 |
| 不确定哪边正确 | 跑 `cargo tree` / `cargo metadata` 验证后再提交 |

## 6. 人为操作与安全检查清单

- 修改 dependency 后：跑 `cargo check` 通过再 commit
- commit `Cargo.lock` 时必须同时 commit 对应 `Cargo.toml`
- 检查 `.gitignore` 不要把 `Cargo.lock` 放进去
- CI 中 lockfile 校验失败视为 PR merge blocker

## 7. 常见错误与修复

### 7.1 "Cargo.lock 陷入循环 merge conflict"

1. `git checkout --ours Cargo.lock` 或 `--theirs`
2. `cargo update --workspace`
3. `git add Cargo.lock && git commit`

### 7.2 "Cargo.lock 与 Cargo.toml 我的依赖冲突"

先确定最终依赖方案（锁定版本、feature flags），然后：

```bash
cargo update -p <crate_name>
```

再在 `Cargo.toml` 和 `Cargo.lock` 同时确认提交。

## 8. 项目内约定

- Rust workspace root 提交 `Cargo.lock`
- `Cargo.lock` 仅包含 workspace 的依赖集合，不需要 flush
- 已合并分支自动清理后，CI 验证 lock 一致
- 不允许 `--force` merge main：强制同步必须走 merge + 手工冲突解决
