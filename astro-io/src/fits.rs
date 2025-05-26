//! FITS file loader

use fitsio::FitsFile;
use anyhow::{Result, bail};
use std::path::Path;

/// Read a FITS file and return its pixel data, width, and height
pub fn load_fits(path: &Path) -> Result<(Vec<f32>, usize, usize)> {
    // Open the FITS file
    let mut file = FitsFile::open(path)?;
    // Access the primary HDU (header-data unit)
    let hdu = file.primary_hdu()?;

    // Extract the image dimensions by borrowing hdu.info
    let (width, height) = if let fitsio::hdu::HduInfo::ImageInfo { shape, .. } = &hdu.info {
        // shape is a reference to Vec<usize>
        let h = shape[0] as usize;
        let w = shape[1] as usize;
        (w, h)
    } else {
        bail!("Primary HDU is not an image");
    };

    // Read the entire image into a Vec<f32>
    let pixels: Vec<f32> = hdu.read_image(&mut file)?;
    
    Ok((pixels, width, height))
}


