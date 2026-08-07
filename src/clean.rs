use std::ffi::OsString;
use std::path::{Component, Path};

/// Normalize an absolute path to its components, resolving `.` and `..`.
/// Returns `None` if the path is not absolute or uses an exotic prefix.
fn absolute_components(path: &Path) -> Option<Vec<OsString>> {
    if !path.is_absolute() {
        return None;
    }

    let mut parts: Vec<OsString> = Vec::new();
    for component in path.components() {
        match component {
            Component::RootDir | Component::CurDir => {}
            Component::ParentDir => {
                parts.pop();
            }
            Component::Normal(name) => parts.push(name.to_os_string()),
            Component::Prefix(_) => return None,
        }
    }
    Some(parts)
}

/// Is `candidate` a strict subdirectory of `root` (root + at least 1 level)?
fn is_strict_prefix(root: &[OsString], candidate: &[OsString]) -> bool {
    root.len() < candidate.len() && candidate.starts_with(root)
}

/// Is `entry` a whitelist container (i.e. the whitelist has sub-entries)?
/// A container is scanned by listing its direct children, but the folder
/// itself is never cleaned as a whole (e.g. `~/.cache`).
fn is_considered_container(entry: &[OsString], whitelist: &[&Path]) -> bool {
    whitelist
        .iter()
        .filter_map(|w| absolute_components(w))
        .any(|other| is_strict_prefix(entry, &other))
}

/// Indicate whether `path` can be safely removed according to the `whitelist`.
///
/// Rules:
/// - An exact whitelist entry is approved.
/// - A direct (1 level) subdirectory of a considered whitelist container is
///   approved (e.g. `~/.cache/*`).
/// - The bare container (`~/.cache`), `$HOME`, the root `/`, false prefixes
///   (`~/.cargo_evil`) and paths outside the whitelist are rejected.
///
/// Paths arrive absolute and with `~` already expanded.
pub fn is_safe_to_clean(path: &Path, whitelist: &[&Path]) -> bool {
    let candidate = match absolute_components(path) {
        Some(c) if c.len() > 1 => c,
        _ => return false,
    };

    // An ancestor of a whitelist entry is never removable ($HOME, /).
    if whitelist
        .iter()
        .filter_map(|w| absolute_components(w))
        .any(|entry| is_strict_prefix(&candidate, &entry))
    {
        return false;
    }

    for w in whitelist {
        let Some(entry) = absolute_components(w) else {
            continue;
        };
        if is_considered_container(&entry, whitelist) {
            if is_strict_prefix(&entry, &candidate) && candidate.len() == entry.len() + 1 {
                return true;
            }
        } else if candidate == entry {
            return true;
        }
    }

    false
}
