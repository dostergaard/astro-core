#[test]
fn test_crate_imports() {
    // Just verify that key exports are available.
    let _ = astro_core::metadata::types::AstroMetadata::default();
    let _star_metrics: Option<astro_core::metrics::types::StarMetrics> = None;
}
