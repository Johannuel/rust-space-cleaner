use std::fmt;
use std::path::PathBuf;

/// State of a source during/after the scan.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ScanStatus {
    Scanning,
    Ok,
    Error,
    NotFound,
}

/// Broad grouping of a cleanable source, drives TUI filters and colors.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Category {
    Dev,
    Games,
    Web,
    System,
    Tools,
}

impl Category {
    #[allow(dead_code)] // consumed by the TUI (filters/colors) in v2
    pub fn label(self) -> &'static str {
        match self {
            Self::Dev => "dev",
            Self::Games => "games",
            Self::Web => "web",
            Self::System => "system",
            Self::Tools => "tools",
        }
    }
}

/// How regrettable deleting a source would be. Drives the TUI risk badge.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Risk {
    Low,
    Medium,
    High,
}

impl Risk {
    #[allow(dead_code)] // consumed by the TUI (risk badge) in v2
    pub fn label(self) -> &'static str {
        match self {
            Self::Low => "low",
            Self::Medium => "medium",
            Self::High => "high",
        }
    }
}

impl fmt::Display for ScanStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Scanning => write!(f, "scan"),
            Self::Ok => write!(f, "ok"),
            Self::Error => write!(f, "error"),
            Self::NotFound => write!(f, "not found"),
        }
    }
}

/// A cleanable source that the scan measures and the UI lists.
#[derive(Debug, Clone)]
pub struct CleanSource {
    pub id: &'static str,
    pub name: String,
    pub path: PathBuf,
    pub size_bytes: u64,
    pub status: ScanStatus,
    pub detail: Option<String>,
    pub category: Category,
    pub risk: Risk,
}

impl CleanSource {
    pub fn new(id: &'static str, name: &str, path: PathBuf) -> Self {
        Self {
            id,
            name: name.to_string(),
            path,
            size_bytes: 0,
            status: ScanStatus::Scanning,
            detail: None,
            category: Category::System,
            risk: Risk::Medium,
        }
    }

    #[allow(dead_code)] // wired by scan.rs when building sources from defs
    pub fn with_meta(mut self, category: Category, risk: Risk) -> Self {
        self.category = category;
        self.risk = risk;
        self
    }
}

/// Human-readable size: B, KB, MB, GB, TB, EB (base 1024, 1 decimal).
pub fn human_size(bytes: u64) -> String {
    if (bytes as u128) < 1024 {
        return format!("{bytes} B");
    }
    // Powers: KB=1024^1 ... EB=1024^6 (we skip PB for brevity).
    let units = [(1i32, "KB"), (2, "MB"), (3, "GB"), (4, "TB"), (6, "EB")];
    let b = bytes as f64;
    for &(power, label) in units.iter().rev() {
        let factor = 1024f64.powi(power);
        if b >= factor {
            let value = b / factor;
            let display = if value >= 100.0 {
                format!("{}", value as u64)
            } else {
                format!("{value:.1}")
            };
            return format!("{display} {label}");
        }
    }
    unreachable!("bytes >= 1024 but no unit matched")
}

impl std::fmt::Display for CleanSource {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}:{:<5} {:<12} {:>10}  {}",
            self.category.label(),
            self.risk.label(),
            self.status,
            human_size(self.size_bytes),
            self.name
        )
    }
}

#[cfg(test)]
mod tests {
    use super::{Category, Risk, human_size};

    #[test]
    fn category_labels() {
        assert_eq!(Category::Dev.label(), "dev");
        assert_eq!(Category::Games.label(), "games");
        assert_eq!(Category::Web.label(), "web");
        assert_eq!(Category::System.label(), "system");
        assert_eq!(Category::Tools.label(), "tools");
    }

    #[test]
    fn risk_labels() {
        assert_eq!(Risk::Low.label(), "low");
        assert_eq!(Risk::Medium.label(), "medium");
        assert_eq!(Risk::High.label(), "high");
    }

    #[test]
    fn human_size_formats_units() {
        assert_eq!(human_size(0), "0 B");
        assert_eq!(human_size(500), "500 B");
        assert_eq!(human_size(1024), "1.0 KB");
        assert_eq!(human_size(1536), "1.5 KB");
        assert_eq!(human_size(1024 * 1024), "1.0 MB");
        assert_eq!(human_size(5 * 1024 * 1024 * 1024), "5.0 GB");
        assert_eq!(human_size(3 * 1024 * 1024 * 1024 * 1024), "3.0 TB");
    }

    #[test]
    fn human_size_rounds_large_values() {
        assert_eq!(human_size(1200), "1.2 KB");
        assert_eq!(human_size(u64::MAX), "16.0 EB");
    }
}
