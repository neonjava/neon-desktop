# Project status — 2026-10-28

Current milestone: **0 — Research and environment (in progress)**.

| Feature | Status | Evidence |
|---|---|---|
| Host research | Implemented | `docs/DEVELOPMENT_ENVIRONMENT.md` |
| Rust workspace, app discovery and search | Verified | Unit tests and CLI smoke tests |
| Preview session files | Implemented | Session wrapper and config; login untested |
| Installer and uninstaller | In progress | Dry run only; real installation untested |
| Native Dock and menu bar | Planned | No implementation |
| Custom compositor | Planned | No implementation |
| GitHub repository | Verified | Public `neonjava/neon-desktop`; `main` pushed |

The host lacks GTK4 development headers, and `sudo` requires an interactive password. No system packages were installed. The display manager login flow and NVIDIA rendering have not been tested. The preview session relies on Waybar and Wofi and is not yet the intended Neon user experience.

Next engineering task: install GTK4 development headers, implement a native GTK4 layer-shell panel and Dock, then validate a real login to the isolated preview session.
