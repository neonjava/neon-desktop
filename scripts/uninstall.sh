#!/usr/bin/env bash
set -euo pipefail
for file in "$HOME/.local/bin/neon-session" "$HOME/.local/share/neon-desktop/hypr/neon.conf"; do
  if [[ -e "$file" ]]; then rm -- "$file"; fi
done
if [[ -e /usr/local/bin/neon-session-start ]] && grep -q "Neon Desktop session wrapper" /usr/local/bin/neon-session-start; then sudo rm -- /usr/local/bin/neon-session-start; fi
if [[ -e /usr/share/wayland-sessions/neon.desktop ]] && grep -q "Neon Desktop (Preview)" /usr/share/wayland-sessions/neon.desktop; then sudo rm -- /usr/share/wayland-sessions/neon.desktop; fi
echo 'Neon-owned files removed.'
