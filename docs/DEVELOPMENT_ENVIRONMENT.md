# Development environment — 2026-10-28

- Fedora Linux 44 Workstation; DNF with Fedora plus third-party repositories.
- Rust 1.99.0, Cargo 1.99.0, Git 2.54.0, GitHub CLI 2.92.0.
- Active Wayland session: Hyprland. GNOME and Hyprland session entries present; KDE user configuration exists, but a KDE session entry was not found.
- Intel Raptor Lake-S UHD on i915; NVIDIA RTX 4060 Mobile on proprietary `nvidia` driver. Rendering and multi-monitor behavior have not been validated in a Neon login session.
- Hyprland, Waybar, Wofi, Hyprpaper, Hyprlock, Nautilus, and gtk-launch available. GTK4 development pkg-config metadata was not detected.
- Existing `~/.config/hypr` and GNOME/KDE user configuration were inspected for presence only and not modified.
- `gh auth status` confirms active `neonjava` authentication. The public repository was created and pushed. A stale inactive RustMC token remains irrelevant. `sudo -n` requires a password, so development packages could not be installed in this run.
- Upstream references: [Hyprland IPC](https://wiki.hypr.land/ipc/), [Hyprland config keywords](https://wiki.hypr.land/0.49.0/Configuring/Keywords/), [gtk-rs Fedora setup](https://gtk-rs.org/gtk4-rs/stable/latest/book/installation_linux.html).
