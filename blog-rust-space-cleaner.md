# Limpia tu disco sin romper nada: rust-space-cleaner (Rust + ratatui)

> Publicado para Hashnode. Repo: [Johannuel/rust-space-cleaner](https://github.com/Johannuel/rust-space-cleaner)

Tu SSD se llena y no sabes por qué. `du -sh ~/.cache/*` dice la verdad: gigabytes en cachés de paquetes, `target/` de proyectos Rust, imágenes colgadas de Docker y logs de systemd. Borrarlos a mano es lento; borrarlos con `rm -rf` a lo loco es cómo se pierde la carpeta equivocada.

**rust-space-cleaner** es un limpiador de cachés con TUI para Arch/Linux, escrito en **Rust** con **ratatui**. Escanea las fuentes de bloat típicas, te muestra cuánto ocupa cada una ordenada de mayor a menor, y solo borra lo que tú confirmes — y solo carpetas de caché de una whitelist explícita.

## Por qué otra herramienta de limpieza

Porque la mayoría son scripts de `rm` con esteroides. Esta se diseñó con una regla de hierro:

> **Nunca se borra nada sin confirmación. Nunca se toca lo que no está en la whitelist.**

- **Dry-run por defecto.** El modo por defecto es solo análisis. Limpiar requiere una tecla explícita (`s`) y confirmación (`y`).
- **Whitelist explícita.** Se borran solo rutas exactas de caché (`~/.cache/cargo`, `~/.rustup/tmp`, ...) o subdirectorios directos de `~/.cache`. Nada más.
- **Normas duras testeadas.** `$HOME`, `/`, prefijos falsos (`~/.cargo_evil`) y cualquier cosa fuera de la whitelist se rechazan. Hay tests de seguridad para cada caso.
- **Los errores no tumban la app.** `/var/log/journal` sin permisos da un estado `error` en esa fila (EACCES capturado), no un panic.

## Cómo funciona

Tres piezas separadas, tal y como pide el diseño:

```
scan.rs   detecta fuentes y mide tamaños (con threads, sin seguir symlinks)
clean.rs  is_safe_to_clean(path, whitelist) — borrado seguro, hecho con TDD
ui.rs     lista ordenada, spinner, modal de confirmación (ratatui + crossterm)
```

El escaneo mide en paralelo. `target/` solo cuenta si es un target real de cargo: tiene que tener `.fingerprint/` o `build/` dentro — así no sugieres borrar un directorio con ese nombre por casualidad.

## Qué limpia

| Fuente | Ruta |
|---|---|
| Caché de usuario | `~/.cache/*` (subdirectorios directos) |
| Cargo | `~/.cache/cargo` + `target/` reales en `~/Projects` |
| Rustup | `~/.rustup/tmp` |
| npm / pnpm | `~/.npm/_cacache`, `~/.cache/pnpm` |
| pip | `~/.cache/pip` |
| Journal de systemd | `/var/log/journal` (requiere sudo) |
| Docker | imágenes `dangling` vía CLI |

## Pruébalo

```bash
git clone https://github.com/Johannuel/rust-space-cleaner
cd rust-space-cleaner
cargo run --release
```

Teclas: `↑↓/jk` navegar, `s` preparar limpieza, `y/n` confirmar, `r` reescanear, `q/Esc` salir.

## Calidad

- 28 tests: unitarios, de integración con un árbol de fixtures simulado y de seguridad de la whitelist.
- `cargo clippy --all-targets -- -D warnings` limpio y CI en GitHub Actions (`fmt` + `clippy` + `test`).
- Fixtures deterministas generados por `tools/gen_fixtures.py`.

## Roadmap

- Borrado real con log de actividad.
- `docker system prune` para contenedores.
- Whitelist configurable por archivo.

## ¿Te sirve?

Si el disco se te llena, pruébalo. Si te gusta la idea de una TUI segura en Rust, ⭐ el repo y abre issues con lo que te faltaría. Aprender Rust haciendo algo que usas cada semana es la mejor motivación que hay.

---

### Blurb para compartir

**Reddit (r/rust, r/archlinux, r/linux):**

> TUI cache-cleaner for Arch/Linux written in Rust with ratatui. Scans ~/.cache, real cargo targets, systemd journal and Docker dangling images; only deletes whitelisted cache dirs after explicit confirmation. Dry-run by default, EACCES handled without panics, 28 tests + CI. Looking for feedback on the safety model and the TUI. https://github.com/Johannuel/rust-space-cleaner

**X/Twitter:**

> Build a safe TUI cache-cleaner for Arch/Linux in Rust + ratatui 🧹 Dry-run by default, whitelist-only deletion, 28 tests, CI green. Scan: ~/.cache, cargo targets, journal, docker dangling. PRs/issues welcome! https://github.com/Johannuel/rust-space-cleaner #rustlang #ratatui #linux
