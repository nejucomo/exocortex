# Exocortex

A micro-note taking app for quick **jots** — single-line plain-text notes.

## Features

- **Quick jot** — a global hotkey (`Ctrl+Shift+J`) pops open a lightweight overlay for capturing a thought instantly; the app UX context is restored exactly as it was when the dialog closes.
- **Jots view** — browse all jots with case-insensitive substring search and timestamp range filters.
- **Action log** — an append-only log of every user-level event (add, edit) with timestamps; all views are derived by replaying this log.
- **Headless CLI** — scriptable, non-interactive commands for every major operation.
- **Persistent storage** — jots and the action log live in the platform data directory (`~/.local/share/exocortex/` on Linux).

---

## Building

### With Nix (recommended)

```sh
# Build the binary
nix build

# Run it
./result/bin/exocortex

# Enter the development shell (Cargo-based workflow)
nix develop --command cargo build
nix develop --command cargo run
```

### Without Nix

Install a recent stable Rust toolchain, then:

```sh
cargo build --release
./target/release/exocortex
```

On Linux you need the following system libraries at link time and runtime:
`libGL`, `libxkbcommon`, `libwayland-client`, `libX11`, `libXcursor`, `libXi`,
`libXrandr`, `libXtst`.

---

## CLI usage

```sh
# Add a jot
exocortex add "remember to call dentist"

# Show the action log
exocortex log

# List all jots
exocortex view

# Filter by substring (case-insensitive)
exocortex view --search dentist

# Filter by timestamp range (RFC 3339)
exocortex view --from 2024-01-01T00:00:00Z --to 2024-12-31T23:59:59Z
```

---

## GUI usage

Launch without any subcommand to open the GUI:

```sh
exocortex
```

| Action | How |
|--------|-----|
| Quick jot (any context) | `Ctrl+Shift+J` (global) |
| Quick jot (in-app) | `Ctrl+Shift+J` or the sidebar button |
| Submit quick jot | `Enter` |
| Dismiss quick jot | `Esc` |
| Edit a jot | Double-click the jot text |
| Switch to Log view | Sidebar → 📋 Log |
| Switch to Jots view | Sidebar → 📝 Jots |

---

## Data format

Jots are stored as a newline-delimited JSON (JSONL) action log:

```
~/.local/share/exocortex/log.jsonl   (Linux)
~/Library/Application Support/exocortex/log.jsonl   (macOS)
%APPDATA%\exocortex\log.jsonl   (Windows)
```

Each line is a JSON object, for example:

```json
{"type":"AddJot","timestamp":"2024-06-01T10:00:00Z","id":"...","text":"remember to call dentist"}
{"type":"EditJot","timestamp":"2024-06-01T10:05:00Z","id":"...","new_text":"call dentist before Friday"}
```

The current state of all jots is always a pure function of replaying this log from the beginning.
