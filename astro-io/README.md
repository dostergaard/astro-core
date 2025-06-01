# astro-io

I/O operations for astronomical image formats.

## Overview

`astro-io` provides functionality for loading and saving astronomical image formats, including FITS and XISF. It handles the low-level details of file I/O, image data extraction, and compression.

## Features

- FITS file loading
- XISF file loading
- Efficient image data handling
- Support for various data types (8-bit, 16-bit, 32-bit float)

## Installation

Add to your `Cargo.toml`:

```toml
[dependencies]
astro-io = "0.2.0"
```

## API Reference

### FITS Module

```rust
/// Read a FITS file and return its pixel data, width, and height
pub fn load_fits(path: &Path) -> Result<(Vec<f32>, usize, usize)>
```

- **Parameters**:
  - `path`: Path to the FITS file
- **Returns**:
  - A tuple containing:
    - `Vec<f32>`: Pixel data as a flattened vector of 32-bit floats
    - `usize`: Width of the image in pixels
    - `usize`: Height of the image in pixels
- **Errors**:
  - If the file cannot be opened
  - If the primary HDU is not an image
  - If the image data cannot be read

```rust
/// Normalize pixel values to a 0.0-1.0 range
pub fn normalize_pixels(pixels: &[f32]) -> Vec<f32>
```

- **Parameters**:
  - `pixels`: Slice of pixel values
- **Returns**:
  - `Vec<f32>`: Normalized pixel values in the range 0.0-1.0

### XISF Module

```rust
/// Read an XISF file and return its pixel data, width, and height
pub fn load_xisf(path: &Path) -> Result<(Vec<f32>, usize, usize)>
```

- **Parameters**:
  - `path`: Path to the XISF file
- **Returns**:
  - A tuple containing:
    - `Vec<f32>`: Pixel data as a flattened vector of 32-bit floats
    - `usize`: Width of the image in pixels
    - `usize`: Height of the image in pixels
- **Errors**:
  - If the file cannot be opened
  - If the XISF signature is invalid
  - If the XML header cannot be parsed
  - If the image data cannot be read

## Usage Examples

### Loading a FITS file

```rust
use astro_io::fits;
use std::path::Path;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let path = Path::new("/path/to/image.fits");
    let (pixels, width, height) = fits::load_fits(path)?;
    
    println!("Image dimensions: {}x{}", width, height);
    println!("Total pixels: {}", pixels.len());
    
    // Normalize pixel values to 0.0-1.0 range
    let normalized = fits::normalize_pixels(&pixels);
    
    Ok(())
}
```

### Loading an XISF file

```rust
use astro_io::xisf;
use std::path::Path;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let path = Path::new("/path/to/image.xisf");
    let (pixels, width, height) = xisf::load_xisf(path)?;
    
    println!("Image dimensions: {}x{}", width, height);
    println!("Total pixels: {}", pixels.len());
    
    Ok(())
}
```

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.