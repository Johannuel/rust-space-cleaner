#[path = "../src/clean.rs"]
mod clean;

use std::path::Path;

use clean::is_safe_to_clean;

fn whitelist_base() -> Vec<&'static Path> {
    vec![
        Path::new("/home/mfcode/.cache"),
        Path::new("/home/mfcode/.cache/cargo"),
        Path::new("/home/mfcode/.cargo"),
        Path::new("/home/mfcode/.rustup/tmp"),
        Path::new("/home/mfcode/.npm/_cacache"),
    ]
}

#[test]
fn aprueba_ruta_exacta_de_la_whitelist() {
    let whitelist = whitelist_base();
    assert!(is_safe_to_clean(
        Path::new("/home/mfcode/.cache/cargo"),
        &whitelist
    ));
    assert!(is_safe_to_clean(
        Path::new("/home/mfcode/.rustup/tmp"),
        &whitelist
    ));
    assert!(is_safe_to_clean(
        Path::new("/home/mfcode/.npm/_cacache"),
        &whitelist
    ));
}

#[test]
fn aprueba_subdirectorio_directo_de_cache() {
    let whitelist = whitelist_base();
    assert!(is_safe_to_clean(
        Path::new("/home/mfcode/.cache/paru"),
        &whitelist
    ));
    assert!(is_safe_to_clean(
        Path::new("/home/mfcode/.cache/pip"),
        &whitelist
    ));
}

#[test]
fn rechaza_subdirectorio_anidado_de_cache() {
    let whitelist = whitelist_base();
    assert!(!is_safe_to_clean(
        Path::new("/home/mfcode/.cache/paru/tmp"),
        &whitelist
    ));
}

#[test]
fn rechaza_todo_cache_a_secas() {
    let whitelist = whitelist_base();
    assert!(!is_safe_to_clean(
        Path::new("/home/mfcode/.cache"),
        &whitelist
    ));
}

#[test]
fn rechaza_prefijo_falso_de_cargo() {
    let whitelist = whitelist_base();
    assert!(!is_safe_to_clean(
        Path::new("/home/mfcode/.cargo_evil"),
        &whitelist
    ));
    assert!(!is_safe_to_clean(
        Path::new("/home/mfcode/.cacheevil"),
        &whitelist
    ));
}

#[test]
fn rechaza_prefijo_falso_anidado() {
    let whitelist = whitelist_base();
    assert!(!is_safe_to_clean(
        Path::new("/home/mfcode/.rustup/tmp_evil"),
        &whitelist
    ));
    assert!(!is_safe_to_clean(
        Path::new("/home/mfcode/.rustup/tmp/otra"),
        &whitelist
    ));
}

#[test]
fn rechaza_rutas_fuera_de_la_whitelist() {
    let whitelist = whitelist_base();
    assert!(!is_safe_to_clean(Path::new("/etc"), &whitelist));
    assert!(!is_safe_to_clean(
        Path::new("/home/mfcode/Documents"),
        &whitelist
    ));
    assert!(!is_safe_to_clean(Path::new("/tmp"), &whitelist));
}

#[test]
fn rechaza_home_y_raiz() {
    let whitelist = whitelist_base();
    assert!(!is_safe_to_clean(Path::new("/home/mfcode"), &whitelist));
    assert!(!is_safe_to_clean(Path::new("/"), &whitelist));
}

#[test]
fn normaliza_puntos_y_barra_final() {
    let whitelist = whitelist_base();
    assert!(is_safe_to_clean(
        Path::new("/home/mfcode/.cache/cargo/"),
        &whitelist
    ));
    assert!(is_safe_to_clean(
        Path::new("/home/mfcode/.cache/./cargo"),
        &whitelist
    ));
    assert!(!is_safe_to_clean(
        Path::new("/home/mfcode/.cache/../.cache"),
        &whitelist
    ));
}

#[test]
fn whitelist_vacia_rechaza_todo() {
    let whitelist: Vec<&Path> = vec![];
    assert!(!is_safe_to_clean(
        Path::new("/home/mfcode/.cache/cargo"),
        &whitelist
    ));
}
