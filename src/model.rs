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
        }
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
            "{:<12} {:>10}  {}",
            self.status,
            human_size(self.size_bytes),
            self.name
        )
    }
}

#[cfg(test)]
mod tests {
    use super::human_size;

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
