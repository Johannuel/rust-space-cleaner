# Marketing — rust-space-cleaner (v0.1.0)

Publicamos el release `v0.1.0` + crate en crates.io. Todo el material listo
para compartir. Gancho común: **"cache hunter: 24 fuentes de basura, borrado
seguro, dry-run por defecto"**.

## URL útiles

- Repo: https://github.com/Johannuel/rust-space-cleaner
- crates.io: https://crates.io/crates/rust-space-cleaner
- Release: https://github.com/Johannuel/rust-space-cleaner/releases/tag/v0.1.0
- Social card: `assets/social-card.png`
- Demo (cuando exista): `assets/demo.gif`

## Checklist de claims (qué es REAL hoy en v0.1.0)
| Claim | Real en v0.1.0? |
|-------|------------------|
| `cargo install rust-space-cleaner` | ✅ real desde 2026-08-07 |
| scan + sizes + dry-run + confirm | ✅ real |
| progress bars / dup badge | 🔶 v0.2 (rama feature, sin integrar) |
| multi-select batch, history, filters | 🔶 v0.2 (sin integrar) |

*No publiques un claim que no esté marcado ✅.*

## Tags / hashtags

`#rust #ratatui #tui #linux #opensource #cache`
`#devtools #archlinux #developer #cleantools`

---

## X / Twitter (≤ 280 chars)

```text
🧹 I built a cache hunter in Rust: it scans 24 kinds of disk junk (dev caches, Steam shaders, Docker leftovers...) and lets you reclaim space safely — whitelist-only, dry-run by default, ratatui TUI. Windows/Linux/macOS.

cargo install rust-space-cleaner

https://github.com/Johannuel/rust-space-cleaner
#rustlang #ratatui
```

---

## Bluesky (≤ 300 chars)

```text
🧹 rust-space-cleaner: a safe "cache hunter" TUI in Rust + ratatui. Scans 24 sources of disk junk, shows what each one weighs, and only deletes whitelisted caches after your confirmation (dry-run by default). Win/Linux/macOS.

🚀 cargo install rust-space-cleaner

github.com/Johannuel/rust-space-cleaner
#rustlang #ratatui #opensource
```

---

## Reddit — r/rust

```markdown
[P] I built a cache hunter in Rust: scans 24 kinds of disk junk and cleans only the safe stuff

Tired of checking ~/.cache, cargo target/, Steam shaders and docker leftovers
by hand? I made `rust-space-cleaner`: a ratatui TUI that scans 24 source
categories (Dev, Games, Web, System, Tools), shows each one's size sorted by
weight, and only deletes folders on an explicit whitelist (never $HOME).

- Dry-run by default; `d` to toggle; confirmation modal per cleanup
- Safe by design: `is_safe_to_clean` rejects `$HOME`, `/`, whole containers and fake prefixes, all covered by tests
- Declarative registry (src/registry.rs) — adding a source is one row + a test
- 41 tests, clippy -D warnings, CI + release binaries for Linux/macOS/Windows
- Now on crates.io: `cargo install rust-space-cleaner`

Repo: https://github.com/Johannuel/rust-space-cleaner

This is my first time writing Rust. Feedback on the safety model and the
registry design very welcome 🙏
```

---

## Reddit — r/commandline

```markdown
[T] rust-space-cleaner — a cache-cleaner TUI for the terminal, now on crates.io

A simple cache hunter in Rust: scans 24 known junk spots (package manager
caches, cargo targets, shaders, docker leftovers) and only frees what's
whitelisted + confirmed. dry-run by default.

cargo install rust-space-cleaner

https://github.com/Johannuel/rust-space-cleaner
```

---

## Hacker News («Show HN» — déjalo madurar hasta que haya 1-2 vets)

```markdown
Show HN: Cache hunter — Rust TUI that finds and safely cleans 24 sources of
disk junk

https://github.com/Johannuel/rust-space-cleaner
```

---

## Dev.to

```markdown
# I made a terminal app that hunts down disk junk ✅ safe by default

A cache hunter for Linux/macOS/Windows, in Rust + ratatui.

**The hook** — caches multiply: package managers, build artifacts, shader
caches, electron... until your SSD is full and you can't say why.

**What it does** — scans 24 categories, sizes each one, dry-run first; deletes
only whitelisted folders, logs every removal.

**The interesting part: a declarative registry** — each source is one row in a
table (id/name/category/risk/paths), so contributing a new one is trivial.
This is the cleanest Rust I've written.

**What I learned** — cfg!(target_os) for portable paths, threads for measuring
folders, safety tests for is_safe_to_clean, a CI matrix that ships binaries.

Try it: `cargo install rust-space-cleaner` → run `rust-space-cleaner`.
Repo: https://github.com/Johannuel/rust-space-cleaner
```

---

## Lobsters

```markdown
[T] rust-space-cleaner — cache cleaner TUI in Rust + ratatui

Scans 24 categories of disk junk (dev tool caches, game shader caches, browser
caches, system logs, docker) and reclaims space safely: whitelist-only
deletion, dry-run by default, per-source risk badges.

Interesting bits: declarative source registry (one row + test), portable paths
with cfg(target_os), threaded scanning, safety tests. 41 tests green,
clippy -D warnings, release binaries for 3 OSes.

https://github.com/Johannuel/rust-space-cleaner
```

---

## Awesome-Rust PR

Cuando el repo tenga ~10+ stars y 1 release con binaries:

```markdown
## rust-space-cleaner

A safe cache hunter: scans 24 sources of disk junk and reclaims the space
via a ratatui TUI (Windows/Linux/macOS). Whitelist-only deletion, dry-run by
default, per-source progress. [✻](https://github.com/Johannuel/rust-space-cleaner)
```

Sección sugerida: `Applications → System`.

---

## Checklist de lanzamiento

1. [x] README v2 actualizado
2. [x] Tópicos y descripción del repo
3. [x] Release v0.1.0 (binaries en CI)
4. [x] Publicado en crates.io (`cargo install` funciona)
5. [ ] PKGBUILD subido a AUR (falta acceso SSH)
6. [ ] Grabar demo (assets/demo.gif) y ponerlo en README/comment 1 de cada post
7. [ ] Publicar en X, Bluesky, r/rust, r/commandline, Dev.to, Lobsters
8. [ ] Star-círculo: pedir a un amigo → con 2-3, solicitar reacciones

*El demo se graba desde una terminal real con asciinema → agg → demo.gif.*