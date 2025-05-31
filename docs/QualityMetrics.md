# Quality Metrics for Astronomical Images

This document describes the quality metrics used to evaluate astronomical images in the astro-metrics library.

## Individual Metrics

### FWHM Score (Focus Quality)

The Full Width at Half Maximum (FWHM) score measures the quality of focus and seeing conditions. It combines:

- **Base FWHM**: Lower FWHM values indicate better focus/seeing
  - Formula: `(1.0 - (median_fwhm / 10.0).min(1.0)).max(0.0)`
  - Typical good values: 2-4 pixels
  - Poor values: >8 pixels

- **FWHM Consistency**: Measures how consistent the star sizes are across the frame
  - Formula: `(1.0 - (fwhm_std_dev / median_fwhm).min(1.0)).max(0.0)`
  - Higher values indicate more consistent star sizes

- **Combined**: `fwhm_base_score * 0.7 + fwhm_consistency * 0.3`

### Elongation/Eccentricity Score (Star Shape)

Measures how round the stars are:

- **Elongation**: Based on the ratio of semi-major to semi-minor axis (a/b)
  - Formula: `(1.0 - ((median_elongation - 1.0) / 2.0).min(1.0)).max(0.0)`
  - Perfect circle: elongation = 1.0, score = 1.0
  - Typical good values: <1.5
  - Poor values: >3.0

### Background Score

Measures the quality of the background, combining:

- **Uniformity**: How uniform the background is across the frame (0-1)
  - Higher values indicate more uniform background

- **Noise Level**: How low the background noise is
  - Formula: `(1.0 - (rms / 10.0).min(1.0)).max(0.0)`
  - Lower RMS values indicate less noise

- **Combined**: `uniformity * 0.7 + noise_score * 0.3`

### Kron Radius Score

Measures the compactness of stars:

- Formula: `(1.0 - (median_kron_radius / 10.0).min(1.0)).max(0.0)`
- Lower Kron radius values indicate tighter, more point-like stars
- Higher scores indicate better optical quality and seeing conditions

### SNR Score

Measures the signal-to-noise ratio using a logarithmic scale:

- Formula: `(1.0 - 10.0 / (10.0 + median_snr)).max(0.0)`
- SNR of 10 → score of 0.5
- SNR of 100 → score of 0.83
- SNR of 1000 → score of 0.99
- Logarithmic scale better represents human perception of quality differences

### Flag Score

Measures the proportion of stars without extraction flags:

- Formula: `1.0 - flagged_fraction`
- Higher values indicate fewer problematic star detections
- Flags indicate issues like truncation at image edges, blending with neighbors, or saturation

## Overall Quality Score

The overall quality score is a weighted average of the individual scores:

- FWHM: 30%
- Eccentricity/Elongation: 20%
- Background: 20%
- Kron Radius: 15%
- SNR: 10%
- Flag: 5%

These weights can be customized to emphasize different aspects of image quality based on specific needs.

## Using Quality Metrics

Quality metrics can be used to:

1. **Compare frames** to select the best ones for stacking
2. **Monitor trends** in image quality over time
3. **Diagnose issues** with equipment or acquisition settings
4. **Optimize processing parameters** for best results

The metrics are normalized to a 0-1 scale where higher values always indicate better quality, making them easy to compare and visualize.