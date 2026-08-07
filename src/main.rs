mod clean;
mod model;
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
        let whitelist: Vec<PathBuf> = [
            self.home.join(".cache"),
            self.home.join(".cargo"),
            self.home.join(".rustup/tmp"),
            self.home.join(".npm/_cacache"),
            self.home.join(".cache/pip"),
        ]
        .into_iter()
        .collect();
        let whitelist_refs: Vec<&std::path::Path> = whitelist.iter().map(|p| p.as_path()).collect();

        if !clean::is_safe_to_clean(&source.path, &whitelist_refs) {
            return Err(format!("ruta no permitida: {}", source.path.display()));
        }
        std::fs::remove_dir_all(&source.path)
            .with_context(|| format!("borrando {}", source.path.display()))
            .map_err(|e| e.to_string())
    }
}

fn main() -> anyhow::Result<()> {
    let mut provider = RealProvider::new();
    provider.rescan();

    let app = App::new(provider.sources());
    app.run(&mut provider)?;
    Ok(())
}
