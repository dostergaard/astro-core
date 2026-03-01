//! Statistical metrics for astronomical images

pub mod background_metrics;
pub mod quality_metrics;
pub mod sep_detect;
pub mod star_metrics;
pub mod types;

// Re-export common types
pub use quality_metrics::{
    calculate_overall_score, calculate_quality_scores, create_frame_metrics,
    create_frame_metrics_with_weights,
};
pub use types::{
    BackgroundMetrics, FrameQualityMetrics, QualityScores, QualityWeights, StarMetrics, StarStats,
};
