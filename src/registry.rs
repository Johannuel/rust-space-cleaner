use std::path::{Path, PathBuf};

use crate::model::{Category, Risk};

/// Declarative definition of a cleanable source.
///
/// `paths` are relative to the home directory (empty for `special` hooks that
/// compute paths at scan time, e.g. docker, journal, cargo targets).
pub struct SourceDef {
    pub id: &'static str,
    pub name: &'static str,
    pub category: Category,
    pub risk: Risk,
    pub paths: &'static [&'static str],
}

pub fn registry() -> &'static [SourceDef] {
    &[
        SourceDef {
            id: "user_cache",
            name: "user cache",
            category: Category::System,
            risk: Risk::Low,
            paths: &[],
        },
        SourceDef {
            id: "cargo_global",
            name: "cargo (global)",
            category: Category::Dev,
            risk: Risk::Low,
            paths: &[".cache/cargo"],
        },
        SourceDef {
            id: "cargo_target",
            name: "cargo targets",
            category: Category::Dev,
            risk: Risk::Medium,
            paths: &[],
        },
        SourceDef {
            id: "rustup_tmp",
            name: "rustup/tmp",
            category: Category::Tools,
            risk: Risk::Low,
            paths: &[".rustup/tmp"],
        },
        SourceDef {
            id: "npm_cache",
            name: "npm/_cacache",
            category: Category::Web,
            risk: Risk::Medium,
            paths: &[".npm/_cacache"],
        },
        SourceDef {
            id: "pnpm_cache",
            name: "pnpm",
            category: Category::Web,
            risk: Risk::Medium,
            paths: &[".cache/pnpm"],
        },
        SourceDef {
            id: "pip_cache",
            name: "pip",
            category: Category::Tools,
            risk: Risk::Low,
            paths: &[".cache/pip"],
        },
        SourceDef {
            id: "go_build",
            name: "go-build",
            category: Category::Dev,
            risk: Risk::Low,
            paths: &[".cache/go-build"],
        },
        SourceDef {
            id: "yarn",
            name: "yarn cache",
            category: Category::Web,
            risk: Risk::Medium,
            paths: &[".cache/yarn"],
        },
        SourceDef {
            id: "dotnet",
            name: "dotnet",
            category: Category::Dev,
            risk: Risk::Medium,
            paths: &[".local/share/NuGet"],
        },
        SourceDef {
            id: "nuget",
            name: "nuget packages",
            category: Category::Dev,
            risk: Risk::Medium,
            paths: &[".nuget/packages"],
        },
        SourceDef {
            id: "maven",
            name: "maven repo",
            category: Category::Dev,
            risk: Risk::Medium,
            paths: &[".m2/repository"],
        },
        SourceDef {
            id: "gradle",
            name: "gradle caches",
            category: Category::Dev,
            risk: Risk::Medium,
            paths: &[".gradle/caches", ".gradle/wrapper/dists"],
        },
        SourceDef {
            id: "conda",
            name: "conda pkgs",
            category: Category::Dev,
            risk: Risk::Medium,
            paths: &[".conda/pkgs"],
        },
        SourceDef {
            id: "steam_shaders",
            name: "steam shader cache",
            category: Category::Games,
            risk: Risk::Low,
            paths: &[".local/share/Steam/steamCache/shadercache"],
        },
        SourceDef {
            id: "proton_shaders",
            name: "proton shader cache",
            category: Category::Games,
            risk: Risk::Low,
            paths: &[".local/share/Steam/steamCache/shadercache"],
        },
        SourceDef {
            id: "lutris",
            name: "lutris cache",
            category: Category::Games,
            risk: Risk::Medium,
            paths: &[".cache/lutris"],
        },
        SourceDef {
            id: "firefox",
            name: "firefox cache",
            category: Category::Web,
            risk: Risk::Medium,
            paths: &[".cache/mozilla/firefox"],
        },
        SourceDef {
            id: "chromium",
            name: "chromium cache",
            category: Category::Web,
            risk: Risk::Medium,
            paths: &[".cache/chromium"],
        },
        SourceDef {
            id: "electron",
            name: "electron cache",
            category: Category::Web,
            risk: Risk::Medium,
            paths: &[".cache/electron"],
        },
        SourceDef {
            id: "flatpak",
            name: "flatpak cache",
            category: Category::System,
            risk: Risk::Medium,
            paths: &[".var/cache"],
        },
        SourceDef {
            id: "trash",
            name: "trash",
            category: Category::System,
            risk: Risk::High,
            paths: &[".local/share/Trash"],
        },
        SourceDef {
            id: "journal",
            name: "journal (systemd)",
            category: Category::System,
            risk: Risk::Medium,
            paths: &[],
        },
        SourceDef {
            id: "docker",
            name: "docker dangling",
            category: Category::System,
            risk: Risk::Medium,
            paths: &[],
        },
    ]
}

/// Resolves a def's paths against `home`. `~` is already expanded by callers.
pub fn expand_paths(def: &SourceDef, home: &Path) -> Vec<PathBuf> {
    def.paths.iter().map(|p| home.join(p)).collect::<Vec<_>>()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn registry_ids_are_unique() {
        let defs = registry();
        let mut ids: Vec<&str> = defs.iter().map(|d| d.id).collect();
        ids.sort_unstable();
        ids.dedup();
        assert_eq!(ids.len(), defs.len(), "duplicate ids in registry");
    }

    #[test]
    fn registry_has_24_sources() {
        assert_eq!(registry().len(), 24);
    }

    #[test]
    fn static_paths_expand_under_home() {
        let home = Path::new("/home/user");
        let defs = registry();
        let gradle = defs.iter().find(|d| d.id == "gradle").unwrap();
        assert_eq!(
            expand_paths(gradle, home),
            vec![
                PathBuf::from("/home/user/.gradle/caches"),
                PathBuf::from("/home/user/.gradle/wrapper/dists"),
            ]
        );
    }

    #[test]
    fn categories_and_risks_are_valid() {
        for d in registry() {
            assert!(!d.name.is_empty());
            assert!(matches!(
                d.category,
                Category::Dev
                    | Category::Games
                    | Category::Web
                    | Category::System
                    | Category::Tools
            ));
            assert!(matches!(d.risk, Risk::Low | Risk::Medium | Risk::High));
        }
    }
}
