# Design v2: rust-space-cleaner — "Cazador de basura total"

Fecha: 2026-08-06
Estado: propuesto (pendiente de revisión)
Espec v1: `2026-08-06-rust-space-cleaner-design.md` (base funcional, 29 tests verdes)

## Problema

La v1 cumple, pero es genérica: 7 fuentes, lista plana, cero filtros. Para
destacar entre tantos "cache cleaners" necesita (a) cubrir casi toda la basura
real de un disco Linux y (b) una TUI que lo haga vistosa y cómoda de usar.

## Objetivo v2

Convertir `rust-space-cleaner` en un **cazador de basura total con TUI
espectacular**:

1. **Registry declarativo** de fuentes: añadir una fuente = una fila + su test.
2. **~24 fuentes** reales y seguras (sección Fuentes).
3. **TUI con inventario, filtros, sort, multi-selección, detalle y progreso**.
4. **Historia persistente**: suma de disco recuperado, últimas limpiezas.
5. **Showcase + outreach**: release v0.1.0, AUR, GIF, README killer, posts.
6. Todo con `cargo test`, `cargo clippy --all-targets -- -D warnings` y
   `cargo fmt --check` verdes; arquitectura portable (Windows/Linux/macOS).

## Arquitectura v2

### Registry (`src/registry.rs`) — núcleo nuevo

Tabla estática (sin trait-objects ni `Box<dyn>`) con metadata por fuente:

```rust
pub enum Category { Dev, Games, Web, System, Tools }
pub enum Risk { Low, Medium, High }   // Low=regenerable, High=no se borra a la ligera

pub struct SourceDef {
    pub id: &'static str,
    pub name: &'static str,
    pub category: Category,
    pub risk: Risk,
    pub paths: Vec<PathCfg>,          // rutas estáticas con cfg por SO
    pub special: Option<fn(&Home, &mut Vec<CleanSource>)>, // hook solo para lógica rara
}
```

- El registry ES la lista blanca: nada fuera de él se borra jamás.
- `candidate_sources()` junta: rutas estáticas (existen o no → status
  NotFound) + hooks especiales (docker, journal, targets de cargo).
- `CleanSource` toma `category` y `risk` (se copian del `SourceDef`).

### `src/scan.rs`

- Se mantienen medición multi-hilo y `is_cargo_target`.
- Cada fuente reporta progreso via `mpsc` (bytes medidos / estimado) para la
  barra de progreso del TUI.

### `src/clean.rs`

- `clean_batch(paths, whitelist)`: valida cada ruta contra la whitelist; una
  ruta no permitida marca `error` en esa fuente y el lote continúa.
- Fuentes `Risk::High` requieren confirmación explícita extra en el modal
  (misma tecla `y`, pero la fila se dibuja con `!!`).

### `src/state.rs` (nuevo)

- Persiste en `~/.config/rust-space-cleaner/state.json` (crea el dir).
- Esquema: `{ total_reclaimed: u64, cleanups: [{ ts, name, size }] }` (max 10).

### `src/ui.rs` (refactor mayor)

- Vista inventario (tab principal).
- Vista historial (tab `h`).
- Modal `?` de ayuda con todos los atajos.

```
src/
  main.rs    -> arranca ratatui + provider (registry + estado)
  model.rs   -> CleanSource + Category + Risk (display/color)
  registry.rs-> SourceDef + tabla de fuentes
  scan.rs    -> medir + targets + docker + journal + progreso mpsc
  clean.rs   -> limpieza segura por lote + whitelist
  state.rs   -> persistencia del historial
  ui/        -> app (estado), lista, detalle, help, tab
```

## Fuentes (objetivo ~24, todas dentro de la whitelist)

| Categoría | Fuentes |
|-----------|---------|
| Dev | cargo (global), targets de cargo, dotnet, nuget, maven, gradle, go-build, conda pkgs |
| Games | steam shader cache, proton shader cache, lutris, blast radius |
| Web | firefox cache, chromium cache, electron cache, npm/yarn/pnpm |
| System | user cache (`~/.cache/*`), journal, docker dangling, flatpak, trash, plasma |
| Tools | rustup/tmp, pip, mise/asdf |

Cada fuente con su `SourceDef` y un test funcional (detecta/medir bajo
fixtures, o `NotFound` si no existe). Las que requieran lógica se marcan con
hook `special`, sin inflar el código.

## UI (detalle)

- Header: título + stats (n fuentes, total recuperable, riesgo agregado).
- Fila: `[cat] [riesgo] detalle nombre .... barra relativa [size]`.
- Sort: `s` cicla tamaño/name/risk (default: tamaño).
- Filter: `t` filtra por categoría (Dev/Games/Web/System/Tools).
- `enter`: panel detalle (top 5 subdirectorios de la fuente).
- `space`: seleccionar; `m`: limpiar selección; `n`/`Esc`: cancelar.
- `r`: reescanear; `h`: historial; `?`: ayuda; `q`/`Esc`: salir.
- Dry-run sigue **ON por defecto**. `d` toggle con indicador en header.
- Progreso: cuando `Scanning`, fila con barra + %.

## Manejo de errores

- `Result` por fuente; EACCES → estado de la fuente, nunca crashea.
- Estado corrupto o ausente → se ignora y se recrea.
- Sin sudo: rutas que requirieran root (journal) se reportan `error` con hint;
  la app sigue viva.

## Testing

- Unit: `registry` (ids únicos, metadata válida), `state` (serializar/parsear),
  `clean` (batch + riesgo), `scan` (measure, targets).
- Fixtures: ampliar `gen_fixtures.py` a un árbol de games/web.
- Cada fuente: test que detecta/mide con fixtures, o `NotFound` sin red.
- CI existente (`cargo test` / `clippy` / `fmt`) sigue pasando.

## Marketing / Showcase (tras el código listo)

- `cargo install --path .` funcionando.
- Release `v0.1.0` con binaries por SO (CI matrix) + AUR `PKGBUILD`.
- `README.md`: GIF demo (asciinema/agg), sección "What it cleans", tabla de
  fuentes con categorías, enlaces de instalación, "Run in your terminal".
- `docs/marketing.md`: copy listo para r/unixporn, r/rust, dev.to, lobste.rs,
  y PR a Awesome-Rust.
- `assets/social-card.svg` actualizada con el nuevo tagline y fuentes top.

## Fuera de alcance (YAGNI v2)

- Sudo / gestión automática de permisos (journal/root queda solo como hint).
- Interfaz web, daemon o sync.
- Búsqueda de outliers tipo ncdu (se delega en el "detalle" de cada fuente).
- Fuentes personalizables por el usuario con configuración (el registry fijo
  cubre el 90% del valor).

## Coordinación (4 sesiones)

- **S3** → motor de fuentes (registry + fuente por fuente + tests) y ampliar
  `gen_fixtures.py`. OJO: no tocar el punto de progreso (mpsc) de `scan.rs`,
  lo hace S2 con el TUI.
- **S2** → TUI completa: lista, historial, detalle, help, atajos, barra de
  progreso mpsc.
- **S1** → clean batch + whitelist + state + tests + confirmación de riesgo.
- **S4 (integrador)** → model.rs/registry core (SourceDef + Category + Risk),
  integración, readme/marketing/release/AUR/benchmarks y merge final.

*Esta sección se refleja 1:1 en `AGENTS.md`.*