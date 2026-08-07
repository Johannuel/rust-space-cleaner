#[path = "../src/clean.rs"]
mod clean;

use std::path::{Path, PathBuf};

use clean::is_safe_to_clean;

fn make_whitelist(base: &Path) -> Vec<PathBuf> {
    vec![
        base.join(".cache"),
        base.join(".cache/cargo"),
        base.join(".cargo"),
        base.join(".rustup/tmp"),
        base.join(".npm/_cacache"),
    ]
}

fn to_refs<P: AsRef<Path>>(paths: &[P]) -> Vec<&Path> {
    paths.iter().map(|p| p.as_ref()).collect()
}

#[test]
fn aprueba_ruta_exacta_de_la_whitelist() {
    let tmp = tempfile::tempdir().unwrap();
    let paths = make_whitelist(tmp.path());
    let whitelist = to_refs(&paths);
    assert!(is_safe_to_clean(
        &tmp.path().join(".cache/cargo"),
        &whitelist
    ));
    assert!(is_safe_to_clean(
        &tmp.path().join(".rustup/tmp"),
        &whitelist
    ));
    assert!(is_safe_to_clean(
        &tmp.path().join(".npm/_cacache"),
        &whitelist
    ));
}

#[test]
fn aprueba_subdirectorio_directo_de_cache() {
    let tmp = tempfile::tempdir().unwrap();
    let paths = make_whitelist(tmp.path());
    let whitelist = to_refs(&paths);
    assert!(is_safe_to_clean(
        &tmp.path().join(".cache/paru"),
        &whitelist
    ));
    assert!(is_safe_to_clean(&tmp.path().join(".cache/pip"), &whitelist));
}

#[test]
fn rechaza_subdirectorio_anidado_de_cache() {
    let tmp = tempfile::tempdir().unwrap();
    let paths = make_whitelist(tmp.path());
    let whitelist = to_refs(&paths);
    assert!(!is_safe_to_clean(
        &tmp.path().join(".cache/paru/tmp"),
        &whitelist
    ));
}

#[test]
fn rechaza_todo_cache_a_secas() {
    let tmp = tempfile::tempdir().unwrap();
    let paths = make_whitelist(tmp.path());
    let whitelist = to_refs(&paths);
    assert!(!is_safe_to_clean(&tmp.path().join(".cache"), &whitelist));
}

#[test]
fn rechaza_prefijo_falso_de_cargo() {
    let tmp = tempfile::tempdir().unwrap();
    let paths = make_whitelist(tmp.path());
    let whitelist = to_refs(&paths);
    assert!(!is_safe_to_clean(
        &tmp.path().join(".cargo_evil"),
        &whitelist
    ));
    assert!(!is_safe_to_clean(
        &tmp.path().join(".cacheevil"),
        &whitelist
    ));
}

#[test]
fn rechaza_prefijo_falso_anidado() {
    let tmp = tempfile::tempdir().unwrap();
    let paths = make_whitelist(tmp.path());
    let whitelist = to_refs(&paths);
    assert!(!is_safe_to_clean(
        &tmp.path().join(".rustup/tmp_evil"),
        &whitelist
    ));
    assert!(!is_safe_to_clean(
        &tmp.path().join(".rustup/tmp/otra"),
        &whitelist
    ));
}

#[test]
fn rechaza_rutas_fuera_de_la_whitelist() {
    let tmp = tempfile::tempdir().unwrap();
    let paths = make_whitelist(tmp.path());
    let whitelist = to_refs(&paths);
    assert!(!is_safe_to_clean(&tmp.path().join("Documents"), &whitelist));
    assert!(!is_safe_to_clean(Path::new("/etc"), &whitelist));
    assert!(!is_safe_to_clean(Path::new("/"), &whitelist));
}

#[test]
fn rechaza_home_y_raiz() {
    let tmp = tempfile::tempdir().unwrap();
    let paths = make_whitelist(tmp.path());
    let whitelist = to_refs(&paths);
    assert!(!is_safe_to_clean(tmp.path(), &whitelist));
    assert!(!is_safe_to_clean(Path::new("/"), &whitelist));
}

#[test]
fn normaliza_puntos_y_barra_final() {
    let tmp = tempfile::tempdir().unwrap();
    let paths = make_whitelist(tmp.path());
    let whitelist = to_refs(&paths);
    assert!(is_safe_to_clean(
        &tmp.path().join(".cache/cargo/"),
        &whitelist
    ));
    assert!(is_safe_to_clean(
        &tmp.path().join(".cache/./cargo"),
        &whitelist
    ));
    assert!(!is_safe_to_clean(
        &tmp.path().join(".cache/../.cache"),
        &whitelist
    ));
}

#[test]
fn whitelist_vacia_rechaza_todo() {
    let empty: Vec<&Path> = vec![];
    let tmp = tempfile::tempdir().unwrap();
    assert!(!is_safe_to_clean(&tmp.path().join(".cache/cargo"), &empty));
}
