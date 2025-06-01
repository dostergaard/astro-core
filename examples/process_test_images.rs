use anyhow::Result;
use astro_io::{fits, xisf};
use astro_metadata::{fits_parser, xisf_parser};
use astro_metrics::sep_detect;
use std::path::Path;
use std::time::Instant;
use std::fs;

fn main() -> Result<()> {
    // Get all files in the tests/data directory
    let test_dir = Path::new("tests/data");
    let entries = fs::read_dir(test_dir)?;
    
    let mut fits_files = Vec::new();
    let mut xisf_files = Vec::new();
    
    // Categorize files by extension
    for entry in entries {
        if let Ok(entry) = entry {
            let path = entry.path();
            if let Some(ext) = path.extension() {
                let ext_str = ext.to_string_lossy().to_lowercase();
                if ext_str == "fits" || ext_str == "fit" {
                    fits_files.push(path);
                } else if ext_str == "xisf" {
                    xisf_files.push(path);
                }
            }
        }
    }
    
    // Process FITS files
    for (i, path) in fits_files.iter().enumerate() {
        println!("\n=== Processing FITS file {}: {} ===", i+1, path.display());
        if let Err(e) = process_fits_file(path) {
            println!("Error processing file: {}", e);
        }
    }
    
    // Process XISF files
    for (i, path) in xisf_files.iter().enumerate() {
        println!("\n=== Processing XISF file {}: {} ===", i+1, path.display());
        if let Err(e) = process_xisf_file(path) {
            println!("Error processing file: {}", e);
        }
    }
    
    Ok(())
}

fn process_fits_file(path: &Path) -> Result<()> {
    // Extract metadata
    let start = Instant::now();
    let metadata = fits_parser::extract_metadata_from_path(path)?;
    let metadata_time = start.elapsed();
    
    // Print basic metadata
    println!("Metadata extraction time: {:?}", metadata_time);
    println!("Object: {}", metadata.exposure.object_name.as_ref().unwrap_or(&String::new()));
    println!("Exposure time: {} seconds", metadata.exposure.exposure_time.unwrap_or_default());
    println!("Camera: {}", metadata.detector.camera_name.as_ref().unwrap_or(&String::new()));
    println!("Dimensions: {}x{}", metadata.detector.width, metadata.detector.height);
    
    if let Some(plate_scale) = metadata.plate_scale() {
        println!("Plate scale: {:.3} arcsec/pixel", plate_scale);
    }
    
    if let Some((width, height)) = metadata.field_of_view() {
        println!("Field of view: {:.2}' × {:.2}'", width, height);
    }
    
    // Load image data
    let start = Instant::now();
    let (pixels, width, height) = fits::load_fits(path)?;
    let load_time = start.elapsed();
    println!("Image loading time: {:?}", load_time);
    println!("Image dimensions: {}x{}", width, height);
    
    // Check for NaN or Inf values
    let has_nan = pixels.iter().any(|&x| x.is_nan());
    let has_inf = pixels.iter().any(|&x| x.is_infinite());
    println!("Image contains NaN values: {}", has_nan);
    println!("Image contains Inf values: {}", has_inf);
    
    // Print some pixel statistics
    if !pixels.is_empty() {
        let mut min_val = pixels[0];
        let mut max_val = pixels[0];
        let mut sum = 0.0;
        
        for &p in &pixels {
            if !p.is_nan() && !p.is_infinite() {
                min_val = min_val.min(p);
                max_val = max_val.max(p);
                sum += p;
            }
        }
        
        let mean = sum / pixels.len() as f32;
        println!("Pixel stats - Min: {}, Max: {}, Mean: {}", min_val, max_val, mean);
    }
    
    // Detect stars - skip if image has NaN or Inf values
    if !has_nan && !has_inf {
        println!("Starting star detection...");
        let start = Instant::now();
        match sep_detect::detect_stars_with_sep_background(&pixels, width, height, None) {
            Ok((star_stats, background)) => {
                let detect_time = start.elapsed();
                println!("Star detection time: {:?}", detect_time);
                println!("Stars detected: {}", star_stats.count);
                println!("Median FWHM: {:.2} pixels", star_stats.median_fwhm);
                println!("Median eccentricity: {:.3}", star_stats.median_eccentricity);
                println!("Median elongation: {:.3}", star_stats.median_elongation);
                println!("Median SNR: {:.1}", star_stats.median_snr);
                println!("Background level: {:.1}", background.median);
                println!("Background RMS: {:.3}", background.rms);
                println!("Background uniformity: {:.3}", background.uniformity);
            },
            Err(e) => {
                println!("Error detecting stars: {}", e);
            }
        }
    } else {
        println!("Skipping star detection due to NaN or Inf values in the image");
    }
    
    Ok(())
}

fn process_xisf_file(path: &Path) -> Result<()> {
    // Extract metadata
    let start = Instant::now();
    let metadata = xisf_parser::extract_metadata_from_path(path)?;
    let metadata_time = start.elapsed();
    
    // Print basic metadata
    println!("Metadata extraction time: {:?}", metadata_time);
    println!("Object: {}", metadata.exposure.object_name.as_ref().unwrap_or(&String::new()));
    println!("Exposure time: {} seconds", metadata.exposure.exposure_time.unwrap_or_default());
    println!("Camera: {}", metadata.detector.camera_name.as_ref().unwrap_or(&String::new()));
    println!("Dimensions: {}x{}", metadata.detector.width, metadata.detector.height);
    
    if let Some(plate_scale) = metadata.plate_scale() {
        println!("Plate scale: {:.3} arcsec/pixel", plate_scale);
    }
    
    if let Some((width, height)) = metadata.field_of_view() {
        println!("Field of view: {:.2}' × {:.2}'", width, height);
    }
    
    // Load image data
    let start = Instant::now();
    let (pixels, width, height) = xisf::load_xisf(path)?;
    let load_time = start.elapsed();
    println!("Image loading time: {:?}", load_time);
    println!("Image dimensions: {}x{}", width, height);
    
    // Check for NaN or Inf values
    let has_nan = pixels.iter().any(|&x| x.is_nan());
    let has_inf = pixels.iter().any(|&x| x.is_infinite());
    println!("Image contains NaN values: {}", has_nan);
    println!("Image contains Inf values: {}", has_inf);
    
    // Print some pixel statistics
    if !pixels.is_empty() {
        let mut min_val = pixels[0];
        let mut max_val = pixels[0];
        let mut sum = 0.0;
        
        for &p in &pixels {
            if !p.is_nan() && !p.is_infinite() {
                min_val = min_val.min(p);
                max_val = max_val.max(p);
                sum += p;
            }
        }
        
        let mean = sum / pixels.len() as f32;
        println!("Pixel stats - Min: {}, Max: {}, Mean: {}", min_val, max_val, mean);
    }
    
    // Detect stars - skip if image has NaN or Inf values
    if !has_nan && !has_inf {
        println!("Starting star detection...");
        let start = Instant::now();
        match sep_detect::detect_stars_with_sep_background(&pixels, width, height, None) {
            Ok((star_stats, background)) => {
                let detect_time = start.elapsed();
                println!("Star detection time: {:?}", detect_time);
                println!("Stars detected: {}", star_stats.count);
                println!("Median FWHM: {:.2} pixels", star_stats.median_fwhm);
                println!("Median eccentricity: {:.3}", star_stats.median_eccentricity);
                println!("Median elongation: {:.3}", star_stats.median_elongation);
                println!("Median SNR: {:.1}", star_stats.median_snr);
                println!("Background level: {:.1}", background.median);
                println!("Background RMS: {:.3}", background.rms);
                println!("Background uniformity: {:.3}", background.uniformity);
            },
            Err(e) => {
                println!("Error detecting stars: {}", e);
            }
        }
    } else {
        println!("Skipping star detection due to NaN or Inf values in the image");
    }
    
    Ok(())
}