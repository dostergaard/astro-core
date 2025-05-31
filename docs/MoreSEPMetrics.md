SEP/SExtractor can give you dozens of additional per-source measurements that are great candidates for frame-culling.  Here are some you might consider adding:

| Metric                      | SEP name(s)                      | What it tells you                                             | Why include it?                                                                                       |
| --------------------------- | -------------------------------- | ------------------------------------------------------------- | ----------------------------------------------------------------------------------------------------- |
| **Kron Radius**             | `FLUX_RADIUS` (e.g. at 50%, 90%) | The radius enclosing a fixed fraction of total flux           | Frames with defocused or trailed stars will have unusually large radii.                               |
| **Total Flux / Mag**        | `FLUX_AUTO` / `MAG_AUTO`         | Automatic aperture flux and magnitude                         | Low‐flux frames (clouds, transparency drop) show systematically low flux.                             |
| **Flux Error / SNR**        | `FLUXERR_AUTO`                   | Uncertainty on `FLUX_AUTO`                                    | Compute per‐star SNR = FLUX\_AUTO/FLUXERR\_AUTO; median SNR flags noisy frames.                       |
| **Peak Surface Brightness** | `MU_MAX`                         | Peak pixel value above local background                       | Helps catch saturated frames or ones with hot pixels.                                                 |
| **Detection Area**          | `ISOAREA_IMAGE`                  | Number of pixels above threshold                              | Long satellite trails or background glows inflate ISOAREA unexpectedly.                               |
| **Shape Axes**              | `A_IMAGE`, `B_IMAGE`             | Semi-major / semi-minor axes lengths                          | Region-by-region PSF shape; you can compute elongation = A/B in addition to eccentricity.             |
| **Elongation**              | `ELONGATION`                     | $a/b$                                                         | A second view on PSF distortion, complementary to eccentricity.                                       |
| **FLAGS**                   | `FLAGS`                          | Bitmask of extraction issues (blends, saturation, truncation) | Frames with a large fraction of flagged stars often have artifacts or focus issues.                   |
| **Neighbors**               | `NPIX_` or segmentation flags    | Number of overlapping detections                              | High blending indicates crowded or trailed frames.                                                    |
| **CLASS\_STAR**             | `CLASS_STAR`                     | Neural‐net “star‐likeness” (0=galaxy/noise, 1=point‐source)   | Low median class\_star can flag fuzzy/blurred frames.                                                 |
| **Centroid shifts**         | Δ(x), Δ(y) across subregions     | Spatial variation in centroid positions                       | You can subdivide your frame into quadrants: large centroid shifts indicate flexure or guiding drift. |

---

### How to use them in culling

1. **Compute per‐frame summaries**
   For each metric above, take the median (or fraction above/below a threshold) across all detected sources.

2. **Add “sigma‐clipping” rules**
   Just like you do with FWHMSigma and StarsSigma, define acceptable ranges for:

   * Kron radius sigma
   * Median SNR sigma
   * Fraction of sources with FLAGS ≠ 0

3. **Combine with existing filters**
   For example:

   ```text
   KronRadiusSigma ≤ 2 &&
   MedianSNRSigma ≥ 1.5 &&
   FlaggedFraction ≤ 0.05
   ```

4. **Visualize correlations**
   Plot FWHM vs. FLUX\_RADIUS or SNR vs. FLAGGED\_FRACTION to see which metrics best separate “good” from “bad” frames in your archive.

Adding any or all of these will give you a richer, multi-dimensional quality metric space—so your automated culling can catch subtle defects (e.g. slight trailing, low transparency, partial saturation) that aren’t visible to the eye or captured by FWHM/eccentricity alone.
