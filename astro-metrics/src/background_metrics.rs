//! Background metrics for astronomical images

/// Holds background statistics for an image
#[derive(Debug, Clone)]
pub struct BackgroundMetrics {
    pub median: f32,       // median background level
    pub rms: f32,          // background noise level (RMS)
    pub min: f32,          // minimum background level
    pub max: f32,          // maximum background level
    pub uniformity: f32,   // measure of background uniformity (0-1, higher is more uniform)
}

impl BackgroundMetrics {
    /// Create a new BackgroundMetrics instance with basic values
    pub fn new(median: f32, rms: f32) -> Self {
        Self {
            median,
            rms,
            min: 0.0,      // Default values, can be updated later
            max: 0.0,
            uniformity: 1.0,
        }
    }
    
    /// Create a new BackgroundMetrics instance with all values
    pub fn with_all_metrics(median: f32, rms: f32, min: f32, max: f32, uniformity: f32) -> Self {
        Self {
            median,
            rms,
            min,
            max,
            uniformity,
        }
    }
}