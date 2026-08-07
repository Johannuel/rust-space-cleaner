# Marketing — rust-space-cleaner (v0.1.0)

Foco de publicación: **Reddit → X → Bluesky** (Hashnode no dio views; Dev.to
y Lobsters quedan como opcionales por tiempo). Todo el copy está listo abajo.
Gancho común: **"cache hunter: 24 fuentes de basura, borrado seguro, dry-run
por defecto"**.

## Plan de publicación (día 1)

1. **Reddit r/opencode** (mañana temprano) — el sub que más visitas dio, ángulo "built with opencode". Post listo abajo.
2. **Reddit r/rust** (1-2h después) — post de más valor técnico, con gancho opencode.
3. **X** 1-2h después (misma mañana).
4. **Bluesky** el mismo día por la tarde.
5. **r/commandline** 2-3 días después (reposteo con el mismo texto, no spam).
6. Contestar TODOS los comments las primeras 48h (es lo que más upvotes da).
7. Si r/rust ignora → re-postear en r/rust con ángulo distinto tras 2 semanas.

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

## Reddit — r/rust (PRIORIDAD 1, día 1 mañana)

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

This is my first time writing Rust — I built it with opencode (an open-source
AI coding agent) as my pair programmer. Feedback on the safety model and the
registry design very welcome 🙏
```

---

## Reddit — r/opencode (PRIORIDAD 1, día 1 mañana — el sub que más visitas da)

```markdown
[P] I built a Rust TUI cache cleaner with opencode — and shipped it to crates.io in a week

I built `rust-space-cleaner` (a cache hunter TUI in Rust + ratatui) mostly
with opencode as my pair programmer. It scans 24 types of disk junk (cargo
targets, Steam shaders, npm, docker leftovers...), shows sizes sorted by
weight, and only deletes whitelisted cache folders after your confirm —
dry-run by default.

What working with opencode taught me:
- The registry design (src/registry.rs) is 24 rows + one test each — the
  agent kept pushing me toward the declarative version, and it paid off
- Safety was reviewable: `is_safe_to_clean` + 41 tests for the dangerous part
- CI + release binaries for Win/Linux/macOS were generated, not hand-written

Repo: https://github.com/Johannuel/rust-space-cleaner
crates.io: https://crates.io/crates/rust-space-cleaner
```

---

## X / Twitter (≤ 280 chars) — día 1, 1-2h tras el post de Reddit

```text
🧹 I built a cache hunter in Rust: scans 24 types of disk junk safely — whitelist-only, dry-run by default, ratatui TUI. Win/Linux/macOS.

cargo install rust-space-cleaner

https://github.com/Johannuel/rust-space-cleaner
#rustlang #ratatui
```

---

## Bluesky (≤ 300 chars; día 1 por la tarde)

```text
🧹 rust-space-cleaner: safe "cache hunter" TUI in Rust + ratatui. Finds 24 types of disk junk, deletes only whitelisted caches after your confirm (dry-run default). Win/Linux/macOS.

🚀 cargo install rust-space-cleaner

github.com/Johannuel/rust-space-cleaner #rustlang #ratatui
```

---

## Reddit — r/commandline (2-3 días después del de r/rust)

```markdown
[T] rust-space-cleaner — a cache-cleaner TUI for the terminal, now on crates.io

A simple cache hunter in Rust: scans 24 known junk spots (package manager
caches, cargo targets, shaders, docker leftovers) and only frees what's
whitelisted + confirmed. dry-run by default.

cargo install rust-space-cleaner

https://github.com/Johannuel/rust-space-cleaner
```

---

## Opcionales (solo si sobra tiempo — bajo prioridad)

### Dev.to

```markdown
# I made a terminal app that hunts down disk junk ✅ safe by default

A cache hunter for Linux/macOS/Windows, in Rust + ratatui. Scans 24 categories,
sizes each one, dry-run first; deletes only whitelisted folders, logs every
removal. Declarative registry: each source is one row in a table, so adding
one is trivial. 41 tests, CI matrix with binaries.

Try it: `cargo install rust-space-cleaner` → run `rust-space-cleaner`.
Repo: https://github.com/Johannuel/rust-space-cleaner
```

### Lobsters

```markdown
[T] rust-space-cleaner — cache cleaner TUI in Rust + ratatui

Scans 24 categories of disk junk and reclaims space safely: whitelist-only
deletion, dry-run by default. Declarative registry (one row + test), portable
paths with cfg(target_os), threaded scanning, 41 tests green.

https://github.com/Johannuel/rust-space-cleaner
```

### Hacker News («Show HN» — solo tras 1-2 reacciones previas)

```markdown
Show HN: Cache hunter — Rust TUI that finds and safely cleans 24 sources of
disk junk

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
7. [ ] **r/rust** (día 1, mañana) + **X** (misma mañana) + **Bluesky** (tarde)
8. [ ] **r/commandline** (2-3 días después)
9. [ ] Star-círculo: pedir a un amigo → con 2-3, solicitar reacciones
10. [ ] Opcional: Dev.to, Lobsters, Show HN (solo si sobra tiempo)

*El demo se graba desde una terminal real con asciinema → agg → demo.gif.*