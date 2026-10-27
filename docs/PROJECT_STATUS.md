# Project status — 2026-10-27

Current milestone: **0 — Research and environment (in progress)**.

| Feature | Status | Evidence |
|---|---|---|
| Host research | Implemented | `docs/DEVELOPMENT_ENVIRONMENT.md` |
| Rust workspace and app discovery | Implemented | Unit tests and CLI |
| Preview session files | Implemented | Session wrapper and config; login untested |
| Installer and uninstaller | In progress | Dry run only; real installation untested |
| Native Dock and menu bar | Planned | No implementation |
| Custom compositor | Planned | No implementation |
| GitHub repository | Blocked | Invalid GitHub CLI tokens |

Known failures: Clippy initially found two style issues; fixed before final verification. The display manager login flow and NVIDIA rendering have not been tested. The preview session relies on Waybar and Wofi and is not yet the intended Neon user experience.

Next engineering task: implement a native GTK4 layer-shell panel and Dock, then validate a real login to the isolated preview session.
