//! Metadata handling for astronomical images

pub mod fits_parser;
pub mod types;
pub mod xisf_parser;

pub use astro_io::fits::FitsHeaderCard;
pub use types::AstroMetadata;
