# Astro-core API Documentation

This document provides a comprehensive overview of the public API exposed by the astro-core crates.

## Table of Contents

- [astro-core (Root Crate)](#astro-core-root-crate)
- [astro-io](#astro-io)
- [astro-metadata](#astro-metadata)
- [astro-metrics](#astro-metrics)

## astro-core (Root Crate)

The root crate re-exports all functionality from the sub-crates.

### Modules

- `io`: Re-export of the astro-io crate
- `metadata`: Re-export of the astro-metadata crate
- `metrics`: Re-export of the astro-metrics crate

### Example

```rust
use astro_core::{io, metadata, metrics};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Load a FITS file
    let (pixels, width, height) = io::fits::load_fits(Path::new("/path/to/image.fits"))?;
    
    // Extract metadata
    let metadata = metadata::fits_parser::extract_metadata_from_path(Path::new("/path/to/image.fits"))?;
    
    // Analyze stars
    let (star_stats, bg_metrics) = metrics::sep_detect::detect_stars_with_sep_background(
        &pixels, width, height, Some(50)
    )?;
    
    println!("Found {} stars", star_stats.count);
    
    Ok(())
}
```

## astro-io

The astro-io crate provides functionality for loading and saving astronomical image formats.

### Modules

- `fits`: Functions for working with FITS files
- `xisf`: Functions for working with XISF files

### Public Functions

#### fits module

```rust
pub fn load_fits(path: &Path) -> Result<(Vec<f32>, usize, usize)>
```
Loads a FITS image file and returns a tuple containing:
- A vector of pixel data as 32-bit floats
- The width of the image in pixels
- The height of the image in pixels

#### xisf module

```rust
pub fn load_xisf(path: &Path) -> Result<(Vec<f32>, usize, usize)>
```
Loads an XISF image file and returns a tuple containing:
- A vector of pixel data as 32-bit floats
- The width of the image in pixels
- The height of the image in pixels

## astro-metadata

The astro-metadata crate provides functionality for extracting and handling metadata from astronomical images.

### Modules

- `types`: Type definitions for astronomical metadata
- `fits_parser`: Functions for extracting metadata from FITS files
- `xisf_parser`: Functions for extracting metadata from XISF files

### Public Types

#### AstroMetadata

The main container for all metadata extracted from an astronomical image.

```rust
pub struct AstroMetadata {
    pub equipment: Equipment,
    pub detector: Detector,
    pub filter: Filter,
    pub exposure: Exposure,
    pub mount: Option<Mount>,
    pub environment: Option<Environment>,
    pub wcs: Option<WcsData>,
    pub xisf: Option<XisfMetadata>,
    pub color_management: Option<ColorManagement>,
    pub attachments: Vec<AttachmentInfo>,
    pub raw_headers: HashMap<String, String>,
}
```

#### Equipment

Information about the telescope and other equipment.

```rust
pub struct Equipment {
    pub telescope_name: Option<String>,
    pub focal_length: Option<f32>,
    pub aperture: Option<f32>,
    pub focal_ratio: Option<f32>,
    pub reducer_flattener: Option<String>,
    pub mount_model: Option<String>,
}
```

#### Detector

Information about the camera and detector.

```rust
pub struct Detector {
    pub camera_name: Option<String>,
    pub pixel_size: Option<f32>,
    pub width: usize,
    pub height: usize,
    pub binning_x: usize,
    pub binning_y: usize,
    pub gain: Option<f32>,
    pub read_noise: Option<f32>,
    pub temperature: Option<f32>,
    pub temp_setpoint: Option<f32>,
    pub cooler_power: Option<f32>,
    pub cooler_status: Option<String>,
}
```

#### Filter

Information about the filter used.

```rust
pub struct Filter {
    pub name: Option<String>,
    pub position: Option<usize>,
    pub wavelength: Option<f32>,
}
```

#### Exposure

Information about the exposure.

```rust
pub struct Exposure {
    pub object_name: Option<String>,
    pub ra: Option<f64>,
    pub dec: Option<f64>,
    pub date_obs: Option<DateTime<Utc>>,
    pub exposure_time: Option<f32>,
    pub frame_type: Option<String>,
    pub sequence_id: Option<String>,
    pub frame_number: Option<usize>,
    pub dither_offset_x: Option<f32>,
    pub dither_offset_y: Option<f32>,
}
```

#### Mount

Information about the telescope mount.

```rust
pub struct Mount {
    pub pier_side: Option<String>,
    pub meridian_flip: Option<bool>,
    pub guide_camera: Option<String>,
    pub guide_rms: Option<f32>,
    pub guide_scale: Option<f32>,
    pub dither_enabled: Option<bool>,
}
```

#### Environment

Information about the environmental conditions.

```rust
pub struct Environment {
    pub ambient_temp: Option<f32>,
    pub humidity: Option<f32>,
    pub dew_heater_power: Option<f32>,
    pub voltage: Option<f32>,
    pub current: Option<f32>,
    pub software_version: Option<String>,
}
```

#### WcsData

World Coordinate System information.

```rust
pub struct WcsData {
    pub crpix1: Option<f64>,
    pub crpix2: Option<f64>,
    pub crval1: Option<f64>,
    pub crval2: Option<f64>,
    pub cd1_1: Option<f64>,
    pub cd1_2: Option<f64>,
    pub cd2_1: Option<f64>,
    pub cd2_2: Option<f64>,
    pub ctype1: Option<String>,
    pub ctype2: Option<String>,
}
```

#### XisfMetadata

XISF-specific metadata.

```rust
pub struct XisfMetadata {
    pub version: String,
    pub creator: Option<String>,
    pub creation_time: Option<DateTime<Utc>>,
    pub block_alignment: Option<usize>,
}
```

#### ColorManagement

Color management information.

```rust
pub struct ColorManagement {
    pub color_space: Option<String>,
    pub icc_profile: Option<Vec<u8>>,
    pub display_function: Option<DisplayFunction>,
}
```

#### DisplayFunction

Display function parameters.

```rust
pub struct DisplayFunction {
    pub function_type: Option<String>,
    pub parameters: HashMap<String, f64>,
}
```

#### AttachmentInfo

Information about an image attachment.

```rust
pub struct AttachmentInfo {
    pub id: String,
    pub geometry: String,
    pub sample_format: String,
    pub bits_per_sample: usize,
    pub compression: Option<String>,
    pub compression_parameters: HashMap<String, String>,
    pub checksum_type: Option<String>,
    pub checksum: Option<String>,
    pub resolution_x: Option<f64>,
    pub resolution_y: Option<f64>,
    pub resolution_unit: Option<String>,
}
```

### Public Methods

#### AstroMetadata

```rust
pub fn can_calculate_plate_scale(&self) -> bool
```
Checks if there is enough information to calculate the plate scale.

```rust
pub fn plate_scale(&self) -> Option<f32>
```
Calculates the plate scale in arcseconds per pixel.

### Public Functions

#### fits_parser module

```rust
pub fn extract_metadata_from_path(path: &Path) -> Result<AstroMetadata>
```
Extracts metadata from a FITS file at the specified path.

```rust
pub fn extract_metadata(fits_file: &mut FitsFile) -> Result<AstroMetadata>
```
Extracts metadata from an already open FITS file.

#### xisf_parser module

```rust
pub fn extract_metadata_from_path(path: &Path) -> Result<AstroMetadata>
```
Extracts metadata from an XISF file at the specified path.

```rust
pub fn extract_metadata<R: Read + Seek>(reader: &mut R) -> Result<AstroMetadata>
```
Extracts metadata from an already open XISF file.

## astro-metrics

The astro-metrics crate provides functionality for analyzing astronomical images and calculating quality metrics.

### Modules

- `star_metrics`: Types and functions for star measurements
- `background_metrics`: Types and functions for background measurements
- `sep_detect`: Functions for detecting stars using the Source Extractor algorithm

### Public Types

#### StarMetrics

Measurements for a single detected star.

```rust
pub struct StarMetrics {
    pub x: f64,         // x centroid
    pub y: f64,         // y centroid
    pub flux: f32,      // total flux
    pub peak: f32,      // peak value
    pub a: f32,         // semi-major axis
    pub b: f32,         // semi-minor axis
    pub theta: f32,     // position angle
    pub eccentricity: f32, // derived from a and b
    pub fwhm: f32,      // derived from a and b
}
```

#### StarStats

Aggregate statistics for a collection of stars.

```rust
pub struct StarStats {
    pub count: usize,           // total number of stars detected
    pub median_fwhm: f32,       // median FWHM across all stars
    pub median_eccentricity: f32, // median eccentricity across all stars
    pub fwhm_std_dev: f32,      // standard deviation of FWHM
    pub eccentricity_std_dev: f32, // standard deviation of eccentricity
}
```

#### BackgroundMetrics

Statistics about the image background.

```rust
pub struct BackgroundMetrics {
    pub level: f32,      // global background level
    pub rms: f32,        // global background RMS
    pub min: f32,        // minimum background value
    pub max: f32,        // maximum background value
    pub uniformity: f32, // background uniformity (0-1)
}
```

### Public Methods

#### StarMetrics

```rust
pub fn calc_fwhm(&mut self)
```
Calculates the FWHM (Full Width at Half Maximum) as the average of semi-major and semi-minor axes.

```rust
pub fn calc_eccentricity(&mut self)
```
Calculates the eccentricity from semi-major and semi-minor axes.

#### StarStats

```rust
pub fn from_stars(stars: &[StarMetrics], max_stars: Option<usize>) -> Self
```
Calculates aggregate statistics from a collection of star metrics.

### Public Functions

#### sep_detect module

```rust
pub fn detect_stars_with_sep_background(
    data: &[f32],
    width: usize,
    height: usize,
    max_stars: Option<usize]
) -> Result<(StarStats, BackgroundMetrics)>
```
Detects stars using SEP's built-in background estimation and returns both star statistics and background metrics.

```rust
pub fn detect_stars_sep(
    data: &[f32],
    width: usize,
    height: usize,
    background: f32,
    std_dev: f32,
    max_stars: Option<usize>
) -> Result<StarStats>
```
Detects stars using the SEP library with provided background and standard deviation values.