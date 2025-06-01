# Astro Core

A collection of Rust crates for astronomical image processing and analysis.

## Notice

Astro Core and its associated crates (astro-io, astro-metadata, and astro-metrics) are being developed to support my astrophotography image processing workflows. These libraries are actively evolving as they're integrated into various utilities I'm building for astronomical image analysis and processing. While they're designed to be stable and reusable, expect ongoing changes and enhancements as development of dependent projects progresses.

## Overview

Astro Core is a modular Rust library that provides components for working with astronomical images. It was extracted from my Astro Frame Selector project to create reusable components that can be used independently in various astronomical image processing applications.

## Crates

The project is organized into several specialized crates:

### astro-io

*Crate not yet published*

Handles I/O operations for astronomical image formats:
- FITS file loading and saving
- XISF file loading and saving
- Efficient image data handling

```rust
use astro_io::fits;
use astro_io::xisf;

// Load a FITS file
let (pixels, width, height) = fits::load_fits(Path::new("/path/to/image.fits"))?;
```

### astro-metadata

*Crate not yet published*

Provides metadata extraction and handling for astronomical images:
- Comprehensive metadata type definitions
- FITS header parsing
- XISF header parsing
- Equipment information (telescope, camera, etc.)
- Exposure details
- Filter information
- Environmental data

```rust
use astro_metadata::fits_parser;
use std::path::Path;

// Extract metadata from a FITS file
let metadata = fits_parser::extract_metadata_from_path(Path::new("/path/to/image.fits"))?;

// Calculate plate scale
if let Some(plate_scale) = metadata.plate_scale() {
    println!("Plate scale: {} arcsec/pixel", plate_scale);
}
```

### astro-metrics

*Crate not yet published*

Implements statistical metrics for astronomical images:
- Star detection and measurement using Source Extractor (SEP)
- Star metrics (count, FWHM, eccentricity, elongation)
- Background analysis (median, RMS, uniformity)
- Quality scoring for image comparison

```rust
use astro_metrics::sep_detect;
use astro_metrics::quality_metrics;

// Detect stars and analyze background
let (star_stats, background) = sep_detect::detect_stars_with_sep_background(
    &image_data, width, height, None)?;

// Calculate quality scores
let scores = quality_metrics::calculate_quality_scores(&star_stats, &background);
println!("Overall quality score: {}", scores.overall);
```

## Features

- Support for FITS and XISF file formats
- Comprehensive metadata extraction
- Statistical analysis of astronomical images
- Quality metrics for image comparison and selection
- Modular design for flexible integration
- Pure Rust implementation with minimal dependencies

## Installation

Add the desired crates to your `Cargo.toml`:

```toml
[dependencies]
astro-io = "0.2.0"
astro-metadata = "0.2.0"
astro-metrics = "0.2.0"
```

Or selectively include only what you need:

```toml
[dependencies]
astro-metadata = "0.2.0"  # If you only need metadata handling
```

## Usage Examples

### Loading a FITS file and extracting metadata

```rust
use astro_core::io;
use astro_core::metadata;
use std::path::Path;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let path = Path::new("/path/to/image.fits");
    
    // Load image data
    let (image_data, width, height) = io::fits::load_fits(path)?;
    
    // Extract metadata
    let metadata = metadata::fits_parser::extract_metadata_from_path(path)?;
    
    println!("Image dimensions: {}x{}", width, height);
    println!("Object: {}", metadata.exposure.object_name.unwrap_or_default());
    println!("Exposure time: {} seconds", metadata.exposure.exposure_time.unwrap_or_default());
    
    Ok(())
}
```

### Analyzing star metrics and image quality

```rust
use astro_core::io;
use astro_core::metrics;
use std::path::Path;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let path = Path::new("/path/to/image.fits");
    
    // Load image data
    let (image_data, width, height) = io::fits::load_fits(path)?;
    
    // Detect stars and analyze background
    let (star_stats, background) = metrics::sep_detect::detect_stars_with_sep_background(
        &image_data, width, height, None)?;
    
    // Calculate quality scores
    let scores = metrics::quality_metrics::calculate_quality_scores(&star_stats, &background);
    
    println!("Found {} stars", star_stats.count);
    println!("Median FWHM: {:.2} pixels", star_stats.median_fwhm);
    println!("Background uniformity: {:.3}", background.uniformity);
    println!("Overall quality score: {:.3}", scores.overall);
    
    Ok(())
}
```

## Development

### Building from Source

```bash
git clone https://github.com/dostergaard/astro-core.git
cd astro-core
cargo build --release
```

### Running Tests

```bash
cargo test --workspace
```

## Documentation

Additional documentation is available in the docs directory:
- [Quality Metrics Documentation](docs/QualityMetrics.md) - Details on image quality scoring
- [NINA Token Support](docs/Supported_NINA_Tokens.md) - Information on supported NINA tokens
- [File Organization Tokens](docs/FileOrgTokens.md) - Tokens for file organization

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.