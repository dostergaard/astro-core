# astro-metadata

Metadata handling for astronomical images.

## Overview

`astro-metadata` provides comprehensive metadata extraction and handling for astronomical images. It defines common metadata structures and parsers for different file formats.

## Features

- Unified metadata structure for astronomical images
- FITS header parsing
- XISF header parsing
- Support for equipment information (telescope, camera, etc.)
- Exposure details extraction
- Filter information
- Environmental data

## Usage

Add to your `Cargo.toml`:

```toml
[dependencies]
astro-metadata = "0.1.0"
```

### Extracting metadata from a FITS file

```rust
use astro_metadata::fits_parser::FitsMetadataParser;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let parser = FitsMetadataParser::new("/path/to/image.fits")?;
    let metadata = parser.parse()?;
    
    if let Some(object) = &metadata.object_name {
        println!("Object: {}", object);
    }
    
    if let Some(exposure) = metadata.exposure_time {
        println!("Exposure time: {} seconds", exposure);
    }
    
    Ok(())
}
```

### Extracting metadata from an XISF file

```rust
use astro_metadata::xisf_parser::XisfMetadataParser;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let parser = XisfMetadataParser::new("/path/to/image.xisf")?;
    let metadata = parser.parse()?;
    
    if let Some(object) = &metadata.object_name {
        println!("Object: {}", object);
    }
    
    if let Some(exposure) = metadata.exposure_time {
        println!("Exposure time: {} seconds", exposure);
    }
    
    Ok(())
}
```

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.