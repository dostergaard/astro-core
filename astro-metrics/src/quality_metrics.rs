//! Quality metrics calculation for astronomical images

use std::path::Path;
use crate::types::{StarStats, BackgroundMetrics, QualityScores, QualityWeights, FrameQualityMetrics};

/// Calculate quality scores for a frame
pub fn calculate_quality_scores(
    star_stats: &StarStats,
    background: &BackgroundMetrics,
) -> QualityScores {
    // Calculate individual scores (0-1 scale, higher is better)
    
    // FWHM score: Lower FWHM is better, so invert the scale
    // Typical good FWHM is 2-4 pixels, terrible is >8 pixels
    let fwhm_base_score = (1.0 - (star_stats.median_fwhm / 10.0).min(1.0)).max(0.0);
    
    // FWHM consistency: Lower std_dev relative to median is better
    let fwhm_consistency = if star_stats.median_fwhm > 0.0 {
        (1.0 - (star_stats.fwhm_std_dev / star_stats.median_fwhm).min(1.0)).max(0.0)
    } else {
        0.0
    };
    
    // Combine base FWHM score with consistency
    let fwhm_score = fwhm_base_score * 0.7 + fwhm_consistency * 0.3;
    
    // Elongation score: Lower elongation (closer to 1.0) is better
    // Typical good value is <1.5, bad is >3.0
    let elongation_score = (1.0 - ((star_stats.median_elongation - 1.0) / 2.0).min(1.0)).max(0.0);
    
    // Use elongation score instead of eccentricity
    let eccentricity_score = elongation_score;
    
    // Background score: Combine uniformity with noise level
    // Lower RMS is better, normalize against a typical good value
    let noise_score = (1.0 - (background.rms / 10.0).min(1.0)).max(0.0);
    let background_score = background.uniformity * 0.7 + noise_score * 0.3;
    
    // Kron radius score: Lower radius is better (tighter stars)
    let kron_score = (1.0 - (star_stats.median_kron_radius / 10.0).min(1.0)).max(0.0);
    
    // SNR score: Use a logarithmic scale that better represents human perception
    // SNR of 10 → 0.5, SNR of 100 → 0.83, SNR of 1000 → 1.0
    let snr_score = (1.0 - 10.0 / (10.0 + star_stats.median_snr)).max(0.0);
    
    // Flag score: Lower flagged fraction is better
    let flag_score = 1.0 - star_stats.flagged_fraction;
    
    // Use default weights to calculate overall score
    let weights = QualityWeights::default();
    let overall = calculate_overall_score(
        fwhm_score,
        eccentricity_score,
        background_score,
        kron_score,
        snr_score,
        flag_score,
        &weights
    );
    
    QualityScores {
        fwhm: fwhm_score,
        eccentricity: eccentricity_score,
        background: background_score,
        kron_radius: kron_score,
        snr: snr_score,
        flag: flag_score,
        overall,
    }
}

/// Calculate overall quality score from individual scores and weights
pub fn calculate_overall_score(
    fwhm_score: f32,
    eccentricity_score: f32,
    background_score: f32,
    kron_score: f32,
    snr_score: f32,
    flag_score: f32,
    weights: &QualityWeights,
) -> f32 {
    let sum = weights.fwhm + weights.eccentricity + weights.background + 
              weights.kron_radius + weights.snr + weights.flag;
    if sum == 0.0 {
        return 0.0;
    }
    
    (fwhm_score * weights.fwhm + 
     eccentricity_score * weights.eccentricity + 
     background_score * weights.background +
     kron_score * weights.kron_radius +
     snr_score * weights.snr +
     flag_score * weights.flag) / sum
}

/// Create frame quality metrics for an image
pub fn create_frame_metrics(
    path: &Path,
    star_stats: StarStats,
    background: BackgroundMetrics,
) -> FrameQualityMetrics {
    // Use filename as frame_id
    let frame_id = path.file_name()
        .map(|name| name.to_string_lossy().to_string())
        .unwrap_or_else(|| "unknown".to_string());
    
    // Calculate quality scores
    let scores = calculate_quality_scores(&star_stats, &background);
    
    FrameQualityMetrics {
        frame_id,
        star_stats,
        background,
        scores,
    }
}

/// Create frame quality metrics with custom weights
pub fn create_frame_metrics_with_weights(
    path: &Path,
    star_stats: StarStats,
    background: BackgroundMetrics,
    weights: QualityWeights,
) -> FrameQualityMetrics {
    // Use filename as frame_id
    let frame_id = path.file_name()
        .map(|name| name.to_string_lossy().to_string())
        .unwrap_or_else(|| "unknown".to_string());
    
    // Calculate individual scores
    // FWHM score with consistency
    let fwhm_base_score = (1.0 - (star_stats.median_fwhm / 10.0).min(1.0)).max(0.0);
    let fwhm_consistency = if star_stats.median_fwhm > 0.0 {
        (1.0 - (star_stats.fwhm_std_dev / star_stats.median_fwhm).min(1.0)).max(0.0)
    } else {
        0.0
    };
    let fwhm_score = fwhm_base_score * 0.7 + fwhm_consistency * 0.3;
    
    // Elongation score
    let elongation_score = (1.0 - ((star_stats.median_elongation - 1.0) / 2.0).min(1.0)).max(0.0);
    let eccentricity_score = elongation_score;
    
    // Enhanced background score
    let noise_score = (1.0 - (background.rms / 10.0).min(1.0)).max(0.0);
    let background_score = background.uniformity * 0.7 + noise_score * 0.3;
    
    // Kron radius score
    let kron_score = (1.0 - (star_stats.median_kron_radius / 10.0).min(1.0)).max(0.0);
    
    // Logarithmic SNR score
    let snr_score = (1.0 - 10.0 / (10.0 + star_stats.median_snr)).max(0.0);
    let flag_score = 1.0 - star_stats.flagged_fraction;
    
    // Calculate overall score with custom weights
    let overall = calculate_overall_score(
        fwhm_score,
        eccentricity_score,
        background_score,
        kron_score,
        snr_score,
        flag_score,
        &weights
    );
    
    let scores = QualityScores {
        fwhm: fwhm_score,
        eccentricity: eccentricity_score,
        background: background_score,
        kron_radius: kron_score,
        snr: snr_score,
        flag: flag_score,
        overall,
    };
    
    FrameQualityMetrics {
        frame_id,
        star_stats,
        background,
        scores,
    }
}

#[cfg(test)]
mod tests {
    use super::*;


    #[test]
    fn test_calculate_quality_scores() {
        // Create test data
        let star_stats = StarStats {
            count: 100,
            median_fwhm: 3.0,
            median_eccentricity: 0.2,
            fwhm_std_dev: 0.5,
            eccentricity_std_dev: 0.05,
            median_kron_radius: 5.0,
            median_flux: 1000.0,
            median_snr: 50.0,
            median_elongation: 1.2,
            flagged_fraction: 0.05,
            kron_radius_std_dev: 1.0,
            flux_std_dev: 200.0,
            snr_std_dev: 10.0,
        };
        
        let background = BackgroundMetrics {
            median: 100.0,
            rms: 5.0,
            min: 90.0,
            max: 110.0,
            uniformity: 0.9,
        };
        
        // Calculate scores
        let scores = calculate_quality_scores(&star_stats, &background);
        
        // Verify scores are in the expected range (0-1)
        assert!(scores.fwhm >= 0.0 && scores.fwhm <= 1.0);
        assert!(scores.eccentricity >= 0.0 && scores.eccentricity <= 1.0);
        assert!(scores.background >= 0.0 && scores.background <= 1.0);
        assert!(scores.kron_radius >= 0.0 && scores.kron_radius <= 1.0);
        assert!(scores.snr >= 0.0 && scores.snr <= 1.0);
        assert!(scores.flag >= 0.0 && scores.flag <= 1.0);
        assert!(scores.overall >= 0.0 && scores.overall <= 1.0);
        
        // Verify specific calculations
        assert_eq!(scores.flag, 0.95); // 1.0 - 0.05
    }
    
    #[test]
    fn test_calculate_overall_score() {
        // Create test weights
        let weights = QualityWeights {
            fwhm: 0.3,
            eccentricity: 0.2,
            background: 0.2,
            kron_radius: 0.15,
            snr: 0.1,
            flag: 0.05,
        };
        
        // Calculate overall score with all metrics at 0.5
        let overall = calculate_overall_score(0.5, 0.5, 0.5, 0.5, 0.5, 0.5, &weights);
        
        // Should be 0.5 since all inputs are 0.5
        assert_eq!(overall, 0.5);
        
        // Test with different values
        let overall = calculate_overall_score(1.0, 0.0, 0.5, 0.5, 0.5, 0.5, &weights);
        
        // Manual calculation: (1.0*0.3 + 0.0*0.2 + 0.5*0.2 + 0.5*0.15 + 0.5*0.1 + 0.5*0.05) / 1.0
        let expected = (1.0*0.3 + 0.0*0.2 + 0.5*0.2 + 0.5*0.15 + 0.5*0.1 + 0.5*0.05) / 1.0;
        assert_eq!(overall, expected);
    }
}