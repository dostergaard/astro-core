//! Statistical metrics for astronomical images

pub mod types;
pub mod star_metrics;
pub mod background_metrics;
pub mod sep_detect;
pub mod quality_metrics;

// Re-export common types
pub use types::{StarMetrics, StarStats, BackgroundMetrics, FrameQualityMetrics, QualityScores, QualityWeights};
pub use quality_metrics::{calculate_quality_scores, calculate_overall_score, create_frame_metrics, create_frame_metrics_with_weights};
