//! Quality metrics calculation for astronomical images

use std::path::{Path, PathBuf};
use anyhow::Result;
use crate::types::{StarStats, BackgroundMetrics, QualityScores, QualityWeights, FrameQualityMetrics};

/// Calculate quality scores for a frame
pub fn calculate_quality_scores(
    star_stats: &StarStats,
    background: &BackgroundMetrics,
) -> QualityScores {
    // Calculate individual scores (0-1 scale, higher is better)
    
    // FWHM score: Lower FWHM is better, so invert the scale
    // Typical good FWHM is 2-4 pixels, terrible is >8 pixels
    let fwhm_score = (1.0 - (star_stats.median_fwhm / 10.0).min(1.0)).max(0.0);
    
    // Eccentricity score: Lower eccentricity (rounder stars) is better
    // 0 = perfect circle, 1 = line
    let eccentricity_score = (1.0 - star_stats.median_eccentricity).max(0.0);
    
    // Background score: Higher uniformity is better
    let background_score = background.uniformity;
    
    // Kron radius score: Lower radius is better (tighter stars)
    let kron_score = (1.0 - (star_stats.median_kron_radius / 10.0).min(1.0)).max(0.0);
    
    // SNR score: Higher is better
    let snr_score = (star_stats.median_snr / 100.0).min(1.0);
    
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
    let fwhm_score = (1.0 - (star_stats.median_fwhm / 10.0).min(1.0)).max(0.0);
    let eccentricity_score = (1.0 - star_stats.median_eccentricity).max(0.0);
    let background_score = background.uniformity;
    let kron_score = (1.0 - (star_stats.median_kron_radius / 10.0).min(1.0)).max(0.0);
    let snr_score = (star_stats.median_snr / 100.0).min(1.0);
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