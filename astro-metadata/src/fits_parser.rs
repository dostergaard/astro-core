//! Parser for FITS file headers
//!
//! This module provides functions to extract metadata from FITS file headers
//! and convert it into the AstroMetadata structure.

use std::collections::HashMap;
use std::fs::File;
use std::path::Path;
use anyhow::{Result, Context};
use chrono::{DateTime, NaiveDateTime, Utc};
use fitsio::FitsFile;
use log::warn;

use super::types::{
    AstroMetadata, Equipment, Detector, Filter, Exposure, Mount, Environment, WcsData
};

/// Extract metadata from a FITS file path
pub fn extract_metadata_from_path(path: &Path) -> Result<AstroMetadata> {
    let mut fits_file = FitsFile::open(path).context("Failed to open FITS file")?;
    extract_metadata(&mut fits_file)
}

/// Extract metadata from a FITS file
pub fn extract_metadata(fits_file: &mut FitsFile) -> Result<AstroMetadata> {
    let hdu = fits_file.primary_hdu()?;
    let mut metadata = AstroMetadata::default();
    let mut raw_headers = HashMap::new();
    
    // Extract common FITS header keywords that we're interested in
    let keywords = [
        "TELESCOP", "FOCALLEN", "APERTURE", "INSTRUME", "CAMERA",
        "PIXSIZE", "XPIXSZ", "NAXIS1", "NAXIS2", "XBINNING", "YBINNING",
        "GAIN", "EGAIN", "RDNOISE", "CCD-TEMP", "CCDTEMP", "SET-TEMP",
        "FILTER", "OBJECT", "RA", "OBJCTRA", "DEC", "OBJCTDEC",
        "DATE-OBS", "EXPTIME", "EXPOSURE", "IMAGETYP", "FRAME"
    ];
    
    // Read each keyword
    for keyword in &keywords {
        if let Ok(value) = hdu.read_key::<String>(fits_file, keyword) {
            raw_headers.insert(keyword.to_string(), value);
        }
    }
    
    // Parse equipment information
    parse_equipment(&mut metadata.equipment, &raw_headers);
    
    // Parse detector information
    parse_detector(&mut metadata.detector, &raw_headers, &hdu.info);
    
    // Parse filter information
    parse_filter(&mut metadata.filter, &raw_headers);
    
    // Parse exposure information
    parse_exposure(&mut metadata.exposure, &raw_headers);
    
    // Parse mount information
    metadata.mount = parse_mount(&raw_headers);
    
    // Parse environment information
    metadata.environment = parse_environment(&raw_headers);
    
    // Parse WCS information
    metadata.wcs = parse_wcs(&raw_headers);
    
    // Store raw headers for any fields we didn't explicitly parse
    metadata.raw_headers = raw_headers;
    
    // Calculate session date
    metadata.calculate_session_date();
    
    Ok(metadata)
}

/// Parse equipment information from FITS headers
fn parse_equipment(equipment: &mut Equipment, headers: &HashMap<String, String>) {
    equipment.telescope_name = get_string_header(headers, &["TELESCOP"]);
    equipment.focal_length = get_float_header(headers, &["FOCALLEN"]);
    equipment.aperture = get_float_header(headers, &["APERTURE"]);
    
    // Calculate focal ratio if not directly available
    if equipment.focal_ratio.is_none() {
        if let (Some(focal_length), Some(aperture)) = (equipment.focal_length, equipment.aperture) {
            if aperture > 0.0 {
                equipment.focal_ratio = Some(focal_length / aperture);
            }
        }
    }
    
    // Try to extract reducer/flattener info from INSTRUME
    if let Some(instrume) = get_string_header(headers, &["INSTRUME"]) {
        if instrume.contains("reducer") || instrume.contains("flattener") {
            equipment.reducer_flattener = Some(instrume);
        }
    }
    
    equipment.mount_model = get_string_header(headers, &["MOUNT"]);
    
    // Focuser information
    equipment.focuser_position = get_int_header(headers, &["FOCPOS", "FOCUSPOS"]);
    equipment.focuser_temperature = get_float_header(headers, &["FOCTEMP", "FOCUSTEMP"]);
}

/// Parse detector information from FITS headers
fn parse_detector(detector: &mut Detector, headers: &HashMap<String, String>, hdu_info: &fitsio::hdu::HduInfo) {
    detector.camera_name = get_string_header(headers, &["INSTRUME", "CAMERA"]);
    detector.pixel_size = get_float_header(headers, &["PIXSIZE", "XPIXSZ"]);
    
    // Get dimensions from NAXIS1/NAXIS2 headers
    if let Some(naxis1) = get_int_header(headers, &["NAXIS1"]) {
        detector.width = naxis1 as usize;
    }
    
    if let Some(naxis2) = get_int_header(headers, &["NAXIS2"]) {
        detector.height = naxis2 as usize;
    }
    
    // If dimensions are not in headers, try to get them from HDU info
    if detector.width == 0 || detector.height == 0 {
        if let fitsio::hdu::HduInfo::ImageInfo { shape, .. } = hdu_info {
            if shape.len() >= 2 {
                // FITS standard: first dimension is y (height), second is x (width)
                detector.height = shape[0] as usize;
                detector.width = shape[1] as usize;
            }
        }
    }
    
    // Binning
    detector.binning_x = get_int_header(headers, &["XBINNING"]).unwrap_or(1) as usize;
    detector.binning_y = get_int_header(headers, &["YBINNING"]).unwrap_or(1) as usize;
    
    // Camera settings
    detector.gain = get_float_header(headers, &["GAIN", "EGAIN"]);
    detector.offset = get_int_header(headers, &["OFFSET", "CCDOFFST"]);
    detector.readout_mode = get_string_header(headers, &["READOUT", "READOUTM"]);
    detector.usb_limit = get_string_header(headers, &["USBLIMIT", "USBTRFC"]);
    detector.read_noise = get_float_header(headers, &["RDNOISE"]);
    detector.temperature = get_float_header(headers, &["CCD-TEMP", "CCDTEMP"]);
    detector.temp_setpoint = get_float_header(headers, &["CCD-TEMP-SETPOINT", "SET-TEMP"]);
    detector.cooler_power = get_float_header(headers, &["COOL-PWR", "COOLPWR"]);
    detector.cooler_status = get_string_header(headers, &["COOL-STAT", "COOLSTAT"]);
    detector.rotator_angle = get_float_header(headers, &["ROTANG", "ROTPA", "ROTATANG"]);
}

/// Parse filter information from FITS headers
fn parse_filter(filter: &mut Filter, headers: &HashMap<String, String>) {
    filter.name = get_string_header(headers, &["FILTER"]);
    
    // Try to get filter position
    if let Some(pos_str) = get_string_header(headers, &["FILTERID", "FLTPOS"]) {
        if let Ok(pos) = pos_str.parse::<usize>() {
            filter.position = Some(pos);
        }
    }
    
    // Filter wavelength is rarely in FITS headers, but we'll check anyway
    filter.wavelength = get_float_header(headers, &["WAVELENG", "WAVELEN"]);
}

/// Parse exposure information from FITS headers
fn parse_exposure(exposure: &mut Exposure, headers: &HashMap<String, String>) {
    exposure.object_name = get_string_header(headers, &["OBJECT"]);
    
    // Parse coordinates
    exposure.ra = get_float_header(headers, &["RA", "OBJCTRA"]).map(|ra| ra as f64 * 15.0); // Convert hours to degrees
    exposure.dec = get_float_header(headers, &["DEC", "OBJCTDEC"]).map(|dec| dec as f64);
    
    // Parse date/time
    if let Some(date_str) = get_string_header(headers, &["DATE-OBS"]) {
        exposure.date_obs = parse_date_time(&date_str);
    }
    
    // Exposure time
    exposure.exposure_time = get_float_header(headers, &["EXPTIME", "EXPOSURE"]);
    
    // Frame type
    exposure.frame_type = get_string_header(headers, &["IMAGETYP", "FRAME"]);
    
    // Sequence information
    exposure.sequence_id = get_string_header(headers, &["SEQID", "SEQFILE"]);
    
    if let Some(frame_num_str) = get_string_header(headers, &["FRAMENUM", "SEQNUM"]) {
        if let Ok(frame_num) = frame_num_str.parse::<usize>() {
            exposure.frame_number = Some(frame_num);
        }
    }
    
    // Dither offsets
    exposure.dither_offset_x = get_float_header(headers, &["DX", "DITHX"]);
    exposure.dither_offset_y = get_float_header(headers, &["DY", "DITHY"]);
    
    // Scheduler information
    exposure.project_name = get_string_header(headers, &["PROJECT", "PROJNAME"]);
    exposure.session_id = get_string_header(headers, &["SESSIONID", "SESSID"]);
}

/// Parse mount information from FITS headers
fn parse_mount(headers: &HashMap<String, String>) -> Option<Mount> {
    // Check if we have any mount information
    if !headers.contains_key("PIERSIDE") && 
       !headers.contains_key("MFLIP") && 
       !headers.contains_key("GUIDERMS") &&
       !headers.contains_key("SITELAT") &&
       !headers.contains_key("OBSLAT") {
        return None;
    }
    
    let mut mount = Mount::default();
    
    mount.pier_side = get_string_header(headers, &["PIERSIDE"]);
    
    // Parse meridian flip
    if let Some(mflip_str) = get_string_header(headers, &["MFLIP", "MFOC"]) {
        mount.meridian_flip = Some(mflip_str.to_lowercase() == "true" || mflip_str == "1");
    }
    
    // Observatory location
    mount.latitude = get_float_header(headers, &["SITELAT", "OBSLAT"]).map(|v| v as f64);
    mount.longitude = get_float_header(headers, &["SITELONG", "OBSLONG"]).map(|v| v as f64);
    mount.height = get_float_header(headers, &["SITEELEV", "OBSELEV"]).map(|v| v as f64);
    
    mount.guide_camera = get_string_header(headers, &["GUIDECAM"]);
    mount.guide_rms = get_float_header(headers, &["GUIDERMS"]);
    mount.guide_scale = get_float_header(headers, &["GUIDESCALE"]);
    
    // Parse dither enabled
    if let Some(dither_str) = get_string_header(headers, &["DITHER"]) {
        mount.dither_enabled = Some(dither_str.to_lowercase() == "true" || dither_str == "1");
    }
    
    // Peak guiding errors
    mount.peak_ra_error = get_float_header(headers, &["PEAKRA", "PEAKRAER"]);
    mount.peak_dec_error = get_float_header(headers, &["PEAKDEC", "PEAKDCER"]);
    
    Some(mount)
}

/// Parse environment information from FITS headers
fn parse_environment(headers: &HashMap<String, String>) -> Option<Environment> {
    // Check if we have any environment information
    if !headers.contains_key("AMB_TEMP") && 
       !headers.contains_key("HUMIDITY") && 
       !headers.contains_key("NINA-VERSION") && 
       !headers.contains_key("EKOS-VERSION") &&
       !headers.contains_key("SQM") {
        return None;
    }
    
    let mut env = Environment::default();
    
    env.ambient_temp = get_float_header(headers, &["AMB_TEMP", "AMBTEMP"]);
    env.humidity = get_float_header(headers, &["HUMIDITY"]);
    env.dew_heater_power = get_float_header(headers, &["DEWPOWER", "DEWPWR"]);
    env.voltage = get_float_header(headers, &["VOLTAGE", "SYSVOLT"]);
    env.current = get_float_header(headers, &["CURRENT", "SYSCURR"]);
    env.sqm = get_float_header(headers, &["SQM", "SQMMAG", "SKYQUAL"]);
    
    // Software version
    if let Some(nina_ver) = get_string_header(headers, &["NINA-VERSION"]) {
        env.software_version = Some(format!("NINA {}", nina_ver));
    } else if let Some(ekos_ver) = get_string_header(headers, &["EKOS-VERSION"]) {
        env.software_version = Some(format!("EKOS {}", ekos_ver));
    } else if let Some(software) = get_string_header(headers, &["SWCREATE", "SOFTWARE"]) {
        env.software_version = Some(software);
    }
    
    Some(env)
}

/// Parse WCS information from FITS headers
fn parse_wcs(headers: &HashMap<String, String>) -> Option<WcsData> {
    // Check if we have any WCS information
    if !headers.contains_key("CRPIX1") && 
       !headers.contains_key("CRPIX2") && 
       !headers.contains_key("CRVAL1") && 
       !headers.contains_key("CRVAL2") {
        return None;
    }
    
    let mut wcs = WcsData::default();
    
    // Reference pixel coordinates
    wcs.crpix1 = get_float_header(headers, &["CRPIX1"]).map(|v| v as f64);
    wcs.crpix2 = get_float_header(headers, &["CRPIX2"]).map(|v| v as f64);
    
    // Reference pixel values (usually RA/DEC in degrees)
    wcs.crval1 = get_float_header(headers, &["CRVAL1"]).map(|v| v as f64);
    wcs.crval2 = get_float_header(headers, &["CRVAL2"]).map(|v| v as f64);
    
    // CD matrix elements (transformation matrix)
    wcs.cd1_1 = get_float_header(headers, &["CD1_1"]).map(|v| v as f64);
    wcs.cd1_2 = get_float_header(headers, &["CD1_2"]).map(|v| v as f64);
    wcs.cd2_1 = get_float_header(headers, &["CD2_1"]).map(|v| v as f64);
    wcs.cd2_2 = get_float_header(headers, &["CD2_2"]).map(|v| v as f64);
    
    // Coordinate system
    wcs.ctype1 = get_string_header(headers, &["CTYPE1"]);
    wcs.ctype2 = get_string_header(headers, &["CTYPE2"]);
    
    // Note: If we need to store equinox information, we would need to add
    // an equinox field to the WcsData struct
    
    Some(wcs)
}

/// Helper function to get a string value from headers
fn get_string_header(headers: &HashMap<String, String>, keys: &[&str]) -> Option<String> {
    for key in keys {
        if let Some(value) = headers.get(*key) {
            if !value.is_empty() {
                return Some(value.clone());
            }
        }
    }
    None
}

/// Helper function to get a float value from headers
fn get_float_header(headers: &HashMap<String, String>, keys: &[&str]) -> Option<f32> {
    for key in keys {
        if let Some(value) = headers.get(*key) {
            if let Ok(float_val) = value.parse::<f32>() {
                return Some(float_val);
            }
        }
    }
    None
}

/// Helper function to get an integer value from headers
fn get_int_header(headers: &HashMap<String, String>, keys: &[&str]) -> Option<i32> {
    for key in keys {
        if let Some(value) = headers.get(*key) {
            if let Ok(int_val) = value.parse::<i32>() {
                return Some(int_val);
            }
        }
    }
    None
}

/// Parse sexagesimal format (HH MM SS or DD MM SS) to decimal degrees
fn parse_sexagesimal(value: &str) -> Option<f64> {
    let parts: Vec<&str> = value.split_whitespace().collect();
    if parts.len() >= 3 {
        if let (Ok(h), Ok(m), Ok(s)) = (parts[0].parse::<f64>(), parts[1].parse::<f64>(), parts[2].parse::<f64>()) {
            let sign = if h < 0.0 || value.starts_with('-') { -1.0 } else { 1.0 };
            return Some(sign * (h.abs() + m / 60.0 + s / 3600.0));
        }
    }
    None
}

/// Helper function to parse date/time strings
fn parse_date_time(date_str: &str) -> Option<DateTime<Utc>> {
    // Try different date formats
    let formats = [
        "%Y-%m-%dT%H:%M:%S%.fZ",   // ISO 8601 with Z suffix
        "%Y-%m-%dT%H:%M:%SZ",      // ISO 8601 with Z suffix, no fractional seconds
        "%Y-%m-%dT%H:%M:%S%.f",    // ISO 8601 with fractional seconds
        "%Y-%m-%dT%H:%M:%S",       // ISO 8601 without fractional seconds
        "%Y-%m-%d %H:%M:%S%.f",    // Space-separated with fractional seconds
        "%Y-%m-%d %H:%M:%S",       // Space-separated without fractional seconds
    ];
    
    for format in &formats {
        if let Ok(dt) = NaiveDateTime::parse_from_str(date_str, format) {
            return Some(DateTime::from_naive_utc_and_offset(dt, Utc));
        }
    }
    
    warn!("Failed to parse date string: {}", date_str);
    None
}
