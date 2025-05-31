//! Type definitions for astronomical metadata
//!
//! This module defines the structures used to represent metadata from
//! astronomical image files, including equipment information, detector
//! settings, filters, exposure details, and more.

use std::collections::HashMap;
use chrono::{DateTime, Utc};
use serde::Serialize;

/// Core metadata structure with nested components for astronomical images
#[derive(Debug, Clone, Default, Serialize)]
pub struct AstroMetadata {
    /// Equipment information
    pub equipment: Equipment,
    /// Detector and camera settings
    pub detector: Detector,
    /// Filter information
    pub filter: Filter,
    /// Exposure and timing information
    pub exposure: Exposure,
    /// Mount and guiding information
    pub mount: Option<Mount>,
    /// Environmental data
    pub environment: Option<Environment>,
    /// World Coordinate System data
    pub wcs: Option<WcsData>,
    /// XISF-specific metadata
    pub xisf: Option<XisfMetadata>,
    /// Color management information
    pub color_management: Option<ColorManagement>,
    /// Image attachments (for multi-image files)
    pub attachments: Vec<AttachmentInfo>,
    /// Raw header values for any fields not explicitly parsed
    pub raw_headers: HashMap<String, String>,
}

/// Equipment information
#[derive(Debug, Clone, Default, Serialize)]
pub struct Equipment {
    /// Telescope make/model
    pub telescope_name: Option<String>,
    /// Focal length in mm
    pub focal_length: Option<f32>,
    /// Aperture/diameter in mm
    pub aperture: Option<f32>,
    /// Focal ratio (f/D)
    pub focal_ratio: Option<f32>,
    /// Reducer/flattener information
    pub reducer_flattener: Option<String>,
    /// Mount model
    pub mount_model: Option<String>,
    /// Focuser position
    pub focuser_position: Option<i32>,
    /// Focuser temperature in °C
    pub focuser_temperature: Option<f32>,
}

/// Detector and camera settings
#[derive(Debug, Clone, Default, Serialize)]
pub struct Detector {
    /// Camera make/model
    pub camera_name: Option<String>,
    /// Pixel size in μm
    pub pixel_size: Option<f32>,
    /// Sensor width in pixels
    pub width: usize,
    /// Sensor height in pixels
    pub height: usize,
    /// Binning in X direction
    pub binning_x: usize,
    /// Binning in Y direction
    pub binning_y: usize,
    /// Gain in e-/ADU
    pub gain: Option<f32>,
    /// Camera offset value
    pub offset: Option<i32>,
    /// Camera readout mode
    pub readout_mode: Option<String>,
    /// USB limit setting (speed or traffic)
    pub usb_limit: Option<String>,
    /// Read noise in e-
    pub read_noise: Option<f32>,
    /// Full well capacity in e-
    pub full_well: Option<f32>,
    /// Sensor temperature in °C
    pub temperature: Option<f32>,
    /// Temperature setpoint in °C
    pub temp_setpoint: Option<f32>,
    /// Cooler power in %
    pub cooler_power: Option<f32>,
    /// Cooler status
    pub cooler_status: Option<String>,
    /// Rotator angle in degrees
    pub rotator_angle: Option<f32>,
}

/// Filter information
#[derive(Debug, Clone, Default, Serialize)]
pub struct Filter {
    /// Filter name
    pub name: Option<String>,
    /// Filter position/slot
    pub position: Option<usize>,
    /// Filter wavelength in nm
    pub wavelength: Option<f32>,
}

/// Exposure and timing information
#[derive(Debug, Clone, Default, Serialize)]
pub struct Exposure {
    /// Object/target name
    pub object_name: Option<String>,
    /// Right ascension in degrees
    pub ra: Option<f64>,
    /// Declination in degrees
    pub dec: Option<f64>,
    /// Observation date/time (UTC)
    pub date_obs: Option<DateTime<Utc>>,
    /// Session date (calculated from date_obs by subtracting 12 hours)
    pub session_date: Option<DateTime<Utc>>,
    /// Exposure time in seconds
    pub exposure_time: Option<f32>,
    /// Frame type (LIGHT, DARK, BIAS, FLAT)
    pub frame_type: Option<String>,
    /// Sequence identifier
    pub sequence_id: Option<String>,
    /// Frame number in sequence
    pub frame_number: Option<usize>,
    /// Dither offset in X direction
    pub dither_offset_x: Option<f32>,
    /// Dither offset in Y direction
    pub dither_offset_y: Option<f32>,
    /// Project name (for scheduler)
    pub project_name: Option<String>,
    /// Session identifier
    pub session_id: Option<String>,
}

/// Mount and guiding information
#[derive(Debug, Clone, Default, Serialize)]
pub struct Mount {
    /// Mount side of pier (EAST, WEST)
    pub pier_side: Option<String>,
    /// Whether a meridian flip occurred
    pub meridian_flip: Option<bool>,
    /// Observatory latitude in degrees (+ north, - south)
    pub latitude: Option<f64>,
    /// Observatory longitude in degrees (+ east, - west)
    pub longitude: Option<f64>,
    /// Observatory height above sea level in meters
    pub height: Option<f64>,
    /// Guide camera information
    pub guide_camera: Option<String>,
    /// Guide RMS error
    pub guide_rms: Option<f32>,
    /// Guide camera plate scale
    pub guide_scale: Option<f32>,
    /// Whether dithering was enabled
    pub dither_enabled: Option<bool>,
    /// Peak RA guiding error during exposure in pixels
    pub peak_ra_error: Option<f32>,
    /// Peak DEC guiding error during exposure in pixels
    pub peak_dec_error: Option<f32>,
}

/// Environmental data
#[derive(Debug, Clone, Default, Serialize)]
pub struct Environment {
    /// Ambient temperature in °C
    pub ambient_temp: Option<f32>,
    /// Humidity in %
    pub humidity: Option<f32>,
    /// Dew heater power in %
    pub dew_heater_power: Option<f32>,
    /// System voltage in V
    pub voltage: Option<f32>,
    /// System current in A
    pub current: Option<f32>,
    /// Software version information
    pub software_version: Option<String>,
    /// Plugin information
    pub plugin_info: Option<String>,
    /// Sky Quality Meter reading in mag/arcsec²
    pub sqm: Option<f32>,
}

/// World Coordinate System data
#[derive(Debug, Clone, Default, Serialize)]
pub struct WcsData {
    /// Coordinate type for axis 1
    pub ctype1: Option<String>,
    /// Coordinate type for axis 2
    pub ctype2: Option<String>,
    /// Reference pixel for axis 1
    pub crpix1: Option<f64>,
    /// Reference pixel for axis 2
    pub crpix2: Option<f64>,
    /// Reference value for axis 1
    pub crval1: Option<f64>,
    /// Reference value for axis 2
    pub crval2: Option<f64>,
    /// CD matrix element 1_1
    pub cd1_1: Option<f64>,
    /// CD matrix element 1_2
    pub cd1_2: Option<f64>,
    /// CD matrix element 2_1
    pub cd2_1: Option<f64>,
    /// CD matrix element 2_2
    pub cd2_2: Option<f64>,
    /// Rotation angle
    pub crota2: Option<f64>,
    /// Airmass
    pub airmass: Option<f32>,
    /// Altitude in degrees
    pub altitude: Option<f32>,
    /// Azimuth in degrees
    pub azimuth: Option<f32>,
}

/// XISF-specific metadata
#[derive(Debug, Clone, Default, Serialize)]
pub struct XisfMetadata {
    /// XISF format version
    pub version: String,
    /// Creator application
    pub creator: Option<String>,
    /// Creation timestamp
    pub creation_time: Option<DateTime<Utc>>,
    /// Block alignment size
    pub block_alignment: Option<usize>,
}

/// Color management information
#[derive(Debug, Clone, Default, Serialize)]
pub struct ColorManagement {
    /// Color space (RGB, Gray, etc.)
    pub color_space: Option<String>,
    /// ICC profile data
    pub icc_profile: Option<Vec<u8>>,
    /// Display function parameters
    pub display_function: Option<DisplayFunction>,
}

/// Display function parameters
#[derive(Debug, Clone, Default, Serialize)]
pub struct DisplayFunction {
    /// Display function type
    pub function_type: Option<String>,
    /// Parameters for the display function
    pub parameters: HashMap<String, f64>,
}

/// Information about an image attachment
#[derive(Debug, Clone, Default, Serialize)]
pub struct AttachmentInfo {
    /// Attachment identifier
    pub id: String,
    /// Image geometry (width:height:channels)
    pub geometry: String,
    /// Sample data type
    pub sample_format: String,
    /// Bits per sample
    pub bits_per_sample: usize,
    /// Compression algorithm
    pub compression: Option<String>,
    /// Compression parameters
    pub compression_parameters: HashMap<String, String>,
    /// Checksum type
    pub checksum_type: Option<String>,
    /// Checksum value
    pub checksum: Option<String>,
    /// Physical resolution in x direction (pixels per unit)
    pub resolution_x: Option<f64>,
    /// Physical resolution in y direction (pixels per unit)
    pub resolution_y: Option<f64>,
    /// Resolution unit
    pub resolution_unit: Option<String>,
}

impl AstroMetadata {
    /// Check if we have enough information to calculate plate scale
    pub fn can_calculate_plate_scale(&self) -> bool {
        self.equipment.focal_length.is_some() && self.detector.pixel_size.is_some()
    }
    
    /// Calculate plate scale in arcsec/pixel
    pub fn plate_scale(&self) -> Option<f32> {
        if let (Some(focal_length), Some(pixel_size)) = (self.equipment.focal_length, self.detector.pixel_size) {
            // Plate scale in arcsec/pixel = (pixel size in μm / focal length in mm) * 206.265
            Some((pixel_size / focal_length) * 206.265)
        } else {
            None
        }
    }
    
    /// Calculate field of view in arcminutes
    pub fn field_of_view(&self) -> Option<(f32, f32)> {
        if let Some(plate_scale) = self.plate_scale() {
            let width_arcmin = (self.detector.width as f32 * plate_scale) / 60.0;
            let height_arcmin = (self.detector.height as f32 * plate_scale) / 60.0;
            Some((width_arcmin, height_arcmin))
        } else {
            None
        }
    }
    
    /// Calculate approximate time zone offset in hours from longitude
    fn approximate_timezone_from_longitude(&self) -> Option<i32> {
        self.mount.as_ref()
            .and_then(|mount| mount.longitude)
            .map(|longitude| (longitude / 15.0).round() as i32)
    }
    
    /// Calculate the session date using location information if available
    pub fn calculate_session_date(&mut self) {
        if let Some(date_obs) = self.exposure.date_obs {
            // Default to UTC time
            let mut local_time = date_obs;
            
            // If we have longitude information, adjust for approximate local time
            if let Some(tz_offset) = self.approximate_timezone_from_longitude() {
                local_time = date_obs + chrono::Duration::hours(tz_offset as i64);
            }
            
            // Get the date at noon (12:00) on the same day in the adjusted time
            let noon = local_time.date().and_hms_opt(12, 0, 0).unwrap();
            
            // If the adjusted observation time is before noon, use the previous day's noon
            self.exposure.session_date = if local_time < noon {
                Some(noon - chrono::Duration::days(1))
            } else {
                Some(noon)
            };
        }
    }
}