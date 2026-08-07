# 🧹 rust-space-cleaner

Limpiador de caché para usuarios de Arch/Linux con **TUI en la terminal** (Rust + [ratatui](https://github.com/ratatui/ratatui)). Escanea fuentes típicas de bloat (cachés de paquetes, builds de Rust, Docker y logs del sistema), te muestra cuánto ocupa cada una y te deja limpiarlas de forma **segura**: solo borra carpetas de una whitelist explícita y siempre con confirmación.

> [!IMPORTANT]
> El modo por defecto es **dry-run**: nada se borra sin que lo confirmes. La herramienta solo limpia carpetas tipo "caché" de su whitelist; nunca toca archivos de usuario ni `$HOME`.

## ¿Para quién?

- **Arch/linuxeros** cuya SSD se llena y ya no quieren adivinar dónde quedaron los gigabytes.
- Quienes usan **Docker** con imágenes colgadas y **Rust** con `target/` acumulados.
- Cualquiera aprendiendo Rust que quiera una TUI real y segura para referenciar.

## Cómo funciona

1. **Escaneo** (`scan.rs`): detecta fuentes conocidas y mide su tamaño en `bytes` (con threads) sin seguir enlaces simbólicos.
2. **Vista TUI** (`ui.rs`): lista ordenada de mayor a menor con estado por fila (`scan | ok | error | no encontrado`), spinner mientras escanea y modal de confirmación.
3. **Limpieza segura** (`clean.rs`): `is_safe_to_clean(path, whitelist)` decide si una ruta puede borrarse (entrada exacta de la whitelist o subdirectorio directo de `~/.cache`). Todo lo demás se rechaza.

## Fuentes escaneadas

| Fuente | Ruta | Notas |
|---|---|---|
| Caché de usuario | `~/.cache/*` | solo subdirectorios directos, nunca todo `~/.cache` |
| Cargo (registry) | `~/.cargo/registry` | |
| Cargo `target/` | `target/` en `~/Projects` | solo si tiene `.fingerprint` o `build/` (es un *target* real) |
| Rustup tmp | `~/.rustup/tmp` | |
| npm / pnpm | `~/.npm/_cacache`, `~/.cache/pnpm` | |
| pip | `~/.cache/pip` | |
| Journal (systemd) | `/var/log/journal` | requiere sudo; error de permisos no tumba la app |
| Docker | imágenes `dangling` | vía `docker images --filter dangling=true` |

## Instalación

```bash
cargo run --release
```

## Uso

| Tecla | Acción |
|---|---|
| `↑` / `↓` o `j` / `k` | navegar por las fuentes |
| `s` | preparar limpieza de la selección |
| `y` / `n` | confirmar / cancelar en el modal |
| `r` | reescanear |
| `q` / `Esc` | salir |

## Seguridad (por diseño)

- **Dry-run por defecto**: la TUI avisa que nada se borra sin confirmar.
- **Whitelist**: solo se borran rutas explícitas (caché) o subdirectorios directos de `~/.cache`.
- **Normas duras**: `$HOME`, `/`, una caché completa de un nivel (p. ej. `~/.cache` a secas), prefijos falsos (`~/.cargo_evil`) y rutas fuera de la whitelist siempre son rechazadas y testeadas.

## Tests

```bash
cargo test                 # 28 tests (unit + integración + seguridad)
cargo clippy --all-targets -- -D warnings   # lints estrictas
cargo fmt --check
```

Los tests de integración usan un árbol simulado determinista que genera `tools/gen_fixtures.py` (y que está en `.gitignore`):

```bash
python3 tools/gen_fixtures.py --generate   # recrea tests/fixtures
python3 tools/gen_fixtures.py --clean      # lo elimina
```

El CI (`.github/workflows/ci.yml`) corre `fmt`, `clippy` y `test`en cada push/PR.

## Estructura

```
src/
  main.rs    -> arranque ratatui + provider real (scan + clean)
  model.rs   -> CleanSource, ScanStatus, human_size
  scan.rs    -> detección de fuentes + medición de tamaño (threads)
  clean.rs   -> is_safe_to_clean (whitelist) — TDD
  ui.rs      -> lista, spinner, modal de confirmación
tests/
  clean_safety.rs   # seguridad de la whitelist
  scan_extra.rs     # escaneo con fixtures simulados
tools/
  gen_fixtures.py   # genera/limpia el árbol de tests
```

## Roadmap

- [ ] Activar/desactivar dry-run en vivo (tecla dedicada).
- [ ] Soporte de contenedores conviven (docker system prune).
- [ ] Configudurable (rutas, whitelist) en un archivo.