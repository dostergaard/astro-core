//! Type definitions for astronomical metrics
//!
//! This module defines the structures used to represent metrics and statistics
//! for astronomical images, including star measurements, background analysis,
//! and quality scores.

use serde::Serialize;

/// Measurements for a single detected star
#[derive(Debug, Clone, Serialize)]
pub struct StarMetrics {
    /// X centroid position
    pub x: f64,
    /// Y centroid position
    pub y: f64,
    /// Total flux
    pub flux: f32,
    /// Peak pixel value
    pub peak: f32,
    /// Semi-major axis
    pub a: f32,
    /// Semi-minor axis
    pub b: f32,
    /// Position angle in radians
    pub theta: f32,
    /// Eccentricity (derived from a and b)
    pub eccentricity: f32,
    /// Full Width at Half Maximum (derived from a and b)
    pub fwhm: f32,
}

/// Aggregate statistics for a collection of stars
#[derive(Debug, Clone, Serialize)]
pub struct StarStats {
    /// Total number of stars detected
    pub count: usize,
    /// Median FWHM across all stars
    pub median_fwhm: f32,
    /// Median eccentricity across all stars
    pub median_eccentricity: f32,
    /// Standard deviation of FWHM
    pub fwhm_std_dev: f32,
    /// Standard deviation of eccentricity
    pub eccentricity_std_dev: f32,
}

/// Holds background statistics for an image
#[derive(Debug, Clone, Serialize)]
pub struct BackgroundMetrics {
    /// Median background level
    pub median: f32,
    /// Background noise level (RMS)
    pub rms: f32,
    /// Minimum background level
    pub min: f32,
    /// Maximum background level
    pub max: f32,
    /// Measure of background uniformity (0-1, higher is more uniform)
    pub uniformity: f32,
}

/// Weights for calculating overall quality score
#[derive(Debug, Clone, Copy, Serialize)]
pub struct QualityWeights {
    /// Weight for FWHM score (default: 0.4)
    pub fwhm: f32,
    /// Weight for eccentricity score (default: 0.3)
    pub eccentricity: f32,
    /// Weight for background score (default: 0.3)
    pub background: f32,
}

impl Default for QualityWeights {
    fn default() -> Self {
        Self {
            fwhm: 0.4,
            eccentricity: 0.3,
            background: 0.3,
        }
    }
}

/// Normalized quality scores for a frame
/// All scores are normalized to a 0-1 scale where higher values are better
#[derive(Debug, Clone, Serialize)]
pub struct QualityScores {
    /// FWHM score (higher means better focus/seeing)
    pub fwhm: f32,
    /// Eccentricity score (higher means rounder stars)
    pub eccentricity: f32,
    /// Background score (higher means better background)
    pub background: f32,
    /// Overall quality score (weighted average of scores)
    pub overall: f32,
}

/// Overall frame quality metrics
#[derive(Debug, Clone, Serialize)]
pub struct FrameQualityMetrics {
    /// Frame identifier
    pub frame_id: String,
    /// Star detection statistics
    pub star_stats: StarStats,
    /// Background statistics
    pub background: BackgroundMetrics,
    /// Normalized quality scores (0-1 scale, higher is better)
    pub scores: QualityScores,
}