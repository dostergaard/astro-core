//! Star detection using the SEP (Source Extractor as a Library) C library

use std::ffi::{c_int, CStr};
use anyhow::{Result, anyhow};
use sep_sys as sep;
use crate::types::{StarMetrics, StarStats, BackgroundMetrics};
 
/// Detect stars using SEP's built-in background estimation and object detection
pub fn detect_stars_with_sep_background(
    data: &[f32],
    width: usize,
    height: usize,
    max_stars: Option<usize>,
) -> Result<(StarStats, BackgroundMetrics)> {
    unsafe {
        // Create a sep_image struct for background estimation
        let mut image_data = data.to_vec();
        let sep_img = sep::sep_image {
            data: image_data.as_mut_ptr() as *const std::ffi::c_void,
            noise: std::ptr::null(),
            mask: std::ptr::null(),
            segmap: std::ptr::null(),
            dtype: sep::SEP_TFLOAT as c_int,
            ndtype: 0,
            mdtype: 0,
            sdtype: 0,
            segids: std::ptr::null_mut(),
            idcounts: std::ptr::null_mut(),
            numids: 0,
            w: width as i64,
            h: height as i64,
            noiseval: 0.0,
            noise_type: 0,
            gain: 1.0,
            maskthresh: 0.0,
        };

        // Set background estimation parameters
        let bw = 64;  // box width
        let bh = 64;  // box height
        let fw = 3;   // filter width
        let fh = 3;   // filter height
        let fthresh = 0.0;  // filter threshold

        // Create a mutable pointer for the background struct
        let mut bkg: *mut sep::sep_bkg = std::ptr::null_mut();

        // Estimate background
        let status = sep::sep_background(
            &sep_img as *const sep::sep_image,
            bw,
            bh,
            fw,
            fh,
            fthresh,
            &mut bkg,
        );

        if status != 0 {
            let mut errbuf = [0i8; 512];
            sep::sep_get_errmsg(status, errbuf.as_mut_ptr());
            let error_msg = CStr::from_ptr(errbuf.as_ptr()).to_string_lossy();
            return Err(anyhow!("SEP background estimation error: {}", error_msg));
        }

        // Get global background and RMS
        let background = sep::sep_bkg_global(bkg);
        let rms = sep::sep_bkg_globalrms(bkg);
        
        // Get min and max background values
        let mut min_bg = f32::MAX;
        let mut max_bg = f32::MIN;
        
        // Calculate background uniformity
        let nx = (*bkg).nx;
        let ny = (*bkg).ny;
        let back = (*bkg).back;
        
        for i in 0..(nx * ny) {
            let val = *back.offset(i as isize);
            min_bg = min_bg.min(val);
            max_bg = max_bg.max(val);
        }
        
        // Calculate uniformity as 1 - (max-min)/max
        // Higher values (closer to 1) mean more uniform background
        let uniformity = if max_bg > 0.0 {
            1.0 - (max_bg - min_bg) / max_bg
        } else {
            1.0
        };
        
        // Create background metrics with all values
        let bg_metrics = BackgroundMetrics::with_all_metrics(
            background, 
            rms,
            min_bg,
            max_bg,
            uniformity
        );

        // Free the background memory
        sep::sep_bkg_free(bkg);

        // Detect stars using the estimated background and RMS
        let star_stats = detect_stars_sep(data, width, height, background, rms, max_stars)?;
        
        Ok((star_stats, bg_metrics))
    }
}

/// Detect stars using the SEP library and return detailed measurements for each star.
pub fn detect_stars_sep(
    data: &[f32],
    width: usize,
    height: usize,
    background: f32,
    std_dev: f32,
    max_stars: Option<usize>,
) -> Result<StarStats> {
    // Skip processing if image is too small
    if width < 3 || height < 3 {
        return Ok(StarStats {
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
        });
    }

    // Create a copy of the data as f32 (SEP requires contiguous memory)
    let mut image_data = data.to_vec();

    unsafe {
        // Create a sep_image struct
        let sep_img = sep::sep_image {
            data: image_data.as_mut_ptr() as *const std::ffi::c_void,
            noise: std::ptr::null(),
            mask: std::ptr::null(),
            segmap: std::ptr::null(),
            dtype: sep::SEP_TFLOAT as c_int,
            ndtype: 0,
            mdtype: 0,
            sdtype: 0,
            segids: std::ptr::null_mut(),
            idcounts: std::ptr::null_mut(),
            numids: 0,
            w: width as i64,
            h: height as i64,
            noiseval: std_dev as f64,
            noise_type: sep::SEP_NOISE_STDDEV as i16,
            gain: 1.0,
            maskthresh: 0.0,
        };

        // Set threshold to 3 sigma above background
        let thresh = background + 3.0 * std_dev;
        
        // Create pointers for the catalog
        let mut catalog: *mut sep::sep_catalog = std::ptr::null_mut();
        
        // Call SEP to extract objects
        let status = sep::sep_extract(
            &sep_img as *const sep::sep_image,
            thresh,
            sep::SEP_THRESH_ABS as c_int,
            5,                            // Minimum area in pixels
            std::ptr::null(),             // No convolution filter
            0,                            // No convolution width
            0,                            // No convolution height
            sep::SEP_FILTER_CONV as c_int,
            32,                           // Deblend thresholds
            0.005,                        // Deblend contrast
            1,                            // Clean flag
            1.0,                          // Clean parameter
            &mut catalog,
        );

        // Check for errors
        if status != 0 {
            let mut errbuf = [0i8; 512];
            sep::sep_get_errmsg(status, errbuf.as_mut_ptr());
            let error_msg = CStr::from_ptr(errbuf.as_ptr()).to_string_lossy();
            return Err(anyhow!("SEP error: {}", error_msg));
        }

        // Convert SEP catalog to Vec<StarMetrics>
        let nobj = (*catalog).nobj as usize;
        let mut stars = Vec::with_capacity(nobj);

        for i in 0..nobj {
            // Get pointers to arrays
            let x = *(*catalog).x.add(i);
            let y = *(*catalog).y.add(i);
            let a = *(*catalog).a.add(i);
            let b = *(*catalog).b.add(i);
            let theta = *(*catalog).theta.add(i);
            let flux = *(*catalog).flux.add(i);
            let peak = *(*catalog).peak.add(i);

            // Extract additional metrics from SEP catalog
            let npix = (*catalog).npix.add(i) as usize;
            let flag = *(*catalog).flag.add(i) as u8;
            
            // Calculate derived metrics
            let elongation = if a > 0.0 && b > 0.0 { a / b } else { 1.0 };
            
            // Calculate Kron radius (radius containing 50% of flux)
            let mut kron_radius = 0.0;
            let mut krflag: i16 = 0;
            
            // Call sep_kron_radius to get the Kron radius
            // Note: SEP expects f64 for coordinates and dimensions
            let kr_status = sep::sep_kron_radius(
                &sep_img as *const sep::sep_image,
                x, y,                  // Object position (already f64)
                a as f64, b as f64,    // Semi-major and semi-minor axes (convert f32 to f64)
                theta as f64,          // Position angle (convert f32 to f64)
                6.0,                   // Number of Kron radii for measurement (typically 2.5 or 6.0)
                0,                     // Flags (0 = default)
                &mut kron_radius as *mut f32 as *mut f64,  // Output Kron radius
                &mut krflag            // Output flag
            );
            
            if kr_status != 0 {
                // If there's an error, use a default value
                kron_radius = 0.0;
            }
            
            // Calculate flux within elliptical aperture (AUTO flux)
            let mut flux_auto = flux;
            let mut fluxerr_auto = 0.0;
            let mut seflag = 0;
            
            if kron_radius > 0.0 {
                // Use Kron radius for aperture measurement (typically 2.5 * kron_radius)
                let kr_scale = 2.5;
                let auto_a = kr_scale * kron_radius * a;
                let auto_b = kr_scale * kron_radius * b;
                
                // Call sep_sum_ellipse to get the flux within the aperture
                let mut flux_auto_f64 = flux_auto as f64;
                let mut fluxerr_auto_f64 = 0.0;
                let mut seflag: i16 = 0;
                
                let se_status = sep::sep_sum_ellipse(
                    &sep_img as *const sep::sep_image,
                    x, y,                       // Object position (already f64)
                    auto_a as f64, auto_b as f64, // Scaled semi-major and semi-minor axes
                    theta as f64,               // Position angle
                    0.0,                        // No inner radius (full ellipse)
                    0,                          // Subpixel sampling (0 = default)
                    0,                          // Error type (0 = default)
                    0,                          // Aperture correction (0 = none)
                    &mut flux_auto_f64,         // Output flux
                    &mut fluxerr_auto_f64,      // Output flux error
                    &mut 0.0,                   // Output area (not used)
                    &mut seflag                 // Output flag
                );
                
                // Convert back to f32 for our struct
                flux_auto = flux_auto_f64 as f32;
                fluxerr_auto = fluxerr_auto_f64 as f32;
                
                if se_status != 0 {
                    // If there's an error, use the isophotal flux
                    flux_auto = flux;
                    fluxerr_auto = 0.0;
                }
            }
            
            let mut star = StarMetrics {
                x,
                y,
                flux,
                peak,
                a,
                b,
                theta,
                eccentricity: 0.0,
                fwhm: 0.0,
                kron_radius,
                flux_auto,
                fluxerr_auto,
                npix,
                elongation,
                flag,
            };

            // Calculate derived metrics
            star.calc_eccentricity();
            star.calc_fwhm();
            stars.push(star);
        }

        // Free the memory allocated by SEP
        if !catalog.is_null() {
            sep::sep_catalog_free(catalog);
        }

        // Calculate aggregate statistics
        let stats = StarStats::from_stars(&stars, max_stars);
        Ok(stats)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_detect_stars_sep() {
        let (w, h) = (20, 20);
        let mut data = vec![0.0; w * h];
        
        // Add a bright star in the center
        data[10 * w + 10] = 100.0;
        
        // Add some fainter stars
        data[5 * w + 5] = 50.0;
        data[15 * w + 15] = 50.0;
        
        // Test detection with background estimation
        let result = detect_stars_with_sep_background(&data, w, h, None);
        assert!(result.is_ok());
        
        let (stats, bg_metrics) = result.unwrap();
        assert!(stats.count > 0, "Should detect at least one star");
        assert!(stats.median_fwhm > 0.0, "FWHM should be positive");
        assert!(stats.median_eccentricity >= 0.0 && stats.median_eccentricity <= 1.0);
        assert!(bg_metrics.rms >= 0.0, "Background RMS should be non-negative");
        assert!(bg_metrics.uniformity >= 0.0 && bg_metrics.uniformity <= 1.0, 
                "Uniformity should be between 0 and 1");
    }
}