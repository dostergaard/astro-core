// astro-core/src/lib.rs
//! Core libraries for astronomical image processing and analysis.
//!
//! This crate provides a collection of tools for working with astronomical images,
//! organized into three main modules:
//!
//! - [`io`]: File I/O operations for astronomical image formats (FITS, XISF)
//! - [`metadata`]: Metadata extraction and handling for astronomical images
//! - [`metrics`]: Statistical analysis and quality metrics for astronomical images
//!
//! # Examples
//!
//! ```no_run
//! use astro_core::{io, metadata, metrics};
//!
//! fn main() -> Result<(), Box<dyn std::error::Error>> {
//!     // Load a FITS file
//!     let loader = io::fits::FitsLoader::new("/path/to/image.fits")?;
//!     let image_data = loader.read_image_data()?;
//!     let metadata = loader.read_metadata()?;
//!     
//!     // Analyze star metrics
//!     let star_metrics = metrics::star_metrics::StarMetrics::new(&image_data)?;
//!     let stars = star_metrics.detect_stars()?;
//!     
//!     println!("Found {} stars", stars.len());
//!     
//!     Ok(())
//! }
//! ```

pub use astro_io as io;
pub use astro_metadata as metadata;
pub use astro_metrics as metrics;
