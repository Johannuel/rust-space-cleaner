# Design: rust-space-cleaner

Fecha: 2026-08-06
Estado: propuesto (pendiente de revisión)

## Problema

El disco se llena con cachés de paquetes, builds y dependencias (Arch, Docker,
Rust). Revisar a mano qué ocupa espacio y qué se puede borrar de forma segura
es lento y arriesgado. Este proyecto practica Rust con ratatui, igual que
`persona-rpg`, y encaja con el stack del perfil (Rust, Linux/Arch, Docker).

## Objetivo

Un TUI que:

1. Escanea fuentes conocidas de caché y muestra su tamaño, ordenado de mayor a
   menor.
2. Permite borrar una fuente con confirmación explícita.
3. Nunca borra archivos de usuario: solo carpetas tipo "caché" de una
   whitelist explícita.
4. Es dry-run por defecto (solo análisis); limpiar requiere una tecla explícita
   y confirmación.

## Fuentes de limpieza (whitelist de escaneo)

| Fuente          | Ruta (con `~` expandido)            | Notas                          |
|-----------------|-------------------------------------|--------------------------------|
| `cache_usuario` | `~/.cache`                           | solo subdirectorios directos, nunca todo `~/.cache` de golpe |
| `cargo_target`  | `~/.cache/cargo`, `*/target`         | incluye `target/` de proyectos Rust |
| `rustup_tmp`    | `~/.rustup/tmp`                      |                                |
| `npm_cache`     | `~/.npm/_cacache`, `~/.cache/pnpm`   |                                |
| `pip_cache`     | `~/.cache/pip`                       |                                |
| `journal`       | `/var/log/journal`                   | requiere sudo (puede fallar)   |
| `docker`        | imágenes/volúmenes `dangling`        | vía `docker` CLI                |

Faltas de permisos (EACCES) se reportan como `error` en el estado de la
fuente, no tumban la app.

### Decisión sobre `~/.cache`

Se listan solo los subdirectorios directos (1 nivel) de `~/.cache` y se
muestran los mayores. La limpieza solo borra subdirectorios concretos, nunca
todo `~/.cache` de una vez.

## Arquitectura

```
src/
  main.rs   -> arranque ratatui + bucle de eventos
  model.rs  -> tipos: CleanSource, ScanResult, CleanError
  scan.rs   -> detección de fuentes + medición de tamaño (con threads)
  clean.rs  -> borrado seguro (whitelist + confirmación)
  ui.rs     -> lista ordenada, spinner, modal de confirmación
```

- `model.rs` define los datos; `scan.rs` los produce; `clean.rs` los consume;
  `ui.rs` solo pinta y captura teclas.
- La limpieza borra SOLO rutas que sean un subdirectorio exacto dentro de la
  whitelist (nunca `$HOME` a secas, nunca la raíz de un proyecto).

## UI (ratatui + crossterm)

- Pantalla principal: lista de fuentes con tamaño formateado (B/KB/MB/GB) y un
  estado por fila: `scan`, `ok`, `error`, `no encontrado`.
- Globos de control: `q`/`Esc` salir, `r` reescanear, `s` seleccionar fuente
  para limpiar, `y`/`n` en el modal de confirmación.
- Dry-run: `true` por defecto; sin una confirmación explícita no se borra nada.

## Manejo de errores

- `scan.rs` y `clean.rs` devuelven `Result`; un fallo de permisos o de comando
  externo (docker) se convierte en estado `error` de esa fuente.
- No hay panics con datos reales del usuario.

## Tests

- `tests` para el formateo de bytes (`human_size`).
- `tests` de seguridad en `clean.rs`: ruta exacta en whitelist, prefijo falso
  (`~/.cargo_evil`), subdirectorio de `~/.cache`, ruta fuera de la whitelist.
- `cargo test`, `cargo clippy`, `cargo fmt` limpios.

## Fuera de alcance (YAGNI)

- Navegador de árbol completo del filesystem (estilo ncdu).
- Soporte multiplataforma: ya cubierto (Windows, Linux y macOS) — `scan.rs` detecta las rutas de caché por SO (`cfg target_os`); el código sigue siendo portable.
- Fichero de configuración editable; los defaults van en código por ahora.
- Sync con la nube o interfaz web.

## Riesgos conocidos

- `target/` de proyectos: hay que confirmar que es un `target` de cargo
  (`.fingerprint`/`build` dentro) antes de sugerir borrarlo entero; evitar
  borrar `target/` de proyectos ajenos o rutas con enlaces simbólicos a /
- Los cachés pueden regenerarse (no son datos de usuario), pero nunca se borra
  sin confirmación.