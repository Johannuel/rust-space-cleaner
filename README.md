# 🧹 rust-space-cleaner

> **The cache hunter.** A safe terminal app that shows every gigabyte of junk hiding on your disk and lets you claim it back — Windows, Linux & macOS.

[![CI](https://img.shields.io/github/actions/workflow/status/Johannuel/rust-space-cleaner/ci.yml?branch=main&logo=github&style=flat-square)](https://github.com/Johannuel/rust-space-cleaner/actions)
[![rustc](https://img.shields.io/badge/rust-1.97%2B-orange?logo=rust&style=flat-square)](https://www.rust-lang.org)
[![crates.io](https://img.shields.io/crates/v/rust-space-cleaner?style=flat-square)](https://crates.io/crates/rust-space-cleaner)
[![AUR](https://img.shields.io/aur/version/rust-space-cleaner?style=flat-square)](https://aur.archlinux.org/packages/rust-space-cleaner)
[![MIT](https://img.shields.io/badge/license-MIT-blue.svg?style=flat-square)](LICENSE)

![rust-space-cleaner preview](assets/social-card.png)

## Why?

SSDs fill up silently. Caches, package registries, build artifacts and docker leftovers pile up until your disk screams — and finding what to delete is a guessing game. `rust-space-cleaner` scans **24 categories of junk** across your system, tells you exactly how much each one weighs, and lets you reclaim the space **safely**: only whitelisted cache folders, always confirmed.

> [!IMPORTANT]
> The default mode is **dry-run**: nothing is deleted unless you explicitly confirm. The tool only cleans "cache"-type folders on its whitelist — it never touches your files or `$HOME`. Every deletion is logged to `clean.log`.

## Install

```bash
# from source (anywhere, with Rust)
cargo install rust-space-cleaner

# Arch Linux (AUR)
yay -S rust-space-cleaner

# or clone & run
git clone https://github.com/Johannuel/rust-space-cleaner
cd rust-space-cleaner && cargo run --release
```

> Binaries for Windows, Linux and macOS ship with each [GitHub release](https://github.com/Johannuel/rust-space-cleaner/releases).

## What it hunts (24 sources)

| Category | Sources |
|---|---|
| **Dev** | cargo registry, cargo `target/` folders, go-build, .NET/NuGet, Maven, Gradle, conda pkgs |
| **Games** | Steam shader cache, Proton shader cache, Lutris cache |
| **Web** | npm, pnpm, yarn, Firefox, Chromium, Electron caches |
| **System** | user cache (`~/.cache/*`), Docker dangling images, systemd journal, flatpak, trash |
| **Tools** | rustup/tmp, pip, dotnet |

Every source is a row in the declarative [registry](src/registry.rs) — it costs one
line to add a new one, and each entry is covered by tests.

## The TUI

- **Inventory** view: sources sorted by size, each with a **status**
  (`scan | ok | error | not found`), live **per-source progress bars**, a
  **dup badge** when a duplicate exists, and its size at a glance.
- **Risk metadata** (low / medium / high) on every source so you never delete
  something by mistake.
- **Dry-run by default**, `d` to toggle, `?` for the full keymap.
- **Multi-select batch, filters and history** land in v0.2 (on the roadmap).

### Keymap

| Key | Action |
|-----|--------|
| `↑` / `↓`, `j` / `k` | navigate |
| `s` | prepare cleanup for the selected row |
| `space` | mark for batch cleanup (upcoming) |
| `y` / `n` | confirm / cancel in the modal |
| `d` | toggle dry-run |
| `r` | rescan |
| `q` / `Esc` | quit |

When dry-run is **off** and you confirm a cleanup, the removal is written to
`~/.local/share/rust-space-cleaner/clean.log` (timestamp, size and path).

## Safety

- **Whitelist-driven**: `clean.rs`’s `is_safe_to_clean` only allows exact
  whitelist entries or direct `~/.cache` sub-folders. `$HOME`, `/`, whole
  containers and fake prefixes (`~/.cargo_evil`) are always rejected — and tested.
- **Dry-run by default** and per-source confirmation.
- **Badges of risk** so high-effort deletes never happen by accident.

## Tested, everywhere

```bash
cargo test                 # unit + integration + binary tests (41, green)
cargo clippy --all-targets -- -D warnings
cargo fmt --check
```

CI (`.github/workflows/ci.yml`) runs fmt + clippy + tests on every push and PR;
a release workflow publishes binaries for all three OSes on new tags.

## Structure

```
src/
  main.rs    -> ratatui startup + real provider
  model.rs   -> CleanSource, ScanStatus, Category, Risk, human_size
  registry.rs-> declarative source registry (the whitelist)
  scan.rs    -> source detection + size measuring (threads, progress)
  clean.rs   -> is_safe_to_clean (whitelist) + batch cleanup
  state.rs   -> persistent history
  ui.rs      -> inventory, filters, detail, history, help
tools/
  gen_fixtures.py   # deterministic fake tree for integration tests
```

## Roadmap

- [x] 24-source declarative registry
- [x] Multi-select batch cleanup
- [x] History (`state.json`)
- [x] Multi-platform: Windows, Linux, macOS
- [x] CI + release binaries
- [ ] AUR package (upstream)
- [ ] Config file (extra per-OS roots)

## Contributing

PRs welcome. Ideas → [open an issue](https://github.com/Johannuel/rust-space-cleaner/issues).
`REGISTRY.md` is your friend: you can add the source you care about in a few lines.

---

<p align="center">
  <sub>Made with ♥ and ratatui. Star it if it saved your disk 🚀</sub>
</p>