# CONTEXT.md — contexto para agentes de opencode

> Léeme primero si acabas de abrir una ventana de opencode en este repo.
> Es el resumen ejecutivo; los detalles están en `AGENTS.md` (coordinación),
> `docs/superpowers/specs/2026-08-06-rust-space-cleaner-v2-design.md` (diseño)
> y `docs/superpowers/plans/2026-08-06-rust-space-cleaner-v2.md` (plan).

## ¿Qué es este proyecto?

**rust-space-cleaner** — un "cache hunter": TUI en Rust + ratatui que escanea
24 fuentes de basura de disco (cachés de dev, juegos, web, sistema y tools),
muestra cuánto pesa cada una y permite limpiarlas SOLO si están en la whitelist,
con dry-run por defecto y confirmación. Multiplataforma (Windows/Linux/macOS).

GitHub: `Johannuel/rust-space-cleaner` (público, MIT). Release `v0.1.0` hecho.

## Estado real (2026-08-06/07)

- ✅ **Core v2 en `main`**: `src/registry.rs` (SourceDef declarativo, 24
  fuentes, ids únicos), `model.rs` con `Category`/`Risk` y
  `CleanSource::with_meta`, `scan::candidate_sources` manejado por el registry.
- ✅ **41 tests verdes**, clippy `-D warnings`, fmt limpio (CI pasa).
- ✅ Marketing: README v2, tópicos GitHub, release `v0.1.0` (binaries
  Linux/macOS/Windows en CI), `PKGBUILD` (AUR), `docs/marketing.md` con copy.
- 🔶 Ramas pendientes de integrar a `main`:
  - `feat/progress-tui` (S2): B4 progreso mpsc + B5 badge DUP + T6/T8 tabs/help/detail.
  - `feat/scan-portable` (S3): B2 multiplataforma real + fixtures games/web.
  - `feat/bench` (S4b): benchs Criterion (opcional).
- ⏳ Pendiente: `clean_batch` + `state.rs` (historial persistente), filtros/sort
  en TUI, demo GIF, posts Reddit/Dev.to, AUR real (subir PKGBUILD).

## Arquitectura

```
src/
  main.rs    -> arranque ratatui + RealProvider (scan + clean)
  model.rs   -> CleanSource, ScanStatus, Category, Risk, human_size
  registry.rs-> SourceDef + tabla de 24 fuentes (la whitelist declarativa)
  scan.rs    -> medir tamaño (threads), targets cargo, docker, journal, progreso
  clean.rs   -> is_safe_to_clean (whitelist, TDD)
  ui.rs      -> TUI ratatui (lista, spinner, modal, dry-run, clean.log)
tests/
  clean_safety.rs   # whitelist
  scan_extra.rs     # fixtures simulados
tools/
  gen_fixtures.py   # árbol de fixtures determinista (gitignored)
```

## Comandos de referencia

```bash
cargo run                       # TUI (necesita TTY real)
cargo test                      # 41 tests
cargo clippy --all-targets -- -D warnings
cargo fmt --check
python3 tools/gen_fixtures.py --generate   # regenerar fixtures
```

## Cómo coordinarse

1. `git fetch origin && git rebase origin/main` al arrancar.
2. Trabaja en tu worktree/rama (ver AGENTS.md).
3. Al terminar: commit + push de TU rama, y registra en AGENTS.md (bitácora)
   + avisa a S4 (integrador) para el merge a `main`.
4. Regla de oro: **nada se borra fuera de la whitelist del registry**. Si
   añades una fuente, es una fila en `registry.rs` + su test.

## Estado de tmux (ventanas de opencode)

- Sesión tmux `0` → ventanas: `0=s1` (repo main), `1=RUST cleaner`,
  `5=s2` (/tmp/rsc-s2), `6=s3` (/tmp/rsc-s3), `7=s4b` (/tmp/rsc-s4-bench).
  Navega: `Ctrl-b` + número de ventana, o `Ctrl-b n/p`.
- Cada ventana corre `opencode` en su worktree con su AGENTS.md.

## Próximos pasos sugeridos (orden)

1. **S4 (integrador)**: integrar `feat/progress-tui` y `feat/scan-portable` a
   main (merge/PR), resolver conflictos en `ui.rs`.
2. **S1**: `clean_batch` + `state.rs` (historial) + tests.
3. **S2**: filtros (`t` categoría), sort (`s`), multi-selección (`space`/`m`).
4. **S3**: más fuentes si quedan (ver spec), gen_fixtures ampliado.
5. **Marketing**: demo GIF (grabar en TTY real), posts Reddit/Dev.to,
   subir PKGBUILD a AUR, PR Awesome-Rust cuando haya ~10 stars.