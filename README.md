# Astro Core

**Astro Core** is a modular collection of Rust crates for astronomical image I/O, metadata extraction, and quantitative image analysis.

Designed for astrophotography tools and observatory workflows, Astro Core provides reusable building blocks for working with FITS and XISF data in pure Rust.

---

## Status & Scope

Astro Core is actively developed and used in production within my own astrophotography tools and workflows. The APIs are designed for reuse and stability, but the ecosystem continues to evolve as new utilities and applications are built on top of it.

The goal is not just experimentation — it is to build a durable, composable foundation for astronomical image tooling in Rust.

---

## Architecture Overview

Astro Core is intentionally modular. Each crate has a focused responsibility:

* **astro-io** → image file loading and saving
* **astro-metadata** → structured metadata extraction
* **astro-metrics** → statistical and quality analysis

These crates are designed to be used independently or together, depending on your application’s needs.

---

## Crates

### astro-io

Handles I/O operations for astronomical image formats:

* FITS file loading and saving
* XISF file loading and saving
* Efficient image data handling

```rust
use astro_io::fits;
use astro_io::xisf;

let (pixels, width, height) = fits::load_fits(Path::new("/path/to/image.fits"))?;
```

---

### astro-metadata

Provides structured metadata extraction and handling:

* FITS header parsing
* XISF header parsing
* Equipment information (telescope, camera, filters)
* Exposure details
* Environmental data
* Plate scale calculations

```rust
use astro_metadata::fits_parser;

let metadata = fits_parser::extract_metadata_from_path(Path::new("/path/to/image.fits"))?;

if let Some(plate_scale) = metadata.plate_scale() {
    println!("Plate scale: {} arcsec/pixel", plate_scale);
}
```

---

### astro-metrics

Implements statistical and quality metrics for astronomical images:

* Star detection and measurement (via SEP)
* Star metrics (count, FWHM, eccentricity, elongation)
* Background analysis (median, RMS, uniformity)
* Composite quality scoring

```rust
use astro_metrics::sep_detect;
use astro_metrics::quality_metrics;

let (star_stats, background) =
    sep_detect::detect_stars_with_sep_background(&image_data, width, height, None)?;

let scores = quality_metrics::calculate_quality_scores(&star_stats, &background);
println!("Overall quality score: {}", scores.overall);
```

---

## Design Goals

* Pure Rust implementation
* Minimal external dependencies
* Clear crate boundaries
* Composable APIs
* Suitable for CLI tools, services, or GUI applications
* Deterministic, testable image quality metrics

---

## Installation

Add only the crates you need:

```toml
[dependencies]
astro-io = "0.2.0"
astro-metadata = "0.2.0"
astro-metrics = "0.2.0"
```

Each crate can be used independently.

---

## Example: Load, Extract Metadata, and Score an Image

```rust
use astro_io::fits;
use astro_metadata::fits_parser;
use astro_metrics::sep_detect;
use astro_metrics::quality_metrics;

let path = Path::new("/path/to/image.fits");

let (image_data, width, height) = fits::load_fits(path)?;
let metadata = fits_parser::extract_metadata_from_path(path)?;

let (star_stats, background) =
    sep_detect::detect_stars_with_sep_background(&image_data, width, height, None)?;

let scores = quality_metrics::calculate_quality_scores(&star_stats, &background);

println!("Object: {:?}", metadata.exposure.object_name);
println!("Stars detected: {}", star_stats.count);
println!("Quality score: {:.3}", scores.overall);
```

---

## Development

Clone and build:

```bash
git clone https://github.com/dostergaard/astro-core.git
cd astro-core
cargo build --release
```

Run tests:

```bash
cargo test --workspace
```

---

## Documentation

Additional documentation is available in the `docs/` directory:

* Quality Metrics documentation
* Supported NINA tokens
* File organization tokens

---

## Contributing

Contributions, suggestions, and issue reports are welcome.

Astro Core is part of a broader effort to build open, composable tools for astrophotography workflows in Rust.

---

## Support

If Astro Core is useful in your projects, consider supporting development via GitHub Sponsors or other contribution channels.

---

## License

MIT License

---

# What changed (strategically)

1. Removed “crate not yet published” language — that reads temporary and weak.
2. Strengthened positioning: this is infrastructure.
3. Clarified architecture boundaries (helps AI reasoning).
4. Removed some redundant example overlap.
5. Tightened tone to professional open-source.
