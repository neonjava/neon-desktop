# Neon Desktop

A Rust-powered, macOS-inspired desktop environment for Linux. Beautiful by design, native by nature.

**Status: early preview, milestone 0 in progress.** This repository currently provides a native Rust application index and launcher command, and a separate Hyprland session configuration. It does not yet have a Neon Dock, menu bar, settings, file manager, notification center, or custom compositor. Do not install it expecting a finished desktop.

## Requirements

Fedora 44, Rust 1.99+, Hyprland, Waybar, Wofi, Hyprlock, GTK launch tools, and Nautilus. The installer checks required commands before writing files.

## Build and try

```sh
cargo test --workspace
cargo run -p neon-session -- apps
cargo run -p neon-session -- launch org.gnome.Nautilus.desktop
./scripts/install.sh --dry-run
```

`./scripts/install.sh` builds the Rust binary, installs Neon-owned files, and registers **Neon Desktop (Preview)** in the Wayland session chooser. It uses `sudo` only for files under `/usr/local/bin` and `/usr/share/wayland-sessions`. It does not edit existing desktop configurations. `./scripts/uninstall.sh` removes those owned files.

The session uses Super+D for Wofi, Super+E for Files, Super+L to lock, Super+Q to close a window, and Super+Shift+E to exit. See [project status](docs/PROJECT_STATUS.md) and [architecture](docs/ARCHITECTURE.md).

## Contributing

See [CONTRIBUTING.md](CONTRIBUTING.md). Issues and security reports are welcome; see [SECURITY.md](SECURITY.md).

Maintained by neonjava. MIT licensed. Neon Desktop is independent of Apple.
