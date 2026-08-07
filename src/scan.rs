use std::env;
use std::fs;
use std::io;
use std::path::{Path, PathBuf};
use std::process::Command;

use crate::model::{CleanSource, ScanStatus};
use crate::registry::{SourceDef, expand_paths, registry};

pub fn home_dir() -> PathBuf {
    #[cfg(target_os = "windows")]
    let home = env::var_os("USERPROFILE").map(PathBuf::from);

    #[cfg(not(target_os = "windows"))]
    let home = env::var_os("HOME").map(PathBuf::from);

    home.unwrap_or_else(|| PathBuf::from("."))
}

pub fn candidate_sources(home: &Path) -> Vec<CleanSource> {
    let mut sources = Vec::new();
    for def in registry() {
        match def.id {
            "user_cache" => user_cache(home, def, &mut sources),
            "cargo_global" => cargo_global(home, def, &mut sources),
            "cargo_target" => project_targets(home, def, &mut sources),
            "journal" => journal(def, &mut sources),
            "docker" => docker_dangling_source(def, &mut sources),
            _ => {
                for path in expand_paths(def, home) {
                    sources.push(
                        CleanSource::new(def.id, def.name, path).with_meta(def.category, def.risk),
                    );
                }
            }
        }
    }
    sources
}

fn user_cache(home: &Path, def: &SourceDef, sources: &mut Vec<CleanSource>) {
    #[cfg(target_os = "windows")]
    let base = {
        let app = env::var_os("LOCALAPPDATA").map(PathBuf::from);
        app.map(|path| path.join("Temp"))
    };
    #[cfg(target_os = "linux")]
    let base = Some(home.join(".cache"));
    #[cfg(target_os = "macos")]
    let base = Some(home.join("Library/Caches"));
    #[cfg(not(any(target_os = "windows", target_os = "linux", target_os = "macos")))]
    let base: Option<PathBuf> = None;

    if let Some(base) = base {
        for path in direct_subdirs(&base) {
            push_subdir_source(sources, def, path);
        }
    }
}

fn push_subdir_source(sources: &mut Vec<CleanSource>, def: &SourceDef, path: PathBuf) {
    if let Some(name) = path.file_name() {
        sources.push(
            CleanSource::new(
                def.id,
                &format!("{}/{}", def.name, name.to_string_lossy()),
                path,
            )
            .with_meta(def.category, def.risk),
        );
    }
}

fn cargo_global(home: &Path, def: &SourceDef, sources: &mut Vec<CleanSource>) {
    #[cfg(target_os = "linux")]
    let path = home.join(".cache/cargo");
    #[cfg(not(target_os = "linux"))]
    let path = home.join(".cargo/registry");

    sources.push(CleanSource::new(def.id, def.name, path).with_meta(def.category, def.risk));
}

fn project_targets(home: &Path, def: &SourceDef, sources: &mut Vec<CleanSource>) {
    let roots = [home.join("Projects"), home.join("code"), home.join("dev")];
    for root in roots {
        for target in find_real_targets(&root) {
            let name = target
                .parent()
                .and_then(Path::file_name)
                .map(|n| n.to_string_lossy().into_owned())
                .unwrap_or_else(|| "target".to_string());
            sources.push(
                CleanSource::new(def.id, &format!("target ({name})"), target)
                    .with_meta(def.category, def.risk),
            );
        }
    }
}

fn journal(def: &SourceDef, sources: &mut Vec<CleanSource>) {
    #[cfg(target_os = "linux")]
    sources.push(
        CleanSource::new(def.id, def.name, PathBuf::from("/var/log/journal"))
            .with_meta(def.category, def.risk),
    );
}

fn docker_dangling_source(def: &SourceDef, sources: &mut Vec<CleanSource>) {
    sources.push(
        CleanSource::new(def.id, def.name, PathBuf::from("docker-cli"))
            .with_meta(def.category, def.risk),
    );
}

pub fn scan_all(sources: Vec<CleanSource>) -> Vec<CleanSource> {
    std::thread::scope(|scope| {
        let handles: Vec<_> = sources
            .into_iter()
            .map(|source| scope.spawn(move || scan_source(source)))
            .collect();
        handles.into_iter().map(|h| h.join().unwrap()).collect()
    })
}

fn scan_source(source: CleanSource) -> CleanSource {
    if source.id == "docker" {
        return scan_docker(source);
    }
    match measure_dir(&source.path) {
        Ok(size) => {
            let mut scanned = source;
            scanned.size_bytes = size;
            scanned.status = ScanStatus::Ok;
            scanned
        }
        Err(ScanError::NotFound) => {
            let mut scanned = source;
            scanned.status = ScanStatus::NotFound;
            scanned
        }
        Err(ScanError::PermissionDenied) => {
            let mut scanned = source;
            scanned.status = ScanStatus::Error;
            scanned.detail = Some("permission denied (EACCES)".to_string());
            scanned
        }
        Err(_) => {
            let mut scanned = source;
            scanned.status = ScanStatus::Error;
            scanned
        }
    }
}

fn scan_docker(mut source: CleanSource) -> CleanSource {
    match docker_dangling() {
        Ok((size, count)) => {
            source.size_bytes = size;
            source.status = ScanStatus::Ok;
            source.detail = Some(format!("{count} dangling images"));
        }
        Err(_) => {
            source.status = ScanStatus::Error;
            source.detail = Some("docker CLI not available".to_string());
        }
    }
    source
}

#[derive(Debug)]
enum ScanError {
    NotFound,
    PermissionDenied,
    Other,
}

impl From<io::Error> for ScanError {
    fn from(err: io::Error) -> Self {
        match err.kind() {
            io::ErrorKind::NotFound => Self::NotFound,
            io::ErrorKind::PermissionDenied => Self::PermissionDenied,
            _ => Self::Other,
        }
    }
}

fn measure_dir(root: &Path) -> Result<u64, ScanError> {
    let meta = fs::symlink_metadata(root)?;
    if meta.file_type().is_file() {
        return Ok(meta.len());
    }
    if !meta.file_type().is_dir() {
        return Err(ScanError::Other);
    }
    let entries = fs::read_dir(root)?;
    let mut total = 0;
    for entry in entries {
        let Ok(entry) = entry else { continue };
        let path = entry.path();
        let Ok(meta) = fs::symlink_metadata(&path) else {
            continue;
        };
        let ty = meta.file_type();
        if ty.is_dir() && !ty.is_symlink() {
            total += measure_dir(&path).unwrap_or(0);
        } else if ty.is_file() {
            total += meta.len();
        }
    }
    Ok(total)
}

fn direct_subdirs(path: &Path) -> Vec<PathBuf> {
    let Ok(entries) = fs::read_dir(path) else {
        return Vec::new();
    };
    entries
        .filter_map(io::Result::ok)
        .map(|e| e.path())
        .filter(|p| p.is_dir())
        .collect()
}

pub fn is_cargo_target(path: &Path) -> bool {
    path.join(".fingerprint").is_dir() || path.join("build").is_dir()
}

fn find_real_targets(projects: &Path) -> Vec<PathBuf> {
    let mut targets = Vec::new();
    for entry in direct_subdirs(projects) {
        let target = entry.join("target");
        if is_cargo_target(&target) {
            targets.push(target);
        }
    }
    targets
}

fn docker_dangling() -> Result<(u64, usize), ScanError> {
    let command = env::var("SCAN_TEST_DOCKER_CMD").unwrap_or_else(|_| "docker".to_string());
    let output = Command::new(command)
        .args([
            "images",
            "-a",
            "--filter",
            "dangling=true",
            "--format",
            "{{.Size}}",
        ])
        .output();
    let output = match output {
        Ok(s) if s.status.success() => s,
        _ => return Err(ScanError::Other),
    };
    let text = String::from_utf8_lossy(&output.stdout);
    let mut total = 0;
    let mut count = 0;
    for line in text.lines() {
        count += 1;
        total += parse_human_size(line).unwrap_or(0);
    }
    Ok((total, count))
}

fn parse_human_size(value: &str) -> Option<u64> {
    let value = value.trim();
    if value.is_empty() {
        return None;
    }
    const SUFFIXES: [(&str, u64); 5] = [
        ("KiB", 1024),
        ("KB", 1024),
        ("MiB", 1024 * 1024),
        ("MB", 1024 * 1024),
        ("GiB", 1024 * 1024 * 1024),
    ];
    for (suffix, factor) in SUFFIXES {
        if let Some(rest) = value.strip_suffix(suffix) {
            let number: f64 = rest.trim().parse().ok()?;
            return Some((number * factor as f64) as u64);
        }
    }
    if let Some(rest) = value.strip_suffix('B') {
        let number: f64 = rest.trim().parse().ok()?;
        return Some(number as u64);
    }
    value.parse::<f64>().ok().map(|n| n as u64)
}

#[cfg(test)]
mod tests {
    use std::os::unix::fs::PermissionsExt;

    use super::*;

    #[test]
    fn recognizes_real_cargo_target() {
        let dir = env::temp_dir().join("scan_target_test_s3");
        let target = dir.join("target");
        fs::create_dir_all(target.join(".fingerprint")).unwrap();
        assert!(is_cargo_target(&target));

        let unmarked = dir.join("unmarked/target");
        fs::create_dir_all(&unmarked).unwrap();
        assert!(!is_cargo_target(&unmarked));

        fs::remove_dir_all(&dir).unwrap();
    }

    #[test]
    fn parses_docker_sizes() {
        assert_eq!(parse_human_size("1.5MB"), Some(1_572_864));
        assert_eq!(parse_human_size("42B"), Some(42));
        assert_eq!(parse_human_size("1.5 MiB"), Some(1_572_864));
        assert_eq!(parse_human_size("not-a-size"), None);
    }

    #[test]
    fn eacces_does_not_panic() {
        let dir = env::temp_dir().join("scan_eacces_test");
        let blocked = dir.join("no_permission");
        fs::create_dir_all(&blocked).unwrap();
        fs::write(blocked.join("file"), b"datos").unwrap();
        fs::set_permissions(&blocked, fs::Permissions::from_mode(0o000)).unwrap();

        let source = CleanSource::new("journal", "fixture", blocked.clone());
        let result = scan_all(vec![source]);
        assert_eq!(result[0].status, ScanStatus::Error);

        fs::set_permissions(&blocked, fs::Permissions::from_mode(0o755)).unwrap();
        fs::remove_dir_all(&dir).unwrap();
    }
}
