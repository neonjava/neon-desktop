# Neon Desktop agent guide

Build a native Rust desktop for Fedora. Keep `main` buildable. Use focused crates and test real behavior. Never edit existing GNOME, KDE, or Hyprland configurations. The preview session owns separate files. Do not claim untested GUI behavior works. Run `cargo fmt --all -- --check`, `cargo clippy --workspace --all-targets -- -D warnings`, and `cargo test --workspace` before committing. Update `docs/PROJECT_STATUS.md` after each change. Review `docs/ARCHITECTURE.md` and `docs/ROADMAP.md` to resume work. Use conventional commits and MIT-compatible dependencies and assets.

## Pull request workflow

All new work goes on `feat/...`, `fix/...`, `docs/...`, or `refactor/...` branches. Commit and push the branch, open a pull request into `main`, and wait for CI and review. Do not push directly to `main` or merge a PR unless the user explicitly requests the merge.

## Code Review Rules

### Session isolation

Flag changes that read, overwrite, or delete a user's existing GNOME, KDE, or Hyprland configuration. Neon session files must remain separate and uninstall must remove only Neon-owned files.

### Functional claims

Flag a feature described as working or verified when the PR contains only a mock interface, a static asset, or untested integration. Require evidence for login, launching, and system-control claims.

### Launcher execution

Flag application launching that interpolates desktop-entry content into a shell command. Use an established desktop-entry launcher or safe argument handling.
