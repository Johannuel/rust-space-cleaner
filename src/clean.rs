use std::ffi::OsString;
use std::path::{Component, Path};

/// Normaliza una ruta absoluta a sus componentes, resolviendo `.` y `..`.
/// Devuelve `None` si la ruta no es absoluta o usa un prefijo raro.
fn componentes_absolutos(path: &Path) -> Option<Vec<OsString>> {
    if !path.is_absolute() {
        return None;
    }

    let mut partes: Vec<OsString> = Vec::new();
    for componente in path.components() {
        match componente {
            Component::RootDir | Component::CurDir => {}
            Component::ParentDir => {
                partes.pop();
            }
            Component::Normal(nombre) => partes.push(nombre.to_os_string()),
            Component::Prefix(_) => return None,
        }
    }
    Some(partes)
}

/// ¿Es `camino` un subdirectorio estricto de `raiz` (raiz + al menos 1 nivel)?
fn es_prefijo_estricto(raiz: &[OsString], camino: &[OsString]) -> bool {
    raiz.len() < camino.len() && camino.starts_with(raiz)
}

/// ¿Es `entrada` un contenedor contemplado de la whitelist (tiene subentradas)?
/// Un contenedor se escanea listando sus hijos directos, pero nunca se borra
/// la carpeta entera (p. ej. `~/.cache`).
fn es_padre_contemplado(entrada: &[OsString], whitelist: &[&Path]) -> bool {
    whitelist
        .iter()
        .filter_map(|w| componentes_absolutos(w))
        .any(|otra| es_prefijo_estricto(entrada, &otra))
}

/// Indica si `path` puede borrarse de forma segura según la `whitelist`.
///
/// Reglas:
/// - Se aprueba una entrada exacta de la whitelist.
/// - Se aprueba un subdirectorio directo (1 nivel) de un contenedor contemplado
///   de la whitelist (p. ej. `~/.cache/*`).
/// - Se rechaza el contenedor a secas (`~/.cache`), `$HOME`, la raíz `/`,
///   prefijos falsos (`~/.cargo_evil`) y rutas fuera de la whitelist.
///
/// Las rutas llegan absolutas y con `~` ya expandido.
pub fn is_safe_to_clean(path: &Path, whitelist: &[&Path]) -> bool {
    let camino = match componentes_absolutos(path) {
        Some(c) if c.len() > 1 => c,
        _ => return false,
    };

    // Un ancestro de una entrada de la whitelist nunca es borrable ($HOME, /).
    if whitelist
        .iter()
        .filter_map(|w| componentes_absolutos(w))
        .any(|entrada| es_prefijo_estricto(&camino, &entrada))
    {
        return false;
    }

    for w in whitelist {
        let Some(entrada) = componentes_absolutos(w) else {
            continue;
        };
        if es_padre_contemplado(&entrada, whitelist) {
            if es_prefijo_estricto(&entrada, &camino) && camino.len() == entrada.len() + 1 {
                return true;
            }
        } else if camino == entrada {
            return true;
        }
    }

    false
}
