#[test]
fn test_crate_imports() {
    // Just verify that key exports are available.
    let _ = ravensky_astro::metadata::types::AstroMetadata::default();
    let _star_metrics: Option<ravensky_astro::metrics::types::StarMetrics> = None;
}
