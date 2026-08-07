mod model;
mod scan;

use model::human_size;
use scan::{candidate_sources, scan_all};

fn main() -> anyhow::Result<()> {
    let home = scan::home_dir();
    println!("Escaneando fuentes de caché...\n");

    let sources = candidate_sources(&home);
    let mut results = scan_all(sources);
    results.sort_by(|a, b| b.size_bytes.cmp(&a.size_bytes));

    println!("{:<12} {:>10}  FUENTE", "ESTADO", "TAMAÑO");
    println!("{}", "-".repeat(60));
    let mut total: u64 = 0;
    for src in &results {
        println!("{src}");
        total += src.size_bytes;
    }
    println!("{}", "-".repeat(60));
    println!("{:<12} {:>10}  TOTAL", "", human_size(total));

    Ok(())
}
