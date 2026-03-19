#[path = "shared/metadata_stats.rs"]
mod metadata_stats;

fn main() {
    if let Err(err) = metadata_stats::run_metadata_stats() {
        eprintln!("Error: {err:#}");
        std::process::exit(1);
    }
}
