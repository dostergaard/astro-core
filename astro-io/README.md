# astro-io

I/O operations for astronomical image formats.

## Overview

`astro-io` provides functionality for loading and saving astronomical image formats, including FITS and XISF. It handles the low-level details of file I/O, image data extraction, and compression.

## Features

- FITS file loading and saving
- XISF file loading and saving
- Efficient image data handling
- Support for various data types (8-bit, 16-bit, 32-bit float)
- Compression/decompression support

## Usage

Add to your `Cargo.toml`:

```toml
[dependencies]
astro-io = "0.1.0"
```

### Loading a FITS file

```rust
use astro_io::fits::FitsLoader;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let loader = FitsLoader::new("/path/to/image.fits")?;
    let image_data = loader.read_image_data()?;
    
    println!("Image dimensions: {}x{}", image_data.width(), image_data.height());
    
    Ok(())
}
```

### Loading an XISF file

```rust
use astro_io::xisf::XisfLoader;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let loader = XisfLoader::new("/path/to/image.xisf")?;
    let image_data = loader.read_image_data()?;
    
    println!("Image dimensions: {}x{}", image_data.width(), image_data.height());
    
    Ok(())
}
```

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.