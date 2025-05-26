# Astro Core

A collection of Rust crates for astronomical image processing and analysis.

## Overview

Astro Core is a modular Rust library that provides components for working with astronomical images. It was extracted from the Astro Frame Selector project to create reusable components that can be used independently in various astronomical image processing applications.

## Crates

The project is organized into several specialized crates:

### astro-io

[![Crate](https://img.shields.io/crates/v/astro-io.svg)](https://crates.io/crates/astro-io)
[![Documentation](https://docs.rs/astro-io/badge.svg)](https://docs.rs/astro-io)

Handles I/O operations for astronomical image formats:
- FITS file loading and saving
- XISF file loading and saving
- Efficient image data handling

```rust
use astro_io::fits::FitsLoader;
use astro_io::xisf::XisfLoader;
```

### astro-metadata

[![Crate](https://img.shields.io/crates/v/astro-metadata.svg)](https://crates.io/crates/astro-metadata)
[![Documentation](https://docs.rs/astro-metadata/badge.svg)](https://docs.rs/astro-metadata)

Provides metadata extraction and handling for astronomical images:
- Comprehensive metadata type definitions
- FITS header parsing
- XISF header parsing
- Equipment information (telescope, camera, etc.)
- Exposure details
- Filter information
- Environmental data

```rust
use astro_metadata::AstroMetadata;
use astro_metadata::fits_parser::FitsMetadataParser;
use astro_metadata::xisf_parser::XisfMetadataParser;
```

### astro-metrics

[![Crate](https://img.shields.io/crates/v/astro-metrics.svg)](https://crates.io/crates/astro-metrics)
[![Documentation](https://docs.rs/astro-metrics/badge.svg)](https://docs.rs/astro-metrics)

Implements statistical metrics for astronomical images:
- Star detection and measurement
- Star metrics (count, FWHM, eccentricity)
- Background analysis (median, RMS, uniformity)
- Source Extractor integration via sep-sys

```rust
use astro_metrics::star_metrics::StarMetrics;
use astro_metrics::background_metrics::BackgroundMetrics;
```

## Features

- Support for FITS and XISF file formats
- Comprehensive metadata extraction
- Statistical analysis of astronomical images
- Modular design for flexible integration
- Pure Rust implementation with minimal dependencies

## Installation

Add the desired crates to your `Cargo.toml`:

```toml
[dependencies]
astro-io = "0.1.0"
astro-metadata = "0.1.0"
astro-metrics = "0.1.0"
```

Or selectively include only what you need:

```toml
[dependencies]
astro-metadata = "0.1.0"  # If you only need metadata handling
```

## Usage Examples

### Loading a FITS file and extracting metadata

```rust
use astro_io::fits::FitsLoader;
use astro_metadata::AstroMetadata;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let loader = FitsLoader::new("/path/to/image.fits")?;
    let image_data = loader.read_image_data()?;
    let metadata = loader.read_metadata()?;
    
    println!("Image dimensions: {}x{}", image_data.width(), image_data.height());
    println!("Object: {}", metadata.object_name.unwrap_or_default());
    println!("Exposure time: {} seconds", metadata.exposure_time.unwrap_or_default());
    
    Ok(())
}
```

### Analyzing star metrics in an image

```rust
use astro_io::fits::FitsLoader;
use astro_metrics::star_metrics::StarMetrics;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let loader = FitsLoader::new("/path/to/image.fits")?;
    let image_data = loader.read_image_data()?;
    
    let metrics = StarMetrics::new(&image_data)?;
    let stars = metrics.detect_stars()?;
    
    println!("Found {} stars", stars.len());
    println!("Average FWHM: {:.2} pixels", metrics.average_fwhm(&stars)?);
    println!("Average eccentricity: {:.3}", metrics.average_eccentricity(&stars)?);
    
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
cargo test --all
```

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.