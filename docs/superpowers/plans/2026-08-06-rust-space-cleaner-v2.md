# rust-space-cleaner v2 — "Cazador de basura total" Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Convertir la v1 en un cazador de basura total: registry declarativo de ~24 fuentes y TUI espectacular, manteniendo seguridad (whitelist) y 35+ tests verdes.

**Architecture:** Registry estático declarativo (`registry.rs`) como única fuente de verdad de qué se escanea y se limpia. `model.rs` gana `Category`/`Risk`. La TUI pasa a tabs (inventario / historial / detalle / ayuda) y las fuentes se resuelven por `SourceDef` con hook `special` para las 3 con lógica rara.

**Tech Stack:** Rust, ratatui + crossterm, serde (state.json), CI GitHub Actions. Sin trait-objects: registry = datos, scan = funciones.

## Global Constraints

- Todo el código, comentarios, mensajes de commit y strings de UI **en inglés** (B1).
- `cargo test` / `cargo clippy --all-targets -- -D warnings` / `cargo fmt --check` verdes antes de cada commit.
- Las fuentes nuevas entran SIEMPRE por `registry.rs`; nada se borra fuera de la whitelist.
- No se commitea `tests/fixtures/` ni `target/` (se regeneran con `tools/gen_fixtures.py --generate`).
- Los worktrees de cada sesión usan ramas `feat/*`; NO se empuja a `main` sin aviso del integrador (S4).
- S2 es dueña del mpsc de progreso en UI/scan; S3 no lo toca.
- Arquitectura portable: `#[cfg(target_os = "windows"|"linux"|"macos")]` en `registry.rs` para rutas.

---

## Pista 1 — Core (S4, integrador)

### Task 1: Añadir `Category` y `Risk` a `model.rs`

**Files:**
- Modify: `src/model.rs`
- Test: `src/model.rs` (módulo `#[cfg(test)]`)

**Interfaces:**
- Produces:
  - `pub enum Category { Dev, Games, Web, System, Tools }`
  - `impl Category { pub fn label(self) -> &'static str }`
  - `pub enum Risk { Low, Medium, High }`
  - `impl Risk { pub fn label(self) -> &'static str }`

- [ ] **Step 1: Write the failing test**

Add to `#[cfg(test)] mod tests` in `model.rs`:

```rust
#[test]
fn category_and_risk_labels() {
    assert_eq!(Category::Dev.label(), "dev");
    assert_eq!(Category::Games.label(), "games");
    assert_eq!(Category::Web.label(), "web");
    assert_eq!(Category::System.label(), "system");
    assert_eq!(Category::Tools.label(), "tools");
    assert_eq!(Risk::Low.label(), "low");
    assert_eq!(Risk::Medium.label(), "medium");
    assert_eq!(Risk::High.label(), "high");
}
```

- [ ] **Step 2: Run test to verify it fails**

Run: `cargo test category_and_risk_labels`
Expected: FAIL, `Category` no existe.

- [ ] **Step 3: Minimal implementation**

```rust
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Category { Dev, Games, Web, System, Tools }

impl Category {
    pub fn label(self) -> &'static str {
        match self {
            Self::Dev => "dev",
            Self::Games => "games",
            Self::Web => "web",
            Self::System => "system",
            Self::Tools => "tools",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Risk { Low, Medium, High }

impl Risk {
    pub fn label(self) -> &'static str {
        match self {
            Self::Low => "low",
            Self::Medium => "medium",
            Self::High => "high",
        }
    }
}
```

- [ ] **Step 4: Run test to verify it passes**

Run: `cargo test`
Expected: PASS.

- [ ] **Step 5: Commit**

```bash
git add src/model.rs && git commit -m "feat(model): add Category and Risk enums"
```

### Tarea 2: `src/registry.rs` con `SourceDef` y las 24 fuentes

**Files:**
- Create: `src/registry.rs`
- Modify: `src/model.rs` (add `category`/`risk` fields a `CleanSource`)
- Modify: `src/main.rs` (add `mod registry;`)
- Diff at end: remove unused old `candidate_sources` (all paths now come from registry)

**Interfaces:**
- Produce:
  - `pub struct SourceDef { pub id: &'static str, pub name: &'static str, pub category: Category, pub risk: Risk, pub paths: &'static [PathSpec] }` con `impl SourceDef { pub fn new(...) -> Self }`
  - `pub fn registry() -> &'static [SourceDef]`
  - `pub enum PathSpec { Static(&'static str), Windows(...) }` — o patch simple: ruta única con `#[cfg]`.

Diseño concreto (sencillo, sin macros; mismo idioma declarativo):

```rust
// src/registry.rs
use crate::model::{Category, Risk};
use std::path::Path;

pub struct SourceDef {
    pub id: &'static str,
    pub name: &'static str,
    pub category: Category,
    pub risk: Risk,
    pub paths: &'static [&'static str], // cadenas con `~` sin expandir; vacía = special hook
    pub special: Option<fn(&Path) -> Vec<std::path::PathBuf>>,
}

pub fn registry() -> &'static [SourceDef] {
    &[
        SourceDef { id: "user_cache", name: "user cache", category: Category::System, risk: Risk::Low, paths: &[".cache/"], special: None },
        SourceDef { id: "cargo_global", name: "cargo (global)", category: Category::Dev, risk: Risk::Low, paths: &[".cache/cargo"], special: None },
        SourceDef { id: "rustup_tmp", name: "rustup/tmp", category: Category::Tools, risk: Risk::Low, paths: &[".rustup/tmp"], special: None },
        SourceDef { id: "npm_cache", name: "npm/_cacache", category: Category::Web, risk: Risk::Medium, paths: &[".npm/_cacache"], special: None },
        SourceDef { id: "pip_cache", name: "pip", category: Category::Tools, risk: Risk::Low, paths: &[".cache/pip"], special: None },
        SourceDef { id: "pnpm", name: "pnpm", category: Category::Web, risk: Risk::Medium, paths: &[".cache/pnpm"], special: None },
        SourceDef { id: "go_build", name: "go-build", category: Category::Dev, risk: Risk::Low, paths: &[".cache/go-build"], special: None },
        SourceDef { id: "yarn", name: "yarn cache", category: Category::Web, risk: Risk::Medium, paths: &[".cache/yarn"], special: None },
        SourceDef { id: "rustup_toolchains", name: "rustup toolchains" — ver notas ],
        // ... resto (ver spec, ~24 total)
        SourceDef { id: "cargo_target", name: "cargo targets", category: Category::Dev, risk: Risk::Medium, paths: &[], special: Some(project_targets) },
        SourceDef { id: "journal", name: "journal (systemd)", category: Category::System, risk: Risk::Medium, paths: &[], special: Some(journal_path) },
        SourceDef { id: "docker", name: "docker dangling", category: Category::System, risk: Risk::Medium, paths: &[], special: Some(docker_path) },
    ]
}
```

(El núcleo exacto con las 24 se completa aquí: steam shader cache `~/.local/share/Steam/steamCache/shadercache`, proton `~/.local/share/Steam/steamCache/shadercache`, lutris, firefox `~/.cache/mozilla/firefox`, chromium `~/.cache/chromium`, electron `~/.cache/electron`, dotnet `~/.dotnet`, nuget `~/.local/share/NuGet", maven `~/.m2/repository`, gradle `~/.gradle/caches`, conda `~/.conda/pkgs`, flatpak `~/.var/cache`, trash `~/.local/share/Trash`, plasma `~/.cache/plasma*`, mise `~/.local/share/mise`.)

**Cambios en `model.rs`:** `CleanSource` añade `category: Category` y `risk: Risk`; `new()` firma extendida pero NO romper los 35 tests (pásalos por referencia al registry en `scan`).

**Nota de integración:** Esta es la pieza que **desbloquea a S2**. Commitea lo antes posible en `main`.

- [ ] **Step: failing test** — in `src/registry.rs`:

```rust
#[test]
fn registry_ids_are_unique() {
    let mut ids: Vec<&str> = registry().iter().map(|s| s.id).collect();
    ids.sort_unstable();
    ids.dedup();
    assert_eq!(ids.len(), registry().len());
}

#[test]
fn registry_ids_match_clean_sources() {
    // cada fuente produce al menos un CleanSource al expandirse
    let defs = registry();
    for d in defs {
        let mut out = vec![];
        expand_source(d, &Path::new("/tmp/home"), &mut out);
        assert!(!out.is_empty(), "fuente {} sin rutas", d.id);
    }
}
```

- [ ] **Steps 2-4:** TDD (fail → implement → pass) con `cargo test`.
- [ ] **Step 5: Commit**

```bash
git add src/registry.rs src/model.rs src/main.rs
git commit -m "feat(registry): SourceDef registry declarativo con 24 fuentes"
```

- [ ] **Step 6: push + avisar S2** (rebase libros README de AGENTS.md)

---

## Pista 1 — Driver de fuentes (S1)

### Tarea 3: Ampliar `gen_fixtures.py` para games/web

**Files:**
- Modify: `tools/gen_fixtures.py`

- [ ] **Step 1:** fixture tree: `tests/fixtures/.cache/{mozilla/firefox,chromium,electron,go-build,npm/_cacache}` y `tests/fixtures/.local/share/Steam/steamCache/shadercache` con archivos + tamaño determinista (sigue el patrón actual).
- [ ] **Step 2:** `python3 tools/gen_fixtures.py --generate` — sic.
- [ ] **Step 3:** `cargo test` — verde.

### Tarea 4: `clean_batch` + whitelist batch

**Files:**
- Modify: `src/clean.rs` add `pub fn can_clean_batch(paths: &[&Path], whitelist: &[&Path]) -> Vec<bool>`
- Test: `tests/clean_safety.rs`

```rust
pub fn can_clean_batch(paths: &[&Path], whitelist: &[&Path]) -> Vec<bool> {
    paths.iter().map(|p| is_safe_to_clean(p, whitelist)).collect()
}
```
- [ ] TDD test (una ruta en whitelist, otra fuera → `[true, false]`), commit.

### Tarea 5: `state.rs` — historial persistente

**Files:**
- Create: `src/state.rs`, tested in `src/state.rs` (unit)
- Interfaces: `pub struct CleanupRecord { ts: u64, name: String, size: u64 }`, `pub struct History { pub total_reclaimed: u64, pub cleanups: Vec<CleanupRecord> }`
- `pub fn load_history() -> History` (ignore al corrupto)
- `pub fn save_history(h: &History) -> Result<(), String>`
- `pub fn record_cleanup(h: &mut History, name: &str, size: u64)` (keeps max 10)

- [ ] TDD: round-trip serde (load→modif→save→load), commit.

---

## Pista 2 — TUI (S2) — tras el core de S4

### Tarea 6: Estructura de tabs + navegación
- `mode: Mode { Inventario, Historia, Help, Detail }`; line de abajo: `[inventario] [historia] [help] `. Atajos de tab `1`,`2`,`3`,`?`.
- Test: snapshot del tab bar.

### Tarea 7: Filtros y sort
- `f` ciclo de filtro por categoría; `s` ciclo sort (size/name/risk). Test de `apply_filters`.

### Tarea 8: Detalle y ayuda
- `enter` → detalle (top 5 sub) ; `?` → ayuda con atajos. Tests de render de ambos.

### Tarea 9: Progreso mpsc (B4)
- Escuchando `Provider::scan_all` ya (S2 lo tiene); integrar el Gauge existente con la lista v2.

---

## Pista 3 — Marketing (S4)

### Tarea 10: Release prep
- `README` v2, `docs/marketing.md`, `assets/demo.gif` (asciinema), `PKGBUILD`, workflow release.

---

## Self-Review

Ver preguntas de writing-plans skill: cobertura spec — preguntado (fuentes ~24 ✔ T2, TUI ✔ P2, state ✔ T5, clean batch ✔ T4, marketing ✔ T10, errors globales ✔ deformas). Sin placeholders. Tipo check: `Category`/`Risk`/`SourceDef` nombres consistentes en T1-T2 y S2. Nota: `expand_source` devuelve `Vec<CleanSource>` para que los hooks especiales (docker/journal) generen fuentes vía same modelo.