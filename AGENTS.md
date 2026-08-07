# AGENTS.md — rust-space-cleaner (DEFINITIVO)

Única fuente de verdad para coordinar las 4 sesiones de opencode (tmux).
**Léelo completo en cada arranque.** Si cambias estado, ACTUALIZA la sección
"Registro" y commitea.

## Cómo arrancar (cada sesión)

```bash
git fetch origin && git rebase origin/main   # estar al día
cat AGENTS.md && cat docs/superpowers/specs/2026-08-06-rust-space-cleaner-v2-design.md
```

Checklist antes de tocar nada:
1. `git status` limpio en tu rama.
2. Trabaja SIEMPRE en tu worktree/rama (abajo).
3. Nada de push a `main` sin avisar a S4 (integrador).

## Las 4 sesiones (definitivo)

| Sesión | Especialidad | Worktree | Rama | Dueño de |
|--------|--------------|----------|------|----------|
| **S1** | Safety & clean | `/tmp/rsc-s1` (a crear) | `feat/clean-batch` | `clean.rs`, `state.rs`, tests safety |
| **S2** | TUI ratatui | `/tmp/rsc-s2` | `feat/progress-tui` | `ui.rs`, progreso mpsc, tabs/detalle/help |
| **S3** | Scan + fuentes | `/tmp/rsc-s3` | `feat/scan-portable` | `scan.rs`, `registry.rs` fuentes, fixtures, CI |
| **S4** | Integrador (yo) | repo principal | `main` | `model.rs`, `registry.rs` core, merge, release, marketing |

## Reparto v2 (vigente)

| Tarea | Sesión | Estado |
|-------|--------|--------|
| B1 traducción a inglés | S4 | ✅ hecho |
| B2 scan multiplataforma real | S3 | 🔶 rama `feat/scan-portable` |
| B3 tests portable (tempdir) | S1 | ✅ hecho en main |
| B4 progreso mpsc | S2 | 🔶 rama `feat/progress-tui` |
| B5 badge DUP | S2 | 🔶 rama `feat/progress-tui` |
| registry.rs core (24 fuentes) | S4 | ✅ en main (41 tests) |
| T6+T8 tabs/help/detail | S2 | 🔶 rama `feat/progress-tui` |
| fixtures games/web | S3 | 🔶 rama `feat/scan-portable` |
| clean_batch + state.rs | S1 | ⏳ pendiente |
| TUI filtros/sort/multi | S2 | ⏳ pendiente |
| marketing/release/README | S4 | ✅ v0.1.0, README v2, PKGBUILD |
| benchs Criterion | S4b | 🔶 `feat/bench` (opcional) |

## Reglas de convivencia (NO NEGOCIABLES)

1. **Un worktree por sesión**, rama `feat/*` propia. Nunca dos sesiones sobre
   el mismo directorio/rama.
2. **No toques el código del otro**: si toca `model.rs`/`main.rs`/`registry.rs`,
   coordina con S4.
3. **Nunca commitees fixtures ni target**: se regeneran con
   `tools/gen_fixtures.py --generate`.
4. **Sin push a `main`**: S4 integra con merge o PR. Avisar en "Registro".
5. Verificación obligatoria antes de "listo":
   `cargo test` + `cargo clippy --all-targets -- -D warnings` + `cargo fmt --check`.
6. Todo en **inglés** (código, commits, strings, README).
7. Comunicación entre sesiones = `AGENTS.md` (editar + commit + push) y el
   `git fetch` al arrancar.

## Registro de coordinación (bitácora)

- `main @ 81dc763` — core v2 (registry 24 fuentes, Category/Risk, 41 tests
  verdes), release v0.1.0 con binaries (CI), README v2, PKGBUILD, spec v2,
  plan v2 en `docs/superpowers/plans/`.
- S2 `feat/progress-tui @ 683b495`: B4+B5+T6+T8 hechos. **Pendiente: rebase
  sobre main e integración (PR o merge S4).**
- S3 `feat/scan-portable @ 3e069bb`: B2 + fixtures games/web. **Pendiente:
  rebase sobre main e integración.**
- S4b `feat/bench @ 5c121e6`: benchs WIP (opcional).

*(Actualiza esta bitácora cada vez que termines algo.)*