#!/usr/bin/env bash
set -euo pipefail
root="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
if [[ "${1:-}" == "--dry-run" ]]; then
  printf 'Would build Rust workspace, install Neon-owned files in ~/.local and register /usr/share/wayland-sessions/neon.desktop\n'
  exit 0
fi
if [[ "$(. /etc/os-release; echo "$ID")" != fedora ]]; then echo 'Fedora is required' >&2; exit 1; fi
for command in cargo Hyprland waybar wofi hyprlock gtk-launch nautilus ptyxis; do
  if ! command -v "$command" >/dev/null; then echo "Missing dependency: $command" >&2; exit 1; fi
done
# Refuse to replace system files that are not already Neon-owned.
for path in /usr/local/bin/neon-session-start /usr/share/wayland-sessions/neon.desktop; do
  if [[ -e "$path" ]] && ! grep -q 'Neon Desktop' "$path"; then
    echo "Refusing to overwrite unrelated file: $path" >&2
    exit 1
  fi
done
cargo build --release --manifest-path "$root/Cargo.toml"
install -Dm755 "$root/target/release/neon-session" "$HOME/.local/bin/neon-session"
sudo install -Dm755 "$root/scripts/neon-session" /usr/local/bin/neon-session-start
install -Dm644 "$root/packaging/hypr/neon.conf" "$HOME/.local/share/neon-desktop/hypr/neon.conf"
sudo install -Dm644 "$root/packaging/sessions/neon.desktop" /usr/share/wayland-sessions/neon.desktop
echo 'Neon Desktop (Preview) is registered. Select it at the login screen.'
