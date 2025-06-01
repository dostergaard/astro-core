//! Star measurement metrics and calculations

use crate::types::{StarMetrics, StarStats};

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

impl StarStats {
    /// Calculate aggregate statistics from a collection of star metrics
    pub fn from_stars(stars: &[StarMetrics], max_stars: Option<usize>) -> Self {
        // Handle empty star list
        if stars.is_empty() {
            return StarStats {
                count: 0,
                median_fwhm: 0.0,
                median_eccentricity: 0.0,
                fwhm_std_dev: 0.0,
                eccentricity_std_dev: 0.0,
                median_kron_radius: 0.0,
                median_flux: 0.0,
                median_snr: 0.0,
                median_elongation: 0.0,
                flagged_fraction: 0.0,
                kron_radius_std_dev: 0.0,
                flux_std_dev: 0.0,
                snr_std_dev: 0.0,
            };
        }
        
        // Sort stars by flux and take the top N if max_stars is specified
        let mut sorted_stars = stars.to_vec();
        // Sort by flux, handling NaN values
        sorted_stars.sort_by(|a, b| {
            if a.flux.is_nan() && b.flux.is_nan() {
                std::cmp::Ordering::Equal
            } else if a.flux.is_nan() {
                std::cmp::Ordering::Less
            } else if b.flux.is_nan() {
                std::cmp::Ordering::Greater
            } else {
                b.flux.partial_cmp(&a.flux).unwrap_or(std::cmp::Ordering::Equal)
            }
        });
        let stars_to_use = if let Some(max) = max_stars {
            &sorted_stars[..max.min(sorted_stars.len())]
        } else {
            &sorted_stars
        };

        // Calculate medians for basic metrics
        let mut fwhm_values: Vec<f32> = stars_to_use.iter().map(|s| s.fwhm).collect();
        let mut ecc_values: Vec<f32> = stars_to_use.iter().map(|s| s.eccentricity).collect();
        
        // Sort values, handling NaN values
        fwhm_values.sort_by(|a, b| {
            if a.is_nan() && b.is_nan() {
                std::cmp::Ordering::Equal
            } else if a.is_nan() {
                std::cmp::Ordering::Greater
            } else if b.is_nan() {
                std::cmp::Ordering::Less
            } else {
                a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal)
            }
        });
        
        ecc_values.sort_by(|a, b| {
            if a.is_nan() && b.is_nan() {
                std::cmp::Ordering::Equal
            } else if a.is_nan() {
                std::cmp::Ordering::Greater
            } else if b.is_nan() {
                std::cmp::Ordering::Less
            } else {
                a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal)
            }
        });
        
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

        // Calculate standard deviations for basic metrics
        let fwhm_std_dev = calculate_std_dev(&fwhm_values);
        let eccentricity_std_dev = calculate_std_dev(&ecc_values);

        // Calculate medians for additional metrics
        let mut kron_values: Vec<f32> = stars_to_use.iter().map(|s| s.kron_radius).collect();
        let mut flux_values: Vec<f32> = stars_to_use.iter().map(|s| s.flux_auto).collect();
        // Calculate SNR values - use AUTO flux and error when available
        let mut snr_values: Vec<f32> = stars_to_use.iter()
            .map(|s| {
                if s.fluxerr_auto > 0.0 {
                    // Use AUTO flux and its error for SNR calculation
                    s.flux_auto / s.fluxerr_auto
                } else if s.flux > 0.0 {
                    // Fallback: estimate SNR using sqrt(flux) as error approximation
                    s.flux / s.flux.sqrt()
                } else {
                    0.0
                }
            })
            .collect();
        let mut elongation_values: Vec<f32> = stars_to_use.iter().map(|s| s.elongation).collect();
        
        // Sort for median calculation, handling NaN values
        let nan_safe_sort = |a: &f32, b: &f32| -> std::cmp::Ordering {
            if a.is_nan() && b.is_nan() {
                std::cmp::Ordering::Equal
            } else if a.is_nan() {
                std::cmp::Ordering::Greater
            } else if b.is_nan() {
                std::cmp::Ordering::Less
            } else {
                a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal)
            }
        };
        
        kron_values.sort_by(nan_safe_sort);
        flux_values.sort_by(nan_safe_sort);
        snr_values.sort_by(nan_safe_sort);
        elongation_values.sort_by(nan_safe_sort);
        
        // Calculate medians
        let median_kron_radius = if !kron_values.is_empty() { kron_values[kron_values.len() / 2] } else { 0.0 };
        let median_flux = if !flux_values.is_empty() { flux_values[flux_values.len() / 2] } else { 0.0 };
        let median_snr = if !snr_values.is_empty() { snr_values[snr_values.len() / 2] } else { 0.0 };
        let median_elongation = if !elongation_values.is_empty() { elongation_values[elongation_values.len() / 2] } else { 0.0 };
        
        // Calculate standard deviations for additional metrics
        let kron_radius_std_dev = calculate_std_dev(&kron_values);
        let flux_std_dev = calculate_std_dev(&flux_values);
        let snr_std_dev = calculate_std_dev(&snr_values);
        
        // Calculate flagged fraction
        let flagged_count = stars_to_use.iter().filter(|s| s.flag != 0).count();
        let flagged_fraction = if !stars_to_use.is_empty() {
            flagged_count as f32 / stars_to_use.len() as f32
        } else {
            0.0
        };

        StarStats {
            count: stars.len(),
            median_fwhm,
            median_eccentricity,
            fwhm_std_dev,
            eccentricity_std_dev,
            median_kron_radius,
            median_flux,
            median_snr,
            median_elongation,
            flagged_fraction,
            kron_radius_std_dev,
            flux_std_dev,
            snr_std_dev,
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_calc_eccentricity() {
        // Create a circular star (a = b)
        let mut star = StarMetrics {
            x: 100.0,
            y: 100.0,
            flux: 1000.0,
            peak: 100.0,
            a: 5.0,
            b: 5.0,
            theta: 0.0,
            eccentricity: 0.0,
            fwhm: 0.0,
            kron_radius: 10.0,
            flux_auto: 1200.0,
            fluxerr_auto: 20.0,
            npix: 50,
            elongation: 1.0,
            flag: 0,
        };
        
        star.calc_eccentricity();
        assert_eq!(star.eccentricity, 0.0);
        
        // Create an elliptical star (a > b)
        let mut star = StarMetrics {
            x: 100.0,
            y: 100.0,
            flux: 1000.0,
            peak: 100.0,
            a: 10.0,
            b: 5.0,
            theta: 0.0,
            eccentricity: 0.0,
            fwhm: 0.0,
            kron_radius: 10.0,
            flux_auto: 1200.0,
            fluxerr_auto: 20.0,
            npix: 50,
            elongation: 2.0,
            flag: 0,
        };
        
        star.calc_eccentricity();
        assert!(star.eccentricity > 0.0 && star.eccentricity < 1.0);
        
        // The eccentricity should be sqrt(1 - (b/a)²) = sqrt(1 - 0.25) = sqrt(0.75) ≈ 0.866
        assert!((star.eccentricity - 0.866).abs() < 0.001);
    }
    
    #[test]
    fn test_calc_fwhm() {
        let mut star = StarMetrics {
            x: 100.0,
            y: 100.0,
            flux: 1000.0,
            peak: 100.0,
            a: 6.0,
            b: 4.0,
            theta: 0.0,
            eccentricity: 0.0,
            fwhm: 0.0,
            kron_radius: 10.0,
            flux_auto: 1200.0,
            fluxerr_auto: 20.0,
            npix: 50,
            elongation: 1.5,
            flag: 0,
        };
        
        star.calc_fwhm();
        // FWHM should be the average of a and b
        assert_eq!(star.fwhm, 5.0);
    }
    
    #[test]
    fn test_from_stars() {
        // Create a collection of test stars
        let stars = vec![
            StarMetrics {
                x: 100.0, y: 100.0, flux: 1000.0, peak: 100.0,
                a: 6.0, b: 4.0, theta: 0.0, eccentricity: 0.8, fwhm: 5.0,
                kron_radius: 10.0, flux_auto: 1200.0, fluxerr_auto: 20.0,
                npix: 50, elongation: 1.5, flag: 0,
            },
            StarMetrics {
                x: 200.0, y: 200.0, flux: 2000.0, peak: 200.0,
                a: 8.0, b: 6.0, theta: 0.0, eccentricity: 0.7, fwhm: 7.0,
                kron_radius: 12.0, flux_auto: 2400.0, fluxerr_auto: 30.0,
                npix: 70, elongation: 1.33, flag: 1,
            },
            StarMetrics {
                x: 300.0, y: 300.0, flux: 3000.0, peak: 300.0,
                a: 4.0, b: 3.0, theta: 0.0, eccentricity: 0.6, fwhm: 3.5,
                kron_radius: 8.0, flux_auto: 3600.0, fluxerr_auto: 40.0,
                npix: 30, elongation: 1.33, flag: 0,
            },
        ];
        
        // Calculate stats
        let stats = StarStats::from_stars(&stars, None);
        
        // Check basic stats
        assert_eq!(stats.count, 3);
        assert_eq!(stats.median_fwhm, 5.0);
        assert_eq!(stats.median_eccentricity, 0.7);
        
        // Check flagged fraction (1 out of 3 stars is flagged)
        assert_eq!(stats.flagged_fraction, 1.0/3.0);
    }
}