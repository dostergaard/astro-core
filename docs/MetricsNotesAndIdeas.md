Below is a summary of each column in **stats2.csv**, what it measures, what it tells you about the frame, its scale/units, and whether higher or lower values are “better” (i.e. generally indicate a higher-quality subframe).

| Statistic                  | What it measures                                           | What it indicates                                                                                | Scale & Units       | Higher vs. Lower                                        |
| -------------------------- | ---------------------------------------------------------- | ------------------------------------------------------------------------------------------------ | ------------------- | ------------------------------------------------------- |
| **star\_count**            | Total number of detected sources (peaks) in the frame      | Frame “richness” in stars; too few = underexposed/blurred; too many = noise or overly deep field | Integer count       | Depends: mid-range ideal (enough real stars, not noise) |
| **median\_fwhm**           | Median FWHM (full-width at half-maximum) of all star PSFs  | Overall sharpness/seeing: how “tight” the star images are                                        | Pixels              | **Lower** is better (sharper)                           |
| **fwhm\_std\_dev**         | Standard deviation of all the individual FWHM measurements | Consistency of focus across the frame                                                            | Pixels              | **Lower** is better (uniform focus)                     |
| **median\_eccentricity**   | Median PSF eccentricity (0 = perfect circle; 1 = line)     | How round the stars are on average                                                               | Unitless \[0…1]     | **Lower** is better (rounder)                           |
| **eccentricity\_std\_dev** | Spread (σ) of star eccentricities                          | Variation in star shapes (distortion/astigmatism)                                                | Unitless            | **Lower** is better (uniform shapes)                    |
| **background\_median**     | Median sky background level across mesh cells              | Typical sky brightness (ADU)                                                                     | ADU (camera counts) | **Lower** is generally better (darker sky)              |
| **background\_rms**        | Median of the per-cell background noise (σ)                | Frame noise floor                                                                                | ADU                 | **Lower** is better (less noise)                        |
| **background\_min**        | Minimum of the mesh-cell background medians                | Darkest region’s sky level                                                                       | ADU                 | **Lower** is better (no bright gradients)               |
| **background\_max**        | Maximum of the mesh-cell background medians                | Brightest region’s sky level                                                                     | ADU                 | **Lower** is better (even illumination)                 |
| **background\_uniformity** | Uniformity ratio = (background\_min ÷ background\_max)     | Flatness of sky: 1.0 means perfectly even background                                             | Unitless \[0…1]     | **Higher** is better (more uniform)                     |

---

### How to interpret

* **Sharpness & Focus (FWHM)**

  * *median\_fwhm* tells you the typical star size; low values (\~2–4 px) are crisp.
  * *fwhm\_std\_dev* flags if some corners of the frame are out of focus.

* **Shape Quality (Eccentricity)**

  * Stars ideally round: *median\_eccentricity* near 0.
  * High *eccentricity\_std\_dev* can mean tube flexure or tracking issues.

* **Background Metrics**

  * *background\_median* & *background\_rms* quantify sky brightness and noise.
  * Large spread between *background\_min* and *background\_max* means vignetting or clouds.
  * *background\_uniformity* close to 1 means even illumination.

* **Star Count**

  * Gives a rough idea of field richness but depends heavily on the detection threshold and subframe contents; too many may be hot pixels or noise peaks.

Use these metrics together (e.g. low median\_fwhm, low background\_rms, high uniformity) to rank or filter the subframes for stacking.


---

Here’s a pragmatic set of **acceptance/rejection criteria** based on the nine metrics.  You can apply them either as **fixed cutoffs** (if you know the system’s “normal” ranges) or as **outlier filters** (relative to the distribution across the full dataset).

---

## 1. Sharpness (FWHM)

| Metric             | What to watch for                  | Rule of thumb                      | Pass if…         |
| ------------------ | ---------------------------------- | ---------------------------------- | ---------------- |
| **median\_fwhm**   | Typical star size (seeing + focus) | ≲ 1.5× the camera’s pixel scale\* | ≤ 4 px (example) |
| **fwhm\_std\_dev** | Variation of FWHM across the frame | ≲ 0.5 px                           | ≤ 0.5 px         |

– If you don’t know absolute numbers, compute the **dataset median** and **MAD** (median absolute deviation), then reject any frame with

```text
median_fwhm > median_fwhm_dataset + 2·MAD_fwhm  
OR  
fwhm_std_dev > 2·MAD_fwhm_stddev
```

---

## 2. Shape (Eccentricity)

| Metric                     | What to watch for               | Rule of thumb | Pass if… |
| -------------------------- | ------------------------------- | ------------- | -------- |
| **median\_eccentricity**   | How round stars are (0…1)       | ≲ 0.3         | ≤ 0.3    |
| **eccentricity\_std\_dev** | Variation in shape (distortion) | ≲ 0.1         | ≤ 0.1    |

– Or reject frames > 75th percentile in **eccentricity** or > 90th percentile in **eccentricity\_std\_dev**.

---

## 3. Background Brightness & Noise

| Metric                     | What to watch for               | Rule of thumb             | Pass if…     |
| -------------------------- | ------------------------------- | ------------------------- | ------------ |
| **background\_median**     | Sky level (ADU)                 | dark sky ≲ some ADU limit | ≤ 1000 ADU\* |
| **background\_rms**        | Noise floor (σ of sky pixels)   | low noise ≲ 5 ADU\*       | ≤ 5 ADU      |
| **background\_uniformity** | Min/Max ratio across mesh (0…1) | flat field ≳ 0.9          | ≥ 0.9        |

– Or dynamically reject frames where `background_rms` or `background_median` > `median + 2·σ` of the dataset, or where `background_uniformity` < `median − 2·σ`.

---

## 4. Star Count

| Metric          | What to watch for                                                     | Rule of thumb                      | Pass if…         |
| --------------- | --------------------------------------------------------------------- | ---------------------------------- | ---------------- |
| **star\_count** | Field fullness; too few = bad focus or clouds; too many = noise peaks | ≳ 30 && ≲ 500 (for typical 60 s L) | 30 ≤ count ≤ 500 |

– Alternatively reject the lowest 10% and highest 5% of star\_count across the dataset to avoid both empty frames and noisy junk.

---

## Putting it all together

You can either:

1. **Hard thresholds** (for a known setup):

   ```text
   median_fwhm   ≤ 4.0 px
   fwhm_std_dev  ≤ 0.5 px
   median_ecc    ≤ 0.3
   ecc_std_dev   ≤ 0.1
   background_rms ≤ 5 ADU
   background_uniformity ≥ 0.9
   30 ≤ star_count ≤ 500
   ```
2. **Statistical filtering** (for any dataset):

   * Compute **median** and **σ or MAD** for each metric across all frames.
   * Reject frames > `(median + k·σ)` for “higher‐is‐worse” metrics (FWHM, rms, eccentricity, background level).
   * Reject frames < `(median − k·σ)` for “lower‐is‐worse” metrics (uniformity, star\_count).
   * A typical choice is `k = 2` or use percentile‐based cutoffs (e.g. discard worst 10%).

---

### Why these rules?

* **Sharpness & Shape** directly impact how stars resolve in the stack—reject blurred or distorted frames.
* **Background & Noise** determine the SNR; high or non-uniform sky adds noise and gradients to the final stack.
* **Star Count** ensures you have enough real signal (not just noise), but not so many false detections that you bloat computation.

By combining **absolute** and **relative** (dataset-driven) thresholds, you’ll automatically exclude subframes that would **degrade** the final stack—and keep only the crisp, low-noise, well-exposed images.

\* *Pixel‐scale* depends on the telescope+camera (e.g. 1″/px → FWHM ∼2–3 px for 2–3″ seeing).\_

---

Absolutely—those same per‐frame metrics can guide **when to stop** (or continue) collecting exposures by tracking **stack quality convergence** and **diminishing returns**. Here’s how you might use them in practice:

---

## 1. Signal‐to‐Noise (SNR) vs. Exposure Count

* **Theory:** SNR improves as $\sqrt{N}$ for $N$ identical frames.
* **Application:** After each new batch of, say, 10 frames, compute the **combined** background RMS (or use the per‐frame *background\_rms* averaged across the stack). Plot SNR or $1/\mathrm{background\_rms}$ vs. $\sqrt{N}$.
* **Stopping rule:** Once the incremental SNR gain from another batch is below a threshold—e.g. less than a 1 % improvement—you’ve essentially reached “enough” data.

---

## 2. Quality Distribution Convergence

* **Monitor** the distribution (e.g. median ± MAD) of key metrics like *median\_fwhm*, *background\_rms*, and *star\_count* as you accumulate frames.
* **Look for plateau:** If after $M$ frames the **mean** and **σ** of the FWHM stop improving (or the background RMS stops dropping), you’re hitting diminishing returns.
* **Rule of thumb:** When the moving average over the last $k$ frames changes by less than a few percent, you can stop.

---

## 3. Reject‐Then‐Collect Feedback Loop

* Use the **acceptance criteria** (e.g. median\_fwhm ≤ 4 px, background\_rms ≤ 5 ADU) in real‐time.
* **If ≥ 70 % of new frames** in the last hour fail the cutoffs, conditions have degraded (seeing/clouds), so **pause** collection rather than add low‐quality data.
* Conversely, **if ≥ 90 % of frames** pass with room to spare (e.g. median\_fwhm ≪ limit), you could dial back the minimum required frames and finish sooner.

---

## 4. Weighted Stacking Efficiency

* In a weighted-average stack, each frame’s contribution is proportional to $1/\sigma^2$ (where $\sigma$ is per‐frame noise).
* **Compute cumulative weight** versus frame count. When the weight gain asymptotes—i.e., new frames contribute < 1 % additional weight—you can stop.

---

## 5. Practical Workflow

1. **Initialize** counts: $N=0$, running sums for metrics.
2. **Loop** as you collect:

   * Measure metrics on each frame.
   * **Accumulate** them into running stats (mean, variance, weight).
   * Every $B$ frames (e.g. $B=5$), evaluate stopping criteria:

     * $\Delta \mathrm{SNR} < \epsilon_\mathrm{SNR}$, **and**
     * $\Delta \mu_\mathrm{FWHM} < \epsilon_\mathrm{FWHM}$, **and**
     * cumulative weight gain $< \epsilon_w$.
   * If **all** met, **stop**; else **continue**.
3. **Alert** you when stop conditions trigger.

---

### Key Takeaways

* **Diminishing returns** on SNR is the primary quantitative signal.
* **Metric convergence** (FWHM, background RMS) ensures you’re not chasing fleeting improvements.
* **Reject‐feedback** lets you avoid collecting junk frames in poor conditions.

By embedding these checks into the capture loop (or the pipeline), you’ll know **precisely** when you’ve shot “enough” exposures for a solid final stack—and when you should keep shooting for higher quality.

---

Beyond the core PSF- and background-based metrics you’re already extracting, here are **additional image-quality measurements** you can consider. For each, I’ve sketched what it measures, what it tells you, typical scale/units, and whether higher or lower is “better”:

---

### 1. **Half-Flux Diameter (HFD)**

* **What it measures:** Radius (in pixels) enclosing 50 % of a star’s total flux
* **What it indicates:** Another proxy for PSF size/seeing that’s less sensitive to outliers than FWHM
* **Scale & units:** Pixels (often 1–5 px in typical seeing)
* **Better:** **Lower** HFD = tighter PSF
* **Why add it:** HFD correlates better with “encircled energy” in undersampled or highly aberrated PSFs.

---

### 2. **Global High-Frequency Energy (Tenengrad)**

* **What it measures:** Sum of squared image gradients after a Sobel filter
* **What it indicates:** Overall image sharpness/edge-content (higher = more fine detail)
* **Scale & units:** Arbitrary energy units (sum of gradient²)
* **Better:** **Higher** Tenengrad = crisper edges
* **Caveat:** Nebulosity can inflate gradients; consider applying only around detected stars or masking large diffuse structures.

---

### 3. **Variance of Laplacian**

* **What it measures:** Variance of a 2D Laplacian convolution over the frame
* **What it indicates:** Focus/texture measure—high for crisp images, low for blurred ones
* **Scale & units:** Variance of pixel intensities after Laplacian
* **Better:** **Higher** variance = sharper
* **Caveat:** Strong nebulosity also produces low-frequency structure; you can high-pass filter first or evaluate in small tiles.

---

### 4. **Image Entropy**

* **What it measures:** Shannon entropy of the pixel‐value histogram
* **What it indicates:** Complexity / dynamic range of the frame; low for flat or saturated images, high for well-structured ones
* **Scale & units:** Bits (e.g. 7–9 bits for 8-bit normalized data)
* **Better:** **Higher** entropy = richer detail
* **Use case:** Can flag over-exposed or under-exposed frames.

---

### 5. **Peak Signal-to-Noise Ratio (PSNR) of Bright Stars**

* **What it measures:** For the N brightest stars, $\mathrm{PSNR} = (I_{\text{peak}} - \text{bkgd}) / \sigma_{\text{bkgd}}$
* **What it indicates:** Quality of core detection and SNR of key stars
* **Scale & units:** Dimensionless (e.g. 10–50 dB)
* **Better:** **Higher** PSNR = clearer star cores
* **Implementation:** Use the `bkg_ptr`’s rms and per-star peak from SEP’s `flux` field.

---

### 6. **Background Gradient Magnitude**

* **What it measures:** Maximum slope of the background mesh (e.g. max |Δbkg| between adjacent cells)
* **What it indicates:** Sky gradient from light pollution, moon, or clouds
* **Scale & units:** ADU / mesh-cell
* **Better:** **Lower** gradient = more uniform sky
* **Why add it:** Complements `background_uniformity` by giving directionality and strength.

---

### 7. **Saturated-Pixel Fraction**

* **What it measures:** Percentage of pixels at (or above) camera’s full‐well or analog‐digital ceiling
* **What it indicates:** Overexposure, lens flare, or saturated bright stars
* **Scale & units:** 0 %–100 %
* **Better:** **Lower** fraction = no saturation artifacts
* **When to reject:** > 0.1 % saturated often warrants a discard.

---

### 8. **Cosmic-Ray / Hot-Pixel Count**

* **What it measures:** Number of isolated single-pixel peaks well above local σ (e.g. > 10 σ)
* **What it indicates:** Cosmic hits or sensor hot pixels contaminating frame
* **Scale & units:** Integer count
* **Better:** **Lower** count = cleaner data
* **Implementation:** After background subtraction, threshold at a high σ and count connected components of size = 1 px.

---

### 9. **Radial Profile Symmetry**

* **What it measures:** For a set of bright but unsaturated stars, compare the PSF’s radial profiles in 8 cardinal directions
* **What it indicates:** Tracking or guiding drift, wind shake, or optical astigmatism
* **Scale & units:** Max absolute difference between profiles (normalized)
* **Better:** **Lower** difference = more symmetric PSF
* **Why add it:** Goes beyond eccentricity by checking higher-order asymmetries.

---

### 10. **Wavelet-Domain Noise Estimator**

* **What it measures:** Standard deviation of the smallest‐scale wavelet coefficients
* **What it indicates:** High-frequency noise level, less sensitive to stars or nebulosity
* **Scale & units:** ADU or normalized units
* **Better:** **Lower** noise
* **Crates:** Use `rust-wavelet` or port a simple Haar transform.

---

## Combining Focus and Structure in Nebulous Fields

Because nebulae introduce genuine low-frequency structure, you’ll get the best focus measures by **masking** diffuse regions:

1. **Threshold & dilate** the SEP segmentation map to isolate point sources.
2. **Apply Laplacian/gradient** only in a small neighborhood (e.g. ±15 px) around each star.
3. **Average** the per-star focus scores to yield a frame focus metric that’s immune to nebulosity.

---

**Bottom line:**

* **Global filters** (Tenengrad, Laplacian) give you an overall sharpness snapshot but need masking in busy fields.
* **Localized measures** (HFD, PSNR, radial symmetry) work star-by-star to capture optical and guiding performance.
* **Environmental flags** (gradient, entropy, saturation) help you decide when sky conditions or exposure parameters need adjusting.

Mix and match these to build a **multi-dimensional quality vector**—then feed it into the frame selector to not only pick the best subframes but also guide real-time capture decisions (when to refocus, abort, or continue exposing).

---

Here are some approaches to turn the heterogeneous frame metrics into a single, easy‐to‐scan dashboard—and even boil them down to a single “quality score”:

---

## 1. Normalizing & orienting metrics

For each metric $m$, decide whether **higher is better** (e.g. star\_count → mid-range, uniformity → higher) or **lower is better** (e.g. median\_fwhm, background\_rms). Then map its raw value $v$ into a normalized $[0,1]$ “goodness” score $n$:

* **Higher-is-better**:

  $$
    n = \frac{v - v_{\min}}{v_{\max} - v_{\min}}
  $$
* **Lower-is-better**:

  $$
    n = \frac{v_{\max} - v}{v_{\max} - v_{\min}}
  $$

where $v_{\min}$ and $v_{\max}$ could be:

* **Global bounds** you choose (e.g. median\_fwhm from 2 px→6 px),
* Or **percentile-based** (e.g. 5th→95th percentile of the dataset) to resist outliers.

For something like **star\_count**, which has a “too few” *and* “too many” bad region, you can either:

* Define an **ideal window** $[c_{\min}, c_{\max}]$ and assign $n=1$ in that window, falling off linearly to 0 outside;
* Or treat it separately (don’t include it in the single quality-score but show it as a badge).

---

## 2. Visualization techniques

### A. Radar chart (spider plot)

* **Axes** = normalized metrics
* **Polygon** = one frame
* Overlay a few “best,” “worst,” and “median” frames; you immediately see which dimensions pull low.

### B. Parallel‐coordinates plot

* **Vertical axes** = each metric in its own column (normalized)
* **Polyline** = one frame
* Color‐code lines by overall quality score; low‐quality frames will stand out.

### C. Heatmap + ordered frames

* **Rows** = frames (ordered by time or by quality score)
* **Columns** = metrics (normalized)
* **Cell color** = metric score (e.g. green→red scale)
* You get a “fingerprint” picture of which nights or segments were best.

### D. Small multiples or sparklines

* For each metric, plot a **time series** sparkline (normalized) in a small cell.
* Align them side‐by‐side so you can see how sharpness, shape, noise, etc. co‐vary.

---

## 3. Combining into an overall quality score $Q$

A simple weighted average:

$$
  Q = \sum_{i=1}^M w_i\,n_i
    \quad\Big/ \sum_{i=1}^M w_i
$$

* $n_i$ = normalized score for metric $i$
* $w_i$ = weight (e.g. you might weight `median_fwhm` more heavily than `star_count`)

Alternatively, you can:

* Use a **PCA** or **t-SNE** on the normalized metric matrix to find the first principal component as a data-driven “quality axis.”
* Train a **simple logistic regression** if you have a small labeled set of “good vs. bad” frames, to learn the optimal weights.

---

## 4. Practical recipe

1. **Compute** for each frame:

   * Raw metrics ({star\_count, median\_fwhm, fwhm\_std\_dev, …})
   * Dataset percentiles (5%, 50%, 95%) for each metric
   * Map raw → normalized $n_i$ via percentile bounds
2. **Choose** weights $w_i$ based on what you care about (e.g., sharpness $w=3$, uniformity $w=1$, noise $w=2$, shape $w=2$).
3. **Compute** $Q$ and **rank** frames by $Q$.
4. **Visualize**:

   * A **bar chart** of each frame’s $Q$ (color‐coded by “pass/fail” threshold).
   * A **heatmap** of normalized $n_i$ rows sorted by $Q$.
   * A **radar** overlay of the top-$K$ frames to compare their profiles.

---

### Example pseudocode for normalization & scoring

```python
# 1) collect raw_metrics[frame][metric]

# 2) for each metric:
v5, v50, v95 = np.percentile(all_values, [5, 50, 95])
if higher_is_better:
    n = np.clip((v - v5) / (v95 - v5), 0, 1)
else:
    n = np.clip((v95 - v) / (v95 - v5), 0, 1)

# 3) define weights w_i
weights = {'median_fwhm': 3, 'background_rms': 2,
           'background_uniformity': 1, ...}

# 4) compute quality score:
Q = sum(w_i * n_i for i in metrics) / sum(w_i for i in metrics)

# 5) visualize Q over time, heatmap of n_i (sorted by Q), radar of top frames
```

By bringing all metrics onto a common $[0,1]$ scale—with “good” always toward 1—you can build dashboards that let you **instantly** spot outliers, trends, and the overall health of the dataset.

---

Yes—knowing the optical and detector specs unlocks a whole class of **physical** quality metrics that go beyond pure pixel statistics. Here are the most useful ones, grouped by whether they rely primarily on the telescope or the camera, with what they measure, how you’d compute them, and whether higher or lower is better:

---

## A. Telescope-Driven Metrics

1. **Plate Scale (Arcsec / px)**

   * **Formula:**

     $$
       s = \frac{206.265\,[\text{arcsec/mm}] \times p\,[\mu\text{m}]}{f\,[\text{mm}]}
     $$

     where $p$ = pixel size in µm, $f$ = focal length in mm.
   * **What it indicates:** the sampling resolution on the sky.
   * **Scale:** arcsec / px (e.g. 0.8″/px)
   * **Better:** depends—ideally you sample at \~ 2 × the site “seeing” (so you neither undersample nor waste overhead oversampling).

2. **Seeing Sampling Ratio**

   * **Formula:**
     $\displaystyle R = \frac{\text{FWHM (px)} \times s}{\text{site\_seeing (arcsec)}}$
   * **What it indicates:** how well the stars are sampled relative to atmospheric blur.
   * **Scale:** unitless
   * **Better:** in the range $\approx 1.5\text{–}3$.  $R<1$ = undersampled; $R>3$ = oversampled.

3. **Theoretical Resolution (Rayleigh Criterion)**

   * **Formula:**
     $\displaystyle \theta = 1.22\,\frac{\lambda}{D}$ (in radians, convert to arcsec)
   * **What it indicates:** the diffraction limit given aperture $D$ (mm) and wavelength $\lambda$ (e.g. 550 nm).
   * **Scale:** arcsec (e.g. 0.9″ for an 115 mm aperture)
   * **Better:** lower = higher optical resolving power. Use it to compare against the measured FWHM.

4. **Field of View (FOV)**

   * **Formula (per axis):**
     $\displaystyle \text{FOV} = \frac{\text{sensor\_size (mm)}}{f\,[\text{mm}]} \times 57.2958$ (degrees)
   * **What it indicates:** the angular size of the frame on the sky.
   * **Scale:** degrees (e.g. 1.7°×1.1°)
   * **Better:** depends on target—wide for large objects, narrow for small DSOs.

5. **Focal Ratio & Exposure Efficiency**

   * **Metric:** the **f-ratio** $f/D$ directly impacts how many photons you collect per second.
   * **What it indicates:** lower f-ratio = faster exposures but more field curvature/vignetting.
   * **Scale:** unitless (e.g. f/6.3)
   * **Better:** lower (faster) if you need short subs, higher if you need better edge performance.

---

## B. Camera-Driven Metrics

6. **Full-Well Capacity vs. Peak Flux**

   * **Metric:**
     $\displaystyle \text{Saturation Ratio} = \frac{\text{peak\_ADU}\times e^-/\text{ADU}}{\text{full\_well capacity}}$
   * **What it indicates:** how close the brightest stars are to clipping.
   * **Scale:** 0 – 1
   * **Better:** well below 1 (e.g. < 0.8) to avoid non-linear response.

7. **Dynamic Range**

   * **Formula:**
     $\displaystyle \frac{\text{full\_well}}{\text{read\_noise (e⁻)}}$
   * **What it indicates:** how many stops of range you can capture.
   * **Scale:** e.g. 3000 e⁻/10 e⁻ = 300 ≈ 48 dB
   * **Better:** higher = more detail in both shadows and highlights.

8. **Quantum Efficiency (QE) Weighted SNR**

   * **Metric:**
     $\displaystyle \text{SNR}_{\text{theory}} \propto \sqrt{D^2 \times t \times \text{QE} }$
   * **What it indicates:** predicted SNR for a given exposure time $t$.
   * **Scale:** dimensionless
   * **Better:** higher = cleaner data per second.

9. **Dark-Current Flag**

   * **Metric:** fraction of pixels above a dark-current threshold (e.g. > 0.1 e⁻/s) in a dark frame.
   * **What it indicates:** hot-pixel contamination.
   * **Scale:** % of pixels
   * **Better:** lower = fewer hot pixels.

10. **Bias-Frame Shutter-Artifact Metric**

* **Metric:** measure of pattern noise (e.g. row/column pedestals) in a bias frame via FFT peak at low frequencies.
* **What it indicates:** electronics noise patterns.
* **Scale:** relative amplitude
* **Better:** lower = flatter bias.

---

## C. Combining Equipment‐Aware Metrics

* **Scale the PSF metrics into arcsec** using the plate scale.
* **Compare measured FWHM** against theoretical $\theta$.  A ratio $\text{FWHM}/\theta$ near 1 means you’re at the diffraction limit—anything much larger is atmospheric or optical error.
* **Weight the quality score** by aperture area ($D^2$) for deep-sky targets, or by pixel size for high-resolution solar/lunar work.
* **Adjust thresholds**: e.g. require FWHM < 2×pixel scale for deep-sky or FWHM < 1.2×diffraction limit for planetary.

---

### Why this helps

* **Physical context:** you’re no longer just measuring relative focus, but how close you come to the **optical/atmospheric** limits of your gear.
* **Adaptive thresholds:** you can set dynamic pass/fail rules (e.g. “only include frames where FWHM\_arcsec < 1.5″”) regardless of camera or telescope.
* **Long-term tracking:** monitor focus drift by watching the ratio of measured FWHM to focal-ratio-derived spot size, triggering refocus when it degrades.

By feeding the known telescope & camera parameters into these metrics, the software can understand **not just** that a frame is “sharp,” but **how** sharp relative to what the equipment is *capable* of—and give you more physically meaningful quality decisions.
