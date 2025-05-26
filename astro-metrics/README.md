# astro-metrics

Statistical metrics for astronomical images.

## Overview

`astro-metrics` provides tools for analyzing astronomical images and calculating quality metrics. It helps identify the best frames for stacking and processing.

## Features

- Star detection using Source Extractor algorithm
- Star metrics calculation (count, FWHM, eccentricity)
- Background analysis (median, RMS, uniformity)
- Quality scoring for frame selection

## Usage

Add to your `Cargo.toml`:

```toml
[dependencies]
astro-metrics = "0.1.0"
```

### Analyzing star metrics

```rust
use astro_io::fits::FitsLoader;
use astro_metrics::star_metrics::StarMetrics;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Load image data
    let loader = FitsLoader::new("/path/to/image.fits")?;
    let image_data = loader.read_image_data()?;
    
    // Calculate star metrics
    let metrics = StarMetrics::new(&image_data)?;
    let stars = metrics.detect_stars()?;
    
    println!("Found {} stars", stars.len());
    println!("Average FWHM: {:.2} pixels", metrics.average_fwhm(&stars)?);
    println!("Average eccentricity: {:.3}", metrics.average_eccentricity(&stars)?);
    
    Ok(())
}
```

### Analyzing background metrics

```rust
use astro_io::fits::FitsLoader;
use astro_metrics::background_metrics::BackgroundMetrics;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Load image data
    let loader = FitsLoader::new("/path/to/image.fits")?;
    let image_data = loader.read_image_data()?;
    
    // Calculate background metrics
    let metrics = BackgroundMetrics::new(&image_data)?;
    
    println!("Background median: {:.2}", metrics.median()?);
    println!("Background RMS: {:.2}", metrics.rms()?);
    println!("Background uniformity: {:.2}%", metrics.uniformity()? * 100.0);
    
    Ok(())
}
```

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.