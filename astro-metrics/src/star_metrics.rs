//! Star measurement metrics and calculations

use std::ffi::c_int;
use anyhow::{Result, anyhow};

/// Measurements for a single detected star
#[derive(Debug, Clone)]
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

impl StarMetrics {
    /// Calculate FWHM as average of semi-major and semi-minor axes
    pub fn calc_fwhm(&mut self) {
        self.fwhm = (self.a + self.b) / 2.0;
    }

    /// Calculate eccentricity from semi-major and semi-minor axes
    pub fn calc_eccentricity(&mut self) {
        if self.a == 0.0 {
            self.eccentricity = 0.0;
        } else {
            self.eccentricity = (1.0 - (self.b * self.b) / (self.a * self.a)).sqrt();
        }
    }
}

/// Aggregate statistics for a collection of stars
#[derive(Debug)]
pub struct StarStats {
    pub count: usize,           // total number of stars detected
    pub median_fwhm: f32,       // median FWHM across all stars
    pub median_eccentricity: f32, // median eccentricity across all stars
    pub fwhm_std_dev: f32,      // standard deviation of FWHM
    pub eccentricity_std_dev: f32, // standard deviation of eccentricity
}

impl StarStats {
    /// Calculate aggregate statistics from a collection of star metrics
    pub fn from_stars(stars: &[StarMetrics], max_stars: Option<usize>) -> Self {
        // Sort stars by flux and take the top N if max_stars is specified
        let mut sorted_stars = stars.to_vec();
        sorted_stars.sort_by(|a, b| b.flux.partial_cmp(&a.flux).unwrap());
        let stars_to_use = if let Some(max) = max_stars {
            &sorted_stars[..max.min(sorted_stars.len())]
        } else {
            &sorted_stars
        };

        // Calculate medians
        let mut fwhm_values: Vec<f32> = stars_to_use.iter().map(|s| s.fwhm).collect();
        let mut ecc_values: Vec<f32> = stars_to_use.iter().map(|s| s.eccentricity).collect();
        
        fwhm_values.sort_by(|a, b| a.partial_cmp(b).unwrap());
        ecc_values.sort_by(|a, b| a.partial_cmp(b).unwrap());
        
        let median_fwhm = if !fwhm_values.is_empty() {
            fwhm_values[fwhm_values.len() / 2]
        } else {
            0.0
        };

        let median_eccentricity = if !ecc_values.is_empty() {
            ecc_values[ecc_values.len() / 2]
        } else {
            0.0
        };

        // Calculate standard deviations
        let fwhm_std_dev = calculate_std_dev(&fwhm_values);
        let eccentricity_std_dev = calculate_std_dev(&ecc_values);

        StarStats {
            count: stars.len(),
            median_fwhm,
            median_eccentricity,
            fwhm_std_dev,
            eccentricity_std_dev,
        }
    }
}

/// Calculate standard deviation of a slice of f32 values
fn calculate_std_dev(values: &[f32]) -> f32 {
    if values.is_empty() {
        return 0.0;
    }

    let mean = values.iter().sum::<f32>() / values.len() as f32;
    let variance = values.iter()
        .map(|&x| {
            let diff = x - mean;
            diff * diff
        })
        .sum::<f32>() / values.len() as f32;
    
    variance.sqrt()
}