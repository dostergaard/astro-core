# Changelog

All notable changes to the Astro Core project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.2.0] - 2025-05-31

### Added
- Enhanced quality metrics with improved scoring algorithms
- Kron radius and AUTO flux calculations using SEP functions
- Logarithmic SNR scoring for better perceptual representation
- FWHM consistency score to detect uneven focus
- Elongation metric for more intuitive star shape assessment
- Comprehensive unit tests for all crates
- Documentation for quality metrics

### Changed
- Improved background scoring to combine uniformity with noise level
- Refactored metrics to use only data available from SEP
- Updated API to be more consistent and intuitive
- Fixed deprecated method calls in chrono library usage

### Fixed
- Type conversion issues in SEP function calls
- Removed unused imports and variables
- Fixed floating point precision issues in tests
