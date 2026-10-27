# Architecture

The Cargo workspace currently has `neon-core` for XDG desktop application discovery and `neon-session` for CLI launch integration. A separate, Neon-owned Hyprland config starts the preview session. Existing user Hyprland configuration is never sourced.

```mermaid
flowchart LR
  DM[Display manager] --> S[Neon session wrapper]
  S --> H[Hyprland with Neon config]
  H --> W[Waybar and Wofi]
  C[neon-session CLI] --> X[XDG desktop entries]
  C --> G[gtk-launch]
```

Future shell surfaces should communicate through a compositor adapter. Hyprland IPC is the initial adapter; Smithay remains a separate experimental track. No custom compositor exists yet.
