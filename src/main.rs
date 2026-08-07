mod clean;
mod model;
mod registry;
mod scan;
mod ui;

use std::path::PathBuf;

use anyhow::Context;

use model::CleanSource;
use scan::{candidate_sources, scan_all};
use ui::{App, SourceProvider};

struct RealProvider {
    home: PathBuf,
    rows: Vec<CleanSource>,
}

impl RealProvider {
    fn new() -> Self {
        Self {
            home: scan::home_dir(),
            rows: Vec::new(),
        }
    }
}

impl SourceProvider for RealProvider {
    fn sources(&self) -> Vec<CleanSource> {
        self.rows.clone()
    }

    fn rescan(&mut self) {
        let sources = candidate_sources(&self.home);
        self.rows = scan_all(sources);
    }

    fn clean(&mut self, source: &CleanSource) -> Result<(), String> {
        let whitelist: Vec<PathBuf> = {
            let mut paths: Vec<PathBuf> = registry::registry()
                .iter()
                .flat_map(|def| registry::expand_paths(def, &self.home))
                .collect();
            paths.push(self.home.join(".cache"));
            paths
        };
        let whitelist_refs: Vec<&std::path::Path> = whitelist.iter().map(|p| p.as_path()).collect();

        if !clean::is_safe_to_clean(&source.path, &whitelist_refs) {
            return Err(format!("path not allowed: {}", source.path.display()));
        }
        let path = source.path.clone();
        std::fs::remove_dir_all(&path)
            .with_context(|| format!("removing {}", path.display()))
            .map_err(|e| e.to_string())?;
        log_clean(&path, source.size_bytes, &self.home)?;
        Ok(())
    }
}

fn log_clean(path: &std::path::Path, bytes: u64, home: &std::path::Path) -> Result<(), String> {
    use std::fs::OpenOptions;
    use std::io::Write;
    use std::time::{SystemTime, UNIX_EPOCH};

    let log_dir = home.join(".local/share/rust-space-cleaner");
    std::fs::create_dir_all(&log_dir).map_err(|e| e.to_string())?;

    let ts = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|d| d.as_secs())
        .unwrap_or(0);
    let line = format!("{ts}\t{bytes}\t{}\n", path.display());

    let mut file = OpenOptions::new()
        .create(true)
        .append(true)
        .open(log_dir.join("clean.log"))
        .map_err(|e| e.to_string())?;
    file.write_all(line.as_bytes()).map_err(|e| e.to_string())
}

fn main() -> anyhow::Result<()> {
    let mut provider = RealProvider::new();
    provider.rescan();

    let app = App::new(provider.sources());
    app.run(&mut provider)?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::log_clean;
    use std::path::PathBuf;

    #[test]
    fn log_writes_timestamp_path_and_size() {
        let tmp = tempfile::tempdir().unwrap();
        log_clean(&PathBuf::from("/home/user/.cache/alpha"), 1234, tmp.path()).unwrap();

        let log = tmp.path().join(".local/share/rust-space-cleaner/clean.log");
        let content = std::fs::read_to_string(&log).unwrap();
        assert!(content.contains("\t1234\t"));
        assert!(content.contains(".cache/alpha"));
        assert!(content.lines().count() == 1);
    }
}
