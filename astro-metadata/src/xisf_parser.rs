//! Parser for XISF file metadata
//!
//! This module provides functions to extract metadata from XISF files
//! and convert it into the AstroMetadata structure.

use std::collections::HashMap;
use std::fs::File;
use std::io::{BufReader, Read, Seek};
use std::path::Path;
use anyhow::{Result, Context};
use chrono::{DateTime, NaiveDateTime, Utc};
use log::warn;

use super::types::{
    AstroMetadata, XisfMetadata, ColorManagement, DisplayFunction, AttachmentInfo
};

/// Extract metadata from an XISF file
pub fn extract_metadata<R: Read + Seek>(reader: &mut R) -> Result<AstroMetadata> {
    let mut metadata = AstroMetadata::default();
    let mut raw_headers = HashMap::new();
    
    // Initialize XISF metadata
    let mut xisf_metadata = XisfMetadata {
        version: "1.0".to_string(),
        creator: None,
        creation_time: None,
        block_alignment: None,
    };
    
    // Read and validate the signature
    let mut signature = [0u8; 8];
    reader.read_exact(&mut signature).context("Failed to read XISF signature")?;
    
    if &signature != b"XISF0100" {
        return Err(anyhow::anyhow!("Invalid XISF signature"));
    }
    
    // Read the header size (4 bytes)
    let mut header_size_bytes = [0u8; 4];
    reader.read_exact(&mut header_size_bytes).context("Failed to read header size")?;
    let header_size = u32::from_le_bytes(header_size_bytes) as usize;
    
    // Extract XML content from the header
    if let Ok(xml_content) = extract_xml_content(reader, header_size) {
        // Extract FITS keywords from the XML
        extract_fits_keywords(&xml_content, &mut metadata, &mut raw_headers);
        
        // Extract other metadata from XML attributes
        extract_xml_attributes(&xml_content, &mut metadata);
        
        // Extract XISF-specific metadata
        extract_xisf_metadata(&xml_content, &mut metadata, &mut xisf_metadata);
        
        // Extract color management information
        extract_color_management(&xml_content, &mut metadata);
        
        // Extract attachment information
        extract_attachments(&xml_content, &mut metadata);
    }
    
    // Store raw headers and XISF metadata
    metadata.raw_headers = raw_headers;
    metadata.xisf = Some(xisf_metadata);
    
    // Calculate session date
    metadata.calculate_session_date();
    
    Ok(metadata)
}

/// Extract metadata from an XISF file path
pub fn extract_metadata_from_path(path: &Path) -> Result<AstroMetadata> {
    let file = File::open(path).context("Failed to open XISF file")?;
    let mut reader = BufReader::new(file);
    extract_metadata(&mut reader)
}

/// Extract XML content from the XISF header
fn extract_xml_content<R: Read>(reader: &mut R, header_size: usize) -> Result<String> {
    // Read the XML header
    let mut header_data = vec![0u8; header_size];
    reader.read_exact(&mut header_data).context("Failed to read XML header")?;
    
    // Find the XML declaration
    let mut xml_start = 0;
    for i in 0..header_data.len() {
        if i + 5 < header_data.len() && &header_data[i..i+5] == b"<?xml" {
            xml_start = i;
            break;
        }
    }
    
    // XISF headers might have null bytes at the end - trim them
    let actual_size = header_data[xml_start..].iter().position(|&b| b == 0)
        .map(|pos| xml_start + pos)
        .unwrap_or(header_data.len());
    
    // Convert to string
    let xml_content = String::from_utf8_lossy(&header_data[xml_start..actual_size]).to_string();
    
    Ok(xml_content)
}

/// Extract FITS keywords from XML content
fn extract_fits_keywords(xml: &str, metadata: &mut AstroMetadata, raw_headers: &mut HashMap<String, String>) {
    let mut pos = 0;
    
    while let Some(start_pos) = xml[pos..].find("<FITSKeyword ") {
        let keyword_start = pos + start_pos;
        
        // Find the end of the FITSKeyword tag
        if let Some(end_pos) = xml[keyword_start..].find("/>") {
            let keyword_end = keyword_start + end_pos + 2;
            let keyword_tag = &xml[keyword_start..keyword_end];
            
            // Extract name and value attributes
            if let Some(name) = extract_attribute(keyword_tag, "name") {
                if let Some(value) = extract_attribute(keyword_tag, "value") {
                    // Remove quotes if present
                    let clean_value = value.trim_matches('\'').to_string();
                    
                    // Store in raw headers
                    raw_headers.insert(name.clone(), clean_value.clone());
                    
                    // Process known FITS keywords
                    process_fits_keyword(metadata, &name, &clean_value);
                }
            }
            
            pos = keyword_end;
        } else {
            break;
        }
    }
}

/// Extract attributes from XML content
fn extract_xml_attributes(xml: &str, metadata: &mut AstroMetadata) {
    // Extract image dimensions
    if let Some(geometry) = extract_attribute(xml, "geometry") {
        let parts: Vec<&str> = geometry.split(':').collect();
        if parts.len() >= 2 {
            metadata.detector.width = parts[0].parse().unwrap_or(0);
            metadata.detector.height = parts[1].parse().unwrap_or(0);
        }
    }
    
    // Extract color space
    if let Some(color_space) = extract_attribute(xml, "colorSpace") {
        if color_space == "Gray" {
            // It's a monochrome image
        }
    }
    
    // Extract sample format
    if let Some(sample_format) = extract_attribute(xml, "sampleFormat") {
        if sample_format == "UInt16" {
            // It's a 16-bit image
        }
    }
    
    // Extract creation time
    if let Some(creation_time) = extract_property_value(xml, "XISF:CreationTime") {
        metadata.exposure.date_obs = parse_date_time(&creation_time);
    }
    
    // Extract creator application
    if let Some(creator_app) = extract_property_value(xml, "XISF:CreatorApplication") {
        if let Some(ref mut env) = metadata.environment {
            env.software_version = Some(creator_app);
        } else {
            let mut env = super::types::Environment::default();
            env.software_version = Some(creator_app);
            metadata.environment = Some(env);
        }
    }
}

/// Extract XISF-specific metadata from XML content
fn extract_xisf_metadata(xml: &str, metadata: &mut AstroMetadata, xisf_metadata: &mut XisfMetadata) {
    // Extract XISF version
    if let Some(version) = extract_attribute(xml, "version") {
        xisf_metadata.version = version;
    }
    
    // Extract creator application
    if let Some(creator_app) = extract_property_value(xml, "XISF:CreatorApplication") {
        xisf_metadata.creator = Some(creator_app);
    }
    
    // Extract creation time
    if let Some(creation_time) = extract_property_value(xml, "XISF:CreationTime") {
        xisf_metadata.creation_time = parse_date_time(&creation_time);
    }
    
    // Extract block alignment
    if let Some(block_alignment) = extract_attribute(xml, "blockAlignment") {
        xisf_metadata.block_alignment = block_alignment.parse::<usize>().ok();
    }
}

/// Extract color management information from XML content
fn extract_color_management(xml: &str, metadata: &mut AstroMetadata) {
    let mut color_management = ColorManagement::default();
    let mut has_color_info = false;
    
    // Extract color space
    if let Some(color_space) = extract_attribute(xml, "colorSpace") {
        color_management.color_space = Some(color_space);
        has_color_info = true;
    }
    
    // Extract ICC profile if present
    if let Some(icc_profile) = extract_property_value(xml, "ICCProfile") {
        // In a real implementation, we would decode the base64 data here
        // For now, we'll just note that it exists
        color_management.icc_profile = Some(Vec::new());
        has_color_info = true;
    }
    
    // Extract display function information
    if let Some(display_function_type) = extract_attribute(xml, "displayFunction") {
        let mut display_function = DisplayFunction::default();
        display_function.function_type = Some(display_function_type);
        
        // Extract display function parameters
        if let Some(params) = extract_attribute(xml, "displayParameters") {
            let param_pairs: Vec<&str> = params.split(';').collect();
            let mut parameters = HashMap::new();
            
            for pair in param_pairs {
                let kv: Vec<&str> = pair.split('=').collect();
                if kv.len() == 2 {
                    if let Ok(value) = kv[1].parse::<f64>() {
                        parameters.insert(kv[0].to_string(), value);
                    }
                }
            }
            
            display_function.parameters = parameters;
        }
        
        color_management.display_function = Some(display_function);
        has_color_info = true;
    }
    
    // Only set color_management if we found any color information
    if has_color_info {
        metadata.color_management = Some(color_management);
    }
}

/// Extract attachment information from XML content
fn extract_attachments(xml: &str, metadata: &mut AstroMetadata) {
    let mut attachments = Vec::new();
    let mut pos = 0;
    
    // Look for Image tags
    while let Some(start_pos) = xml[pos..].find("<Image ") {
        let image_start = pos + start_pos;
        
        // Find the end of the Image tag
        if let Some(end_pos) = xml[image_start..].find(">") {
            let image_end = image_start + end_pos + 1;
            let image_tag = &xml[image_start..image_end];
            
            // Create a new attachment
            let mut attachment = AttachmentInfo::default();
            
            // Extract attachment ID
            if let Some(id) = extract_attribute(image_tag, "id") {
                attachment.id = id;
            } else {
                attachment.id = format!("image{}", attachments.len());
            }
            
            // Extract geometry
            if let Some(geometry) = extract_attribute(image_tag, "geometry") {
                attachment.geometry = geometry;
            }
            
            // Extract sample format
            if let Some(sample_format) = extract_attribute(image_tag, "sampleFormat") {
                attachment.sample_format = sample_format;
            } else {
                attachment.sample_format = "UInt16".to_string(); // Default
            }
            
            // Extract bits per sample
            if let Some(bits_per_sample) = extract_attribute(image_tag, "bitsPerSample") {
                attachment.bits_per_sample = bits_per_sample.parse().unwrap_or(16);
            } else {
                attachment.bits_per_sample = 16; // Default
            }
            
            // Extract compression
            if let Some(compression) = extract_attribute(image_tag, "compression") {
                attachment.compression = Some(compression);
                
                // Extract compression parameters
                if let Some(params) = extract_attribute(image_tag, "compressionParameters") {
                    let param_pairs: Vec<&str> = params.split(';').collect();
                    let mut parameters = HashMap::new();
                    
                    for pair in param_pairs {
                        let kv: Vec<&str> = pair.split('=').collect();
                        if kv.len() == 2 {
                            parameters.insert(kv[0].to_string(), kv[1].to_string());
                        }
                    }
                    
                    attachment.compression_parameters = parameters;
                }
            }
            
            // Extract checksum
            if let Some(checksum_type) = extract_attribute(image_tag, "checksumType") {
                attachment.checksum_type = Some(checksum_type);
                
                if let Some(checksum) = extract_attribute(image_tag, "checksum") {
                    attachment.checksum = Some(checksum);
                }
            }
            
            // Extract resolution information
            if let Some(resolution_x) = extract_attribute(image_tag, "xResolution") {
                attachment.resolution_x = resolution_x.parse::<f64>().ok();
                
                if let Some(resolution_y) = extract_attribute(image_tag, "yResolution") {
                    attachment.resolution_y = resolution_y.parse::<f64>().ok();
                }
                
                if let Some(resolution_unit) = extract_attribute(image_tag, "resolutionUnit") {
                    attachment.resolution_unit = Some(resolution_unit);
                }
            }
            
            // Add the attachment to the list
            attachments.push(attachment);
            
            pos = image_end;
        } else {
            break;
        }
    }
    
    // If we found at least one attachment, update the metadata
    if !attachments.is_empty() {
        metadata.attachments = attachments;
    }
}

/// Process a FITS keyword and update metadata
fn process_fits_keyword(metadata: &mut AstroMetadata, name: &str, value: &str) {
    match name {
        // Equipment information
        "TELESCOP" => metadata.equipment.telescope_name = Some(value.to_string()),
        "FOCALLEN" => metadata.equipment.focal_length = value.parse().ok(),
        "APERTURE" => metadata.equipment.aperture = value.parse().ok(),
        "FOCRATIO" => metadata.equipment.focal_ratio = value.parse().ok(),
        
        // Detector information
        "INSTRUME" | "CAMERA" => metadata.detector.camera_name = Some(value.to_string()),
        "XPIXSZ" | "PIXSIZE" => metadata.detector.pixel_size = value.parse().ok(),
        "XBINNING" => metadata.detector.binning_x = value.parse().unwrap_or(1),
        "YBINNING" => metadata.detector.binning_y = value.parse().unwrap_or(1),
        "GAIN" | "EGAIN" => metadata.detector.gain = value.parse().ok(),
        "RDNOISE" => metadata.detector.read_noise = value.parse().ok(),
        "CCD-TEMP" | "CCDTEMP" => metadata.detector.temperature = value.parse().ok(),
        "SET-TEMP" => metadata.detector.temp_setpoint = value.parse().ok(),
        
        // Filter information
        "FILTER" => metadata.filter.name = Some(value.to_string()),
        
        // Exposure information
        "OBJECT" => metadata.exposure.object_name = Some(value.to_string()),
        "RA" | "OBJCTRA" => {
            // Handle both numeric and sexagesimal formats
            if let Ok(ra) = value.parse::<f32>() {
                metadata.exposure.ra = Some(ra as f64);
            } else {
                // Try to parse sexagesimal format (HH MM SS)
                if let Some(ra_deg) = parse_sexagesimal(value) {
                    metadata.exposure.ra = Some(ra_deg * 15.0); // Convert hours to degrees
                }
            }
        },
        "DEC" | "OBJCTDEC" => {
            // Handle both numeric and sexagesimal formats
            if let Ok(dec) = value.parse::<f32>() {
                metadata.exposure.dec = Some(dec as f64);
            } else {
                // Try to parse sexagesimal format (DD MM SS)
                if let Some(dec_deg) = parse_sexagesimal(value) {
                    metadata.exposure.dec = Some(dec_deg);
                }
            }
        },
        "DATE-OBS" => metadata.exposure.date_obs = parse_date_time(value),
        "EXPTIME" | "EXPOSURE" => metadata.exposure.exposure_time = value.parse().ok(),
        "IMAGETYP" | "FRAME" => metadata.exposure.frame_type = Some(value.to_string()),
        
        // Mount information
        "PIERSIDE" => {
            if let Some(ref mut mount) = metadata.mount {
                mount.pier_side = Some(value.to_string());
            } else {
                let mut mount = super::types::Mount::default();
                mount.pier_side = Some(value.to_string());
                metadata.mount = Some(mount);
            }
        },
        
        // Environment information
        "AMB_TEMP" | "AMBTEMP" => {
            if let Some(ref mut env) = metadata.environment {
                env.ambient_temp = value.parse().ok();
            } else {
                let mut env = super::types::Environment::default();
                env.ambient_temp = value.parse().ok();
                metadata.environment = Some(env);
            }
        },
        "HUMIDITY" => {
            if let Some(ref mut env) = metadata.environment {
                env.humidity = value.parse().ok();
            } else {
                let mut env = super::types::Environment::default();
                env.humidity = value.parse().ok();
                metadata.environment = Some(env);
            }
        },
        
        // WCS information
        "CRPIX1" => {
            if let Some(ref mut wcs) = metadata.wcs {
                wcs.crpix1 = value.parse().ok();
            } else {
                let mut wcs = super::types::WcsData::default();
                wcs.crpix1 = value.parse().ok();
                metadata.wcs = Some(wcs);
            }
        },
        "CRPIX2" => {
            if let Some(ref mut wcs) = metadata.wcs {
                wcs.crpix2 = value.parse().ok();
            } else {
                let mut wcs = super::types::WcsData::default();
                wcs.crpix2 = value.parse().ok();
                metadata.wcs = Some(wcs);
            }
        },
        
        // Observatory location
        "SITELAT" | "OBSLAT" => {
            if let Some(ref mut mount) = metadata.mount {
                mount.latitude = value.parse().ok();
            } else {
                let mut mount = super::types::Mount::default();
                mount.latitude = value.parse().ok();
                metadata.mount = Some(mount);
            }
        },
        "SITELONG" | "OBSLONG" => {
            if let Some(ref mut mount) = metadata.mount {
                mount.longitude = value.parse().ok();
            } else {
                let mut mount = super::types::Mount::default();
                mount.longitude = value.parse().ok();
                metadata.mount = Some(mount);
            }
        },
        "SITEELEV" | "OBSELEV" => {
            if let Some(ref mut mount) = metadata.mount {
                mount.height = value.parse().ok();
            } else {
                let mut mount = super::types::Mount::default();
                mount.height = value.parse().ok();
                metadata.mount = Some(mount);
            }
        },
        
        // Detector information
        "OFFSET" | "CCDOFFST" => metadata.detector.offset = value.parse().ok(),
        "READOUT" | "READOUTM" => metadata.detector.readout_mode = Some(value.to_string()),
        "USBLIMIT" | "USBTRFC" => metadata.detector.usb_limit = Some(value.to_string()),
        "ROTANG" | "ROTPA" | "ROTATANG" => metadata.detector.rotator_angle = value.parse().ok(),
        
        // Equipment information
        "FOCPOS" | "FOCUSPOS" => metadata.equipment.focuser_position = value.parse().ok(),
        "FOCTEMP" | "FOCUSTEMP" => metadata.equipment.focuser_temperature = value.parse().ok(),
        
        // Mount information
        "PEAKRA" | "PEAKRAER" => {
            if let Some(ref mut mount) = metadata.mount {
                mount.peak_ra_error = value.parse().ok();
            } else {
                let mut mount = super::types::Mount::default();
                mount.peak_ra_error = value.parse().ok();
                metadata.mount = Some(mount);
            }
        },
        "PEAKDEC" | "PEAKDCER" => {
            if let Some(ref mut mount) = metadata.mount {
                mount.peak_dec_error = value.parse().ok();
            } else {
                let mut mount = super::types::Mount::default();
                mount.peak_dec_error = value.parse().ok();
                metadata.mount = Some(mount);
            }
        },
        
        // Environment information
        "SQM" | "SQMMAG" | "SKYQUAL" => {
            if let Some(ref mut env) = metadata.environment {
                env.sqm = value.parse().ok();
            } else {
                let mut env = super::types::Environment::default();
                env.sqm = value.parse().ok();
                metadata.environment = Some(env);
            }
        },
        
        // Exposure information
        "PROJECT" | "PROJNAME" => metadata.exposure.project_name = Some(value.to_string()),
        "SESSIONID" | "SESSID" => metadata.exposure.session_id = Some(value.to_string()),
        
        // Ignore other keywords
        _ => {}
    }
}

/// Extract an attribute value from XML content
fn extract_attribute(xml: &str, attr_name: &str) -> Option<String> {
    let search_pattern = format!("{}=\"", attr_name);
    
    if let Some(start_pos) = xml.find(&search_pattern) {
        let start = start_pos + search_pattern.len();
        if let Some(end_pos) = xml[start..].find('"') {
            return Some(xml[start..start+end_pos].to_string());
        }
    }
    
    None
}

/// Extract a property value from XML content
fn extract_property_value(xml: &str, property_id: &str) -> Option<String> {
    let search_pattern = format!("id=\"{}\" type=\"", property_id);
    
    if let Some(start_pos) = xml.find(&search_pattern) {
        // Find the closing > of the Property tag
        if let Some(tag_end) = xml[start_pos..].find(">") {
            let tag_end_pos = start_pos + tag_end + 1;
            
            // Find the closing </Property> tag
            if let Some(end_tag_pos) = xml[tag_end_pos..].find("</Property>") {
                let value_end = tag_end_pos + end_tag_pos;
                return Some(xml[tag_end_pos..value_end].trim().to_string());
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