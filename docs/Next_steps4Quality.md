Now that we have integrated with SEP we have access to all of the detected object (presumably stars) centroids. We could pass this data to any other star related metric functions so that they dont have to include star detection in their algorithms and would be consistently measuring the same objects. We might also want to be able to pass in an upper limit on the number of stars to use for measurement to improve efficiencies with a default of 1000. Although we still want the count of the actual number os stars detected we could keep the list of the n brightest stars to pass to other measurement functions.

We also have access via SEP to do background extraction and background noise detection and therefore maybe we can develop some metrics to determine the quality of the background that is better than a simple mean for the whole image to distinguish frames with too much light or obstructions such as clouds or tree branches.

The goal is to determine the quality of the whole frame not just a single object within it. We should use a Rust‐centric workflow, with common crates and idioms. Here are a few ideas for better metrics but we should focus on the 4 or 5 most determinative measures. Currently I typically rely on "FWHMSigma <= 2 && EccentricitySigma <= 2 && MedianSigma <= 2 && StarsSigma >= -1.5" in PixInsight's SubFrameSelector as well as the "quality" score in AstroPixelProcessor.

The full frame focus measures are intriguing assuming those algorithms are effective in an astrophotograph that often contain clouds of nebulosity or fuzzy galaxies. It would be a useful metric that I have not seen offered in other tools.

1. **Collect per-star metrics into Rust collections**

   * In the star-detection loop (e.g. using the existing detection code or an `opencv` crate binding), push each star's FWHM and elongation into a `Vec<f32>`.
   * If you're doing this in parallel, consider using `rayon::iter::ParallelIterator` to detect & collect across image strips simultaneously.

```rust
  pub struct StarMetrics {
      pub x: f64,         // x centroid
      pub y: f64,         // y centroid
      pub flux: f32,      // total flux
      pub peak: f32,      // peak value
      pub a: f32,         // semi-major axis
      pub b: f32,         // semi-minor axis
      pub theta: f32,     // position angle
      pub eccentricity: f32, // derived from a and b
      pub fwhm: f32,      // derived from a and b
  }
```

2. **Compute robust statistics with Rust numeric crates**

   * Use [`ndarray`](https://crates.io/crates/ndarray) to wrap the `Vec`s if you prefer array operations, or compute directly on the `Vec` with [`statrs`](https://crates.io/crates/statrs) or [`statistical`](https://crates.io/crates/statistical).

     ```rust
     let median_fwhm = statistical::median(&fwhm_values);
     let iqr_fwhm    = statistical::quartiles(&fwhm_values).iqr();
     let std_fwhm    = statistical::standard_deviation(&fwhm_values, None);
     let skew_fwhm   = statrs::statistics::Statistics::skewness(&fwhm_values);
     ```

3. **Compute full‐frame focus measures**

   * Use the [`opencv`](https://crates.io/crates/opencv) crate to run a Laplacian or Sobel kernel over the frame:

     ```rust
     let laplacian = imgproc::laplacian(&gray_mat, core::CV_64F, 1, 1.0, 0.0, core::BORDER_DEFAULT)?;
     let lap_var   = laplacian.to_mat()?.mean_std_dev(/* … */)?.1.mean();
     ```
   
4. **Tile the frame and measure spatial uniformity**

   * Divide the image buffer into an N×M grid by slicing the `ndarray::Array2<u8>` (or the flat buffer with manual indexing).
   * For each tile, recompute just the stats you care about (e.g. median FWHM, background RMS). Collect each tile's result into a small `Vec`, then compute `max – min` or `std_dev` across that `Vec`.
   * Example (pseudocode):

     ```rust
     let tiles = image.view()
                      .axis_chunks_iter(Axis(0), tile_h)
                      .flat_map(|row| row.axis_chunks_iter(Axis(1), tile_w));
     let mut tile_medians = Vec::with_capacity(num_tiles);
     for tile in tiles {
         let stars = detect_stars_in(&tile);
         tile_medians.push(statistical::median(&stars.fwhm));
     }
     let fwhm_spread = tile_medians.iter().cloned().fold(0./0., f32::max)
                       - tile_medians.iter().cloned().fold(0./0., f32::min);
     ```

5. **Emit an extended CSV row per frame**

   * Use the [`csv`](https://crates.io/crates/csv) crate along with Serde to serialize a struct:

     ```rust
     #[derive(Serialize)]
     struct FrameStats {
         frame: String,
         star_count: usize,
         median_fwhm: f32,
         fwhm_iqr: f32,
         fwhm_stddev: f32,
         fwhm_skew: f32,
         laplacian_var: f64,
         tenengrad: f64,
         background_rms: f32,
         fwhm_tile_spread: f32,
         background_tile_spread: f32,
         quality_score: f32,
     }
     let mut wtr = csv::Writer::from_path("sfs_stats.csv")?;
     wtr.serialize(frame_stats)?;
     wtr.flush()?;
     ```
   * Compute an overall `quality_score` by normalizing each metric (e.g. min–max scaling) and combining them in the Rust code before serialization.

–––
With these steps in place, each row in the output will still correspond to one frame, but now carries a richer set of robust, Rust-computed metrics that better capture overall image quality.

---

# Ideas section

```rust
use serde::Serialize;

#[derive(Serialize)]
pub struct FrameStats {
    // File information
    pub frame: String,
    pub exposure: f32,
    pub filter: String,
    
    // Star detection stats
    pub star_count: usize,
    pub median_fwhm: f32,
    pub fwhm_std_dev: f32,
    pub median_eccentricity: f32,
    pub eccentricity_std_dev: f32,
    
    // Background stats
    pub background_median: f32,
    pub background_rms: f32,
    
    // Normalized scores (-2 to +2 sigma range)
    pub fwhm_score: f32,
    pub eccentricity_score: f32,
    pub background_score: f32,
    pub star_count_score: f32,
    
    // Overall quality (weighted average of normalized scores)
    pub quality_score: f32,
}
```
