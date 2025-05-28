# Astro Frame Selector: Metadata Integration Plan

After reviewing the AstroMetadata.md document, this plan outlines the integration of metadata extraction and handling into the Astro Frame Selector project.

## 1. Data Structure Design

We should create a modular, flexible system for representing astronomical metadata. Here's a proposed structure:

```rust
// Core metadata structure with nested components
pub struct AstroMetadata {
    // Equipment information
    pub equipment: Equipment,
    // Detector and camera settings
    pub detector: Detector,
    // Filter information
    pub filter: Filter,
    // Exposure and timing information
    pub exposure: Exposure,
    // Mount and guiding information
    pub mount: Option<Mount>,
    // Environmental data
    pub environment: Option<Environment>,
    // World Coordinate System data
    pub wcs: Option<WcsData>,
    // Raw header values for any fields not explicitly parsed
    pub raw_headers: HashMap<String, String>,
}

// Equipment information
pub struct Equipment {
    pub telescope_name: Option<String>,
    pub focal_length: Option<f32>,  // mm
    pub aperture: Option<f32>,      // mm
    pub focal_ratio: Option<f32>,   // f/D
    pub reducer_flattener: Option<String>,
    pub mount_model: Option<String>,
}

// Detector and camera settings
pub struct Detector {
    pub camera_name: Option<String>,
    pub pixel_size: Option<f32>,    // μm
    pub width: usize,               // pixels
    pub height: usize,              // pixels
    pub binning_x: usize,
    pub binning_y: usize,
    pub gain: Option<f32>,          // e-/ADU
    pub read_noise: Option<f32>,    // e-
    pub full_well: Option<f32>,     // e-
    pub temperature: Option<f32>,   // °C
    pub temp_setpoint: Option<f32>, // °C
    pub cooler_power: Option<f32>,  // %
    pub cooler_status: Option<String>,
}

// Filter information
pub struct Filter {
    pub name: Option<String>,
    pub position: Option<usize>,
    pub wavelength: Option<f32>,    // nm
}

// Exposure and timing information
pub struct Exposure {
    pub object_name: Option<String>,
    pub ra: Option<f64>,            // degrees
    pub dec: Option<f64>,           // degrees
    pub date_obs: Option<DateTime<Utc>>,
    pub exposure_time: Option<f32>, // seconds
    pub frame_type: Option<String>, // "LIGHT", "DARK", "BIAS", "FLAT"
    pub sequence_id: Option<String>,
    pub frame_number: Option<usize>,
    pub dither_offset_x: Option<f32>,
    pub dither_offset_y: Option<f32>,
}

// Mount and guiding information
pub struct Mount {
    pub pier_side: Option<String>,  // "EAST", "WEST"
    pub meridian_flip: Option<bool>,
    pub guide_camera: Option<String>,
    pub guide_rms: Option<f32>,
    pub guide_scale: Option<f32>,
    pub dither_enabled: Option<bool>,
}

// Environmental data
pub struct Environment {
    pub ambient_temp: Option<f32>,  // °C
    pub humidity: Option<f32>,      // %
    pub dew_heater_power: Option<f32>, // %
    pub voltage: Option<f32>,       // V
    pub current: Option<f32>,       // A
    pub software_version: Option<String>,
    pub plugin_info: Option<String>,
}

// World Coordinate System data
pub struct WcsData {
    pub ctype1: Option<String>,
    pub ctype2: Option<String>,
    pub crpix1: Option<f64>,
    pub crpix2: Option<f64>,
    pub crval1: Option<f64>,
    pub crval2: Option<f64>,
    pub cd1_1: Option<f64>,
    pub cd1_2: Option<f64>,
    pub cd2_1: Option<f64>,
    pub cd2_2: Option<f64>,
    pub crota2: Option<f64>,
    pub airmass: Option<f32>,
    pub altitude: Option<f32>,
    pub azimuth: Option<f32>,
}
```

## 2. Project Restructuring

To accommodate this new functionality, I recommend restructuring the project as follows:

```
astro_frame_selector/
├── src/
│   ├── main.rs                  # Entry point
│   ├── lib.rs                   # Library exports
│   ├── args.rs                  # Command line arguments
│   ├── loaders/                 # File loaders
│   │   ├── mod.rs
│   │   ├── fits.rs              # FITS file loading
│   │   └── raw.rs               # RAW file loading
│   ├── metadata/                # New metadata module
│   │   ├── mod.rs
│   │   ├── types.rs             # Metadata type definitions
│   │   ├── fits_parser.rs       # FITS header parser
│   │   ├── xisf_parser.rs       # XISF header parser (future)
│   │   └── derived.rs           # Derived metrics calculation
│   ├── stats/                   # Statistics module
│   │   ├── mod.rs
│   │   ├── star_metrics.rs
│   │   ├── background_metrics.rs
│   │   └── sep_detect.rs
│   └── output/                  # Output module
│       ├── mod.rs
│       ├── csv.rs               # CSV output
│       └── preview.rs           # Image preview generation
├── benches/                     # Benchmarks
├── tests/                       # Integration tests
└── docs/                        # Documentation
```

## 3. Implementation Plan

1. **Create the metadata module structure**:
   - Define the types in `metadata/types.rs`
   - Implement parsers for FITS headers in `metadata/fits_parser.rs`
   - Add derived metrics calculation in `metadata/derived.rs`

2. **Update the loaders**:
   - Modify `loaders/fits.rs` to extract metadata while loading the image data
   - Return both image data and metadata from the loader functions

3. **Integrate with statistics**:
   - Update `stats/mod.rs` to use metadata for context-aware analysis
   - Add metadata-based quality scoring

4. **Enhance output**:
   - Update `output/csv.rs` to include metadata in the output
   - Add metadata display to image previews

## 4. Metadata Extraction Implementation

Here's a sketch of how the FITS metadata extraction might work:

```rust
// In metadata/fits_parser.rs
pub fn extract_metadata(fits_file: &mut FitsFile) -> Result<AstroMetadata> {
    let hdu = fits_file.primary_hdu()?;
    let mut metadata = AstroMetadata::default();
    let mut raw_headers = HashMap::new();
    
    // Extract all header keys and values
    for key in hdu.info.iter_keys() {
        if let Ok(value) = hdu.read_key::<String>(fits_file, &key) {
            raw_headers.insert(key.clone(), value);
        }
    }
    
    // Parse equipment information
    metadata.equipment.telescope_name = raw_headers.get("TELESCOP").cloned();
    metadata.equipment.focal_length = parse_float_header(&raw_headers, "FOCALLEN");
    // ... more parsing
    
    // Store raw headers for any fields we didn't explicitly parse
    metadata.raw_headers = raw_headers;
    
    Ok(metadata)
}

// Helper function to parse float values with error handling
fn parse_float_header(headers: &HashMap<String, String>, key: &str) -> Option<f32> {
    headers.get(key).and_then(|v| v.parse::<f32>().ok())
}
```

## 5. Integration with Existing Code

Update the `load_frame` function to return metadata:

```rust
// In loaders/fits.rs
pub fn load_fits(path: &Path) -> Result<(Vec<f32>, usize, usize, AstroMetadata)> {
    let mut file = FitsFile::open(path)?;
    let hdu = file.primary_hdu()?;
    
    // Extract dimensions
    let (width, height) = if let HduInfo::ImageInfo { shape, .. } = &hdu.info {
        let h = shape[0] as usize;
        let w = shape[1] as usize;
        (w, h)
    } else {
        bail!("Primary HDU is not an image");
    };
    
    // Read image data
    let pixels: Vec<f32> = hdu.read_image(&mut file)?;
    
    // Extract metadata
    let metadata = metadata::fits_parser::extract_metadata(&mut file)?;
    
    Ok((pixels, width, height, metadata))
}
```

## 6. Handling Missing Fields

To handle missing fields gracefully:

1. Use `Option<T>` for all metadata fields that might be missing
2. Implement default values where appropriate
3. Add helper methods to check if critical fields are present
4. Provide fallback mechanisms for derived calculations

```rust
impl AstroMetadata {
    // Check if we have enough information to calculate plate scale
    pub fn can_calculate_plate_scale(&self) -> bool {
        self.equipment.focal_length.is_some() && self.detector.pixel_size.is_some()
    }
    
    // Calculate plate scale with fallback
    pub fn plate_scale(&self) -> Option<f32> {
        if let (Some(focal_length), Some(pixel_size)) = (self.equipment.focal_length, self.detector.pixel_size) {
            // Plate scale in arcsec/pixel = (pixel size in μm / focal length in mm) * 206.265
            Some((pixel_size / focal_length) * 206.265)
        } else {
            None
        }
    }
}
```

## 7. Next Steps

1. Implement the basic metadata structures
2. Add FITS header parsing
3. Integrate with the existing image loading code
4. Update the statistics and output modules to use metadata
5. Add metadata-based quality scoring