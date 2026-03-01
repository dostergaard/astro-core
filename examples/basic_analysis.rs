use astro_core::{io, metrics};
use std::path::Path;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = std::env::args().collect();
    let filepath = args.get(1).map(Path::new).unwrap_or_else(|| {
        eprintln!("Usage: cargo run --example basic_analysis /path/to/fits_file.fits");
        std::process::exit(1);
    });

    // Load the image
    println!("Loading image: {}", filepath.display());
    let (pixels, width, height) = io::fits::load_fits(filepath)?;

    println!("Image dimensions: {}x{}", width, height);

    // Basic pixel statistics
    println!("Total pixels: {}", pixels.len());
    println!(
        "Min value: {:.2}",
        pixels.iter().fold(f32::INFINITY, |a, &b| a.min(b))
    );
    println!(
        "Max value: {:.2}",
        pixels.iter().fold(f32::NEG_INFINITY, |a, &b| a.max(b))
    );

    // Detect stars using SEP
    println!("\nDetecting stars...");
    let (star_stats, bg_metrics) = metrics::sep_detect::detect_stars_with_sep_background(
        &pixels,
        width,
        height,
        Some(50), // Limit to top 50 stars
    )?;

    // Print star statistics
    println!("Found {} stars", star_stats.count);
    println!("Median FWHM: {:.2} pixels", star_stats.median_fwhm);
    println!("Median eccentricity: {:.3}", star_stats.median_eccentricity);

    // Print background metrics
    println!("\nBackground statistics:");
    println!("Background Min: {:.2}", bg_metrics.min);
    println!("Background Max: {:.2}", bg_metrics.max);
    println!("Background RMS: {:.2}", bg_metrics.rms);
    println!(
        "Background uniformity: {:.1}%",
        bg_metrics.uniformity * 100.0
    );

    Ok(())
}
