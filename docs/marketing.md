# Marketing — rust-space-cleaner (release v0.1.0)

Publicamos el release `v0.1.0` (binaries por SO). Aquí está el material listo
para compartir. Todos los posts usan el mismo gancho: **"cache hunter: 24
fuentes de basura, borrado seguro, dry-run por defecto"**.

## URL útiles

- Repo: https://github.com/Johannuel/rust-space-cleaner
- Release: https://github.com/Johannuel/rust-space-cleaner/releases/tag/v0.1.0
- Social card: `assets/social-card.png`
- Demo (cuando exista): `assets/demo.gif`

## Tags / hashtags

`#rust #ratatui #tui #archlinux #linux`
`#cleaning #cache #devtools #opensource`

---

## Reddit — r/rust

```markdown
[P] I built a cache hunter in Rust that scans 24 kinds of disk junk and cleans
only the safe stuff.

Tired of checking ~/.cache, cargo target/, Steam shaders and docker leftovers
by hand? I made `rust-space-cleaner`: a ratatui TUI that scans 24 source
categories (Dev, Games, Web, System, Tools) with per-source progress + dup
badges, and only deletes folders on an explicit whitelist (never $HOME).

- Dry-run by default; a `d` to toggle; confirmation modal per cleanup
- Declarative registry (src/registry.rs) — adding a source is one row + a test
- 41 tests, clippy -D warnings, CI+release binaries for Linux/macOS/Windows
- cargo install rust-space-cleaner

Repo: https://github.com/Johannuel/rust-space-cleaner

I'm a beginner; feedback on safety & registry design very welcome 🙏
```

---

## Reddit — r/unixporn

```markdown
[Tool] When your SSD whispers, hunt the cache.

ratatui TUI that lists the 24 biggest junk offenders on your disk with a
risk badge per row, lets you multi-select and clean them safely (whitelist +
confirm, dry-run default). Progress bars while scanning.

fcapt set colours: gruped, no-nonsense. Yes it's a screenshot of a terminal
app.

https://github.com/Johannuel/rust-space-cleaner
```

---

## Dev.to

```markdown
# I made a terminal app that hunts down your disk junk
A cache hunter for Linux/macOS/Windows.

**(intro párrafo)** Caches multiply: package managers, build artifacts,
shader caches, electron... until your SSD is full and you can't say why.

**What it does** — scans 24 categories, sizes each, dry-run first, delete
only whitelisted, log every removal.

**The interesting part: a declarative registry** — each source is a row in a
table (id/name/category/risk/paths), so contributing a new one is trivial.
This is the cleanest Rust I've written.

**Complexities I learned** — cfg target_os, rayon-free threads, safety tests
for is_safe_to_clean, CI matrix.

Wrap: repo + cargo install one-liner. Ask for feedback.
```

---

## Lobsters

```markdown
[T] rust-space-cleaner — a safe cache cleaner TUI in Rust + ratatui

Scans 24 categories of disk junk (development tool caches, games shaders,
web browser/crypto caches, system logs and docker) and lets you reclaim space
safely: whitelist-only deletion, dry-run by default, per-source risk badges.

Interesting bits: declarative source registry (add one = one row + test),
portable paths with cfg target_os, thread pool scanning with progress stream.
41 tests green.

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
4. [x] PKGBUILD en repo (subir a AUR)
5. [ ] Grabar demo (assets/demo.gif) y ponerlo en README/comment 1 de cada post
6. [ ] Publicar en r/rust, r/unixporn, Dev.to, Lobste
7. [ ] Pedir a un amigo que estre depurar → star del otro → con 2, pedir 10? readme

*El demo se graba desde una terminal real con asciinema → agg → demo.gif.*