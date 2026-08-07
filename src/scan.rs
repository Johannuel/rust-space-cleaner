use std::env;
use std::fs;
use std::io;
use std::path::{Path, PathBuf};
use std::process::Command;

use crate::model::{CleanSource, ScanStatus};

pub fn home_dir() -> PathBuf {
    #[cfg(target_os = "windows")]
    let home = env::var_os("USERPROFILE").map(PathBuf::from);

    #[cfg(not(target_os = "windows"))]
    let home = env::var_os("HOME").map(PathBuf::from);

    home.unwrap_or_else(|| PathBuf::from("."))
}

pub fn candidate_sources(home: &Path) -> Vec<CleanSource> {
    let mut lista = Vec::new();
    cache_usuario(home, &mut lista);
    cargo_global(home, &mut lista);
    targets_de_proyectos(home, &mut lista);
    rustup_tmp(home, &mut lista);
    npm_pnpm(home, &mut lista);
    pip_cache(home, &mut lista);
    journal(&mut lista);
    docker_dangling_fuente(&mut lista);
    lista
}

fn subdirector_como_fuente(
    lista: &mut Vec<CleanSource>,
    id: &'static str,
    prefijo: &str,
    ruta: PathBuf,
) {
    if let Some(nombre) = ruta.file_name() {
        lista.push(CleanSource::new(
            id,
            &format!("{prefijo}{}", nombre.to_string_lossy()),
            ruta,
        ));
    }
}

fn cache_usuario(home: &Path, lista: &mut Vec<CleanSource>) {
    #[cfg(target_os = "windows")]
    let base = {
        let app = env::var_os("LOCALAPPDATA").map(PathBuf::from);
        app.map(|ruta| ruta.join("Temp"))
    };
    #[cfg(target_os = "linux")]
    let base = Some(home.join(".cache"));
    #[cfg(target_os = "macos")]
    let base = Some(home.join("Library/Caches"));
    #[cfg(not(any(target_os = "windows", target_os = "linux", target_os = "macos")))]
    let base: Option<PathBuf> = None;

    if let Some(base) = base {
        for ruta in subdirectorios_directos(&base) {
            subdirector_como_fuente(lista, "cache_usuario", "cache_usuario/", ruta);
        }
    }
}

fn cargo_global(home: &Path, lista: &mut Vec<CleanSource>) {
    #[cfg(target_os = "linux")]
    let ruta = home.join(".cache/cargo");
    #[cfg(not(target_os = "linux"))]
    let ruta = home.join(".cargo/registry");

    lista.push(CleanSource::new("cargo_target", "cargo (global)", ruta));
}

fn targets_de_proyectos(home: &Path, lista: &mut Vec<CleanSource>) {
    let raices = [home.join("Projects"), home.join("code"), home.join("dev")];
    for raiz in raices {
        for target in encontrar_targets_reales(&raiz) {
            let nombre = target
                .parent()
                .and_then(Path::file_name)
                .map(|n| n.to_string_lossy().into_owned())
                .unwrap_or_else(|| "target".to_string());
            lista.push(CleanSource::new(
                "cargo_target",
                &format!("target ({nombre})"),
                target,
            ));
        }
    }
}

fn rustup_tmp(home: &Path, lista: &mut Vec<CleanSource>) {
    lista.push(CleanSource::new(
        "rustup_tmp",
        "rustup/tmp",
        home.join(".rustup/tmp"),
    ));
}

fn npm_pnpm(home: &Path, lista: &mut Vec<CleanSource>) {
    #[cfg(target_os = "windows")]
    let base = env::var_os("LOCALAPPDATA").map(PathBuf::from);
    #[cfg(not(target_os = "windows"))]
    let base = Some(home.to_path_buf());

    let Some(base) = base else { return };
    #[cfg(target_os = "windows")]
    {
        lista.push(CleanSource::new(
            "npm_cache",
            "npm-cache",
            base.join("npm-cache"),
        ));
        lista.push(CleanSource::new(
            "npm_cache",
            "pnpm-cache",
            base.join("pnpm-cache"),
        ));
    }
    #[cfg(not(target_os = "windows"))]
    {
        lista.push(CleanSource::new(
            "npm_cache",
            "npm/_cacache",
            base.join(".npm/_cacache"),
        ));
        lista.push(CleanSource::new(
            "npm_cache",
            "pnpm",
            base.join(".cache/pnpm"),
        ));
    }
}

fn pip_cache(home: &Path, lista: &mut Vec<CleanSource>) {
    #[cfg(target_os = "windows")]
    let base = env::var_os("LOCALAPPDATA")
        .map(PathBuf::from)
        .map(|app| app.join("pip/cache"));
    #[cfg(not(target_os = "windows"))]
    let base = Some(home.join(".cache/pip"));

    if let Some(base) = base {
        lista.push(CleanSource::new("pip_cache", "pip", base));
    }
}

fn journal(lista: &mut Vec<CleanSource>) {
    #[cfg(target_os = "linux")]
    lista.push(CleanSource::new(
        "journal",
        "journal (systemd)",
        PathBuf::from("/var/log/journal"),
    ));
}

fn docker_dangling_fuente(lista: &mut Vec<CleanSource>) {
    lista.push(CleanSource::new(
        "docker",
        "docker dangling",
        PathBuf::from("docker-cli"),
    ));
}

pub fn scan_all(sources: Vec<CleanSource>) -> Vec<CleanSource> {
    std::thread::scope(|ambito| {
        let hijos: Vec<_> = sources
            .into_iter()
            .map(|fuente| ambito.spawn(move || escanear_fuente(fuente)))
            .collect();
        hijos.into_iter().map(|h| h.join().unwrap()).collect()
    })
}

fn escanear_fuente(fuente: CleanSource) -> CleanSource {
    if fuente.id == "docker" {
        return escanear_docker(fuente);
    }
    match medir_dir(&fuente.path) {
        Ok(size) => {
            let mut f = fuente;
            f.size_bytes = size;
            f.status = ScanStatus::Ok;
            f
        }
        Err(ScanError::Inexistente) => {
            let mut f = fuente;
            f.status = ScanStatus::NotFound;
            f
        }
        Err(ScanError::Permiso) => {
            let mut f = fuente;
            f.status = ScanStatus::Error;
            f.detail = Some("sin permisos (EACCES)".to_string());
            f
        }
        Err(_) => {
            let mut f = fuente;
            f.status = ScanStatus::Error;
            f
        }
    }
}

fn escanear_docker(fuente: CleanSource) -> CleanSource {
    let mut fuente = fuente;
    match docker_dangling() {
        Ok((size, cantidad)) => {
            fuente.size_bytes = size;
            fuente.status = ScanStatus::Ok;
            fuente.detail = Some(format!("{cantidad} imágenes dangling"));
        }
        Err(_) => {
            fuente.status = ScanStatus::Error;
            fuente.detail = Some("docker CLI no disponible".to_string());
        }
    }
    fuente
}

#[derive(Debug)]
enum ScanError {
    Inexistente,
    Permiso,
    Otro,
}

impl From<io::Error> for ScanError {
    fn from(err: io::Error) -> Self {
        match err.kind() {
            io::ErrorKind::NotFound => Self::Inexistente,
            io::ErrorKind::PermissionDenied => Self::Permiso,
            _ => Self::Otro,
        }
    }
}

fn medir_dir(raiz: &Path) -> Result<u64, ScanError> {
    let meta = fs::symlink_metadata(raiz)?;
    if meta.file_type().is_file() {
        return Ok(meta.len());
    }
    if !meta.file_type().is_dir() {
        return Err(ScanError::Otro);
    }
    let entradas = fs::read_dir(raiz)?;
    let mut total = 0;
    for entrada in entradas {
        let Ok(entrada) = entrada else { continue };
        let ruta = entrada.path();
        let Ok(meta) = fs::symlink_metadata(&ruta) else {
            continue;
        };
        let tipo = meta.file_type();
        if tipo.is_dir() && !tipo.is_symlink() {
            total += medir_dir(&ruta).unwrap_or(0);
        } else if tipo.is_file() {
            total += meta.len();
        }
    }
    Ok(total)
}

fn subdirectorios_directos(ruta: &Path) -> Vec<PathBuf> {
    let Ok(entradas) = fs::read_dir(ruta) else {
        return Vec::new();
    };
    entradas
        .filter_map(io::Result::ok)
        .map(|e| e.path())
        .filter(|p| p.is_dir())
        .collect()
}

pub fn es_target_cargo(path: &Path) -> bool {
    path.join(".fingerprint").is_dir() || path.join("build").is_dir()
}

fn encontrar_targets_reales(proyectos: &Path) -> Vec<PathBuf> {
    let mut targets = Vec::new();
    for entrada in subdirectorios_directos(proyectos) {
        let target = entrada.join("target");
        if es_target_cargo(&target) {
            targets.push(target);
        }
    }
    targets
}

fn docker_dangling() -> Result<(u64, usize), ScanError> {
    let comando = env::var("SCAN_TEST_DOCKER_CMD").unwrap_or_else(|_| "docker".to_string());
    let salida = Command::new(comando)
        .args([
            "images",
            "-a",
            "--filter",
            "dangling=true",
            "--format",
            "{{.Size}}",
        ])
        .output();
    let salida = match salida {
        Ok(s) if s.status.success() => s,
        _ => return Err(ScanError::Otro),
    };
    let texto = String::from_utf8_lossy(&salida.stdout);
    let mut total = 0;
    let mut cantidad = 0;
    for linea in texto.lines() {
        cantidad += 1;
        total += parse_tamano_humano(linea).unwrap_or(0);
    }
    Ok((total, cantidad))
}

fn parse_tamano_humano(cadena: &str) -> Option<u64> {
    let cadena = cadena.trim();
    if cadena.is_empty() {
        return None;
    }
    const SUFIJOS: [(&str, u64); 5] = [
        ("KiB", 1024),
        ("KB", 1024),
        ("MiB", 1024 * 1024),
        ("MB", 1024 * 1024),
        ("GiB", 1024 * 1024 * 1024),
    ];
    for (sufijo, factor) in SUFIJOS {
        if let Some(resto) = cadena.strip_suffix(sufijo) {
            let numero: f64 = resto.trim().parse().ok()?;
            return Some((numero * factor as f64) as u64);
        }
    }
    if let Some(resto) = cadena.strip_suffix('B') {
        let numero: f64 = resto.trim().parse().ok()?;
        return Some(numero as u64);
    }
    cadena.parse::<f64>().ok().map(|n| n as u64)
}

#[cfg(test)]
mod tests {
    use std::os::unix::fs::PermissionsExt;

    use super::*;

    #[test]
    fn reconoce_target_real() {
        let dir = env::temp_dir().join("scan_target_test_s3");
        let target = dir.join("target");
        fs::create_dir_all(target.join(".fingerprint")).unwrap();
        assert!(es_target_cargo(&target));

        let sin_marca = dir.join("sin_meta/target");
        fs::create_dir_all(&sin_marca).unwrap();
        assert!(!es_target_cargo(&sin_marca));

        fs::remove_dir_all(&dir).unwrap();
    }

    #[test]
    fn parsea_tamanos_docker() {
        assert_eq!(parse_tamano_humano("1.5MB"), Some(1_572_864));
        assert_eq!(parse_tamano_humano("42B"), Some(42));
        assert_eq!(parse_tamano_humano("1.5 MiB"), Some(1_572_864));
        assert_eq!(parse_tamano_humano("not-a-size"), None);
    }

    #[test]
    fn eacces_no_tira_panico() {
        let dir = env::temp_dir().join("scan_eacces_test");
        let bloqueado = dir.join("sin_permiso");
        fs::create_dir_all(&bloqueado).unwrap();
        fs::write(bloqueado.join("archivo"), b"datos").unwrap();
        fs::set_permissions(&bloqueado, fs::Permissions::from_mode(0o000)).unwrap();

        let fuente = CleanSource::new("journal", "fixture", bloqueado.clone());
        let resultado = scan_all(vec![fuente]);
        assert_eq!(resultado[0].status, ScanStatus::Error);

        fs::set_permissions(&bloqueado, fs::Permissions::from_mode(0o755)).unwrap();
        fs::remove_dir_all(&dir).unwrap();
    }
}
