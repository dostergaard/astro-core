use std::env;
use std::path::Path;
use std::process;

use astro_metadata::fits_parser::extract_metadata_from_path;

fn main() {
    // Get command line arguments
    let args: Vec<String> = env::args().collect();

    // Check if a file path was provided
    if args.len() < 2 {
        eprintln!("Usage: {} <fits_file_path>", args[0]);
        process::exit(1);
    }

    // Get the file path from arguments
    let file_path = Path::new(&args[1]);

    // Extract metadata from the FITS file
    match extract_metadata_from_path(file_path) {
        Ok(metadata) => {
            // Print formatted metadata
            println!("FITS Metadata for: {}", file_path.display());
            println!("\n=== Equipment Information ===");
            if let Some(telescope) = &metadata.equipment.telescope_name {
                println!("Telescope: {}", telescope);
            }
            if let Some(focal_length) = metadata.equipment.focal_length {
                println!("Focal Length: {} mm", focal_length);
            }
            if let Some(aperture) = metadata.equipment.aperture {
                println!("Aperture: {} mm", aperture);
            }
            if let Some(focal_ratio) = metadata.equipment.focal_ratio {
                println!("Focal Ratio: f/{:.1}", focal_ratio);
            }

            println!("\n=== Detector Information ===");
            if let Some(camera) = &metadata.detector.camera_name {
                println!("Camera: {}", camera);
            }
            println!(
                "Image Dimensions: {}x{} pixels",
                metadata.detector.width, metadata.detector.height
            );
            if metadata.detector.binning_x > 1 || metadata.detector.binning_y > 1 {
                println!(
                    "Binning: {}x{}",
                    metadata.detector.binning_x, metadata.detector.binning_y
                );
            }
            if let Some(temp) = metadata.detector.temperature {
                println!("Sensor Temperature: {:.1} °C", temp);
            }

            println!("\n=== Exposure Information ===");
            if let Some(object) = &metadata.exposure.object_name {
                println!("Object: {}", object);
            }
            if let Some(exp_time) = metadata.exposure.exposure_time {
                println!("Exposure Time: {:.2} seconds", exp_time);
            }
            if let Some(frame_type) = &metadata.exposure.frame_type {
                println!("Frame Type: {}", frame_type);
            }
            if let Some(date_obs) = metadata.exposure.date_obs {
                println!(
                    "Observation Date: {}",
                    date_obs.format("%Y-%m-%d %H:%M:%S UTC")
                );
            }
            if let (Some(ra), Some(dec)) = (metadata.exposure.ra, metadata.exposure.dec) {
                println!("Coordinates: RA={:.6}°, Dec={:.6}°", ra, dec);
            }

            println!("\n=== Filter Information ===");
            if let Some(filter) = &metadata.filter.name {
                println!("Filter: {}", filter);
            }

            // Print mount information if available
            if let Some(mount) = &metadata.mount {
                println!("\n=== Mount Information ===");
                if let Some(pier_side) = &mount.pier_side {
                    println!("Pier Side: {}", pier_side);
                }
                if let (Some(lat), Some(long)) = (mount.latitude, mount.longitude) {
                    println!("Location: Lat={:.6}°, Long={:.6}°", lat, long);
                }
                if let Some(guide_rms) = mount.guide_rms {
                    println!("Guide RMS: {:.2} pixels", guide_rms);
                }
            }

            // Print environment information if available
            if let Some(env) = &metadata.environment {
                println!("\n=== Environment Information ===");
                if let Some(temp) = env.ambient_temp {
                    println!("Ambient Temperature: {:.1} °C", temp);
                }
                if let Some(humidity) = env.humidity {
                    println!("Humidity: {:.1}%", humidity);
                }
                if let Some(sqm) = env.sqm {
                    println!("Sky Quality: {:.2} mag/arcsec²", sqm);
                }
            }

            // Print calculated values
            println!("\n=== Calculated Values ===");
            if let Some(plate_scale) = metadata.plate_scale() {
                println!("Plate Scale: {:.3} arcsec/pixel", plate_scale);
            }
            if let Some((width, height)) = metadata.field_of_view() {
                println!("Field of View: {:.2}' × {:.2}' (arcmin)", width, height);
            }

            // Print raw headers
            println!("\n=== Raw FITS Headers ===");
            for (key, value) in &metadata.raw_headers {
                println!("{} = {}", key, value);
            }
        }
        Err(err) => {
            eprintln!("Error extracting metadata: {}", err);
            process::exit(1);
        }
    }
}
