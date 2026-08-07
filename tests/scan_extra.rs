#![allow(dead_code)]

#[path = "../src/scan.rs"]
mod scan;

#[path = "../src/model.rs"]
mod model;

#[path = "../src/registry.rs"]
mod registry;

use std::path::Path;

use model::{CleanSource, ScanStatus};
use scan::{candidate_sources, is_cargo_target, scan_all};

const RAIZ: &str = env!("CARGO_MANIFEST_DIR");

fn home_fixtures() -> String {
    format!("{RAIZ}/tests/fixtures")
}

fn generar_fixtures() -> bool {
    std::process::Command::new("python3")
        .args(["tools/gen_fixtures.py", "--generate"])
        .current_dir(RAIZ)
        .output()
        .map(|o| o.status.success())
        .unwrap_or(false)
}

fn es_root() -> bool {
    std::process::Command::new("id")
        .arg("-u")
        .output()
        .map(|o| String::from_utf8_lossy(&o.stdout).trim() == "0")
        .unwrap_or(false)
}

#[test]
fn cache_usuario_mide_solo_subdirectorios_directos() {
    if !generar_fixtures() {
        eprintln!("skip: no hay python3");
        return;
    }
    let fuentes = candidate_sources(Path::new(&home_fixtures()));
    let alfa = fuentes
        .iter()
        .find(|f| f.name.ends_with("alpha"))
        .expect("candidate_sources debe listar .cache/alpha");

    let resultados = scan_all(vec![alfa.clone()]);
    assert_eq!(resultados[0].status, ScanStatus::Ok);
    assert_eq!(resultados[0].size_bytes, 1024);
}

#[test]
fn targets_reales_detectan_solo_cargo_targets() {
    if !generar_fixtures() {
        eprintln!("skip: sin python3");
        return;
    }
    assert!(is_cargo_target(Path::new(&format!(
        "{}/Projects/buena/target",
        home_fixtures()
    ))));
    assert!(!is_cargo_target(Path::new(&format!(
        "{}/Projects/falsa/target",
        home_fixtures()
    ))));

    let home = home_fixtures();
    let fuentes = candidate_sources(Path::new(&home));
    let reales: Vec<_> = fuentes
        .iter()
        .filter(|f| f.name.starts_with("target ("))
        .collect();

    assert!(
        reales.iter().any(|f| f.name == "target (buena)"),
        "buena tiene marca .fingerprint/build"
    );
    assert!(
        !reales.iter().any(|f| f.name.contains("falsa")),
        "falsa NO tiene marca y debe ignorarse"
    );

    let buena = reales.iter().find(|f| f.name == "target (buena)").unwrap();
    let resultado = scan_all(vec![(*buena).clone()]);
    assert_eq!(resultado[0].status, ScanStatus::Ok);
    assert_eq!(resultado[0].size_bytes, 2 + 96);
}

#[test]
fn journal_sin_permiso_se_reporta_sin_panico() {
    if !generar_fixtures() {
        eprintln!("skip: sin python3");
        return;
    }
    if es_root() {
        eprintln!("skip: ejecutando como root no se puede simular EACCES");
        return;
    }
    let ruta = format!("{}/journal/sin_permiso", home_fixtures());
    let fuente = CleanSource::new("journal", "sin_permiso", ruta.into());
    let resultados = scan_all(vec![fuente]);
    assert_eq!(resultados[0].status, ScanStatus::Error);
    assert_eq!(
        resultados[0].detail.as_deref(),
        Some("permission denied (EACCES)")
    );
}

#[test]
fn fuente_inexistente_se_marca_not_found() {
    let ruta = format!("{}/no/existe", home_fixtures());
    let fuente = CleanSource::new("rustup_tmp", "inexistente", ruta.into());
    let resultados = scan_all(vec![fuente]);
    assert_eq!(resultados[0].status, ScanStatus::NotFound);
}

#[test]
fn home_dir_siempre_devuelve_algo() {
    assert!(!scan::home_dir().as_os_str().is_empty());
}

#[test]
fn docker_dangling_opcional_por_env() {
    let Ok(comando) = std::env::var("SCAN_TEST_DOCKER_CMD") else {
        eprintln!("skip: define SCAN_TEST_DOCKER_CMD para probar docker");
        return;
    };
    let fuente = CleanSource::new("docker", "docker dangling", "docker-cli".into());
    let resultados = scan_all(vec![fuente]);
    assert_eq!(resultados[0].status, ScanStatus::Ok);
    let detalle = resultados[0].detail.as_deref().unwrap_or_default();
    assert!(detalle.contains("dangling images"));
    let _ = comando;
}
