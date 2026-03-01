//! Background metrics for astronomical images

use crate::types::BackgroundMetrics;

impl BackgroundMetrics {
    /// Create a new BackgroundMetrics instance with basic values
    pub fn new(median: f32, rms: f32) -> Self {
        Self {
            median,
            rms,
            min: 0.0, // Default values, can be updated later
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
