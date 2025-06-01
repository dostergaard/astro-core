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

/// Normalize pixel values to a 0.0-1.0 range
pub fn normalize_pixels(pixels: &[f32]) -> Vec<f32> {
    if pixels.is_empty() {
        return Vec::new();
    }
    
    // Find min and max values
    let mut min_val = pixels[0];
    let mut max_val = pixels[0];
    
    for &pixel in pixels {
        min_val = min_val.min(pixel);
        max_val = max_val.max(pixel);
    }
    
    // Avoid division by zero
    let range = max_val - min_val;
    if range == 0.0 {
        return vec![0.0; pixels.len()];
    }
    
    // Normalize each pixel
    pixels.iter().map(|&p| (p - min_val) / range).collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_normalize_pixels() {
        // Test with normal range
        let pixels = vec![100.0, 200.0, 300.0, 400.0, 500.0];
        let normalized = normalize_pixels(&pixels);
        
        assert_eq!(normalized.len(), 5);
        assert_eq!(normalized[0], 0.0);
        assert_eq!(normalized[4], 1.0);
        assert!((normalized[2] - 0.5).abs() < 0.001);
        
        // Test with empty input
        let empty: Vec<f32> = Vec::new();
        let result = normalize_pixels(&empty);
        assert_eq!(result.len(), 0);
        
        // Test with single value (avoid division by zero)
        let single = vec![42.0];
        let result = normalize_pixels(&single);
        assert_eq!(result, vec![0.0]);
        
        // Test with all same values
        let same = vec![10.0, 10.0, 10.0];
        let result = normalize_pixels(&same);
        assert_eq!(result, vec![0.0, 0.0, 0.0]);
    }
    
    #[test]
    fn test_extract_dimensions() {
        // This is a mock test since we can't easily create a FitsFile for testing
        // In a real test, we would use a test FITS file
        
        // Test the dimension extraction logic
        let shape = vec![1024, 768];
        let (width, height) = extract_dimensions_from_shape(&shape);
        
        assert_eq!(width, 768);
        assert_eq!(height, 1024);
        
        // Test with different dimensions
        let shape = vec![2048, 1536];
        let (width, height) = extract_dimensions_from_shape(&shape);
        
        assert_eq!(width, 1536);
        assert_eq!(height, 2048);
    }
    
    // Helper function to test dimension extraction logic
    fn extract_dimensions_from_shape(shape: &[usize]) -> (usize, usize) {
        let h = shape[0];
        let w = shape[1];
        (w, h)
    }
}


