//! FITS-style coordinate keyword projection.
//!
//! These helpers preserve `RA`/`DEC` and `OBJCTRA`/`OBJCTDEC` as independent
//! source-identified pairs. They intentionally do not infer what either pair
//! means for a particular producer or observing workflow.

use log::warn;
use std::collections::HashMap;

use super::types::{CoordinatePair, Exposure, HeaderCoordinatePairs};

pub(crate) fn populate_header_coordinates(
    exposure: &mut Exposure,
    headers: &HashMap<String, String>,
) {
    let header_coordinates = HeaderCoordinatePairs {
        ra_dec: CoordinatePair {
            ra: parse_ra_degrees(headers),
            dec: parse_dec_degrees(headers),
        },
        objctra_objctdec: CoordinatePair {
            ra: parse_objctra_degrees(headers),
            dec: parse_objctdec_degrees(headers),
        },
    };

    // Retain the established convenience behavior while exposing the source
    // pairs above for consumers that require unambiguous provenance.
    exposure.ra = header_coordinates
        .ra_dec
        .ra
        .or(header_coordinates.objctra_objctdec.ra);
    exposure.dec = header_coordinates
        .ra_dec
        .dec
        .or(header_coordinates.objctra_objctdec.dec);
    exposure.header_coordinates = header_coordinates;
}

fn parse_ra_degrees(headers: &HashMap<String, String>) -> Option<f64> {
    let value = get_header_value(headers, "RA")?;
    if let Some(ra_degrees) = value.parse::<f64>().ok().filter(is_valid_ra_degrees) {
        Some(ra_degrees)
    } else {
        warn!("Ignoring invalid decimal-degree RA value: {value}");
        None
    }
}

fn parse_objctra_degrees(headers: &HashMap<String, String>) -> Option<f64> {
    let value = get_header_value(headers, "OBJCTRA")?;
    let hours = value
        .parse::<f64>()
        .ok()
        .or_else(|| parse_sexagesimal(value));
    let ra_degrees = hours.map(|hours| hours * 15.0)?;

    if is_valid_ra_degrees(&ra_degrees) {
        Some(ra_degrees)
    } else {
        warn!("Ignoring invalid hour-angle OBJCTRA value: {value}");
        None
    }
}

fn parse_dec_degrees(headers: &HashMap<String, String>) -> Option<f64> {
    let value = get_header_value(headers, "DEC")?;
    parse_declination_degrees(value, "DEC")
}

fn parse_objctdec_degrees(headers: &HashMap<String, String>) -> Option<f64> {
    let value = get_header_value(headers, "OBJCTDEC")?;
    let dec_degrees = value
        .parse::<f64>()
        .ok()
        .or_else(|| parse_sexagesimal(value));
    match dec_degrees.filter(is_valid_dec_degrees) {
        Some(dec_degrees) => Some(dec_degrees),
        None => {
            warn!("Ignoring invalid OBJCTDEC value: {value}");
            None
        }
    }
}

fn parse_declination_degrees(value: &str, keyword: &str) -> Option<f64> {
    match value.parse::<f64>().ok().filter(is_valid_dec_degrees) {
        Some(dec_degrees) => Some(dec_degrees),
        None => {
            warn!("Ignoring invalid decimal-degree {keyword} value: {value}");
            None
        }
    }
}

fn get_header_value<'a>(headers: &'a HashMap<String, String>, key: &str) -> Option<&'a str> {
    headers.get(key).map(String::as_str).or_else(|| {
        headers
            .iter()
            .find(|(header_key, _)| header_key.eq_ignore_ascii_case(key))
            .map(|(_, value)| value.as_str())
    })
}

fn is_valid_ra_degrees(value: &f64) -> bool {
    value.is_finite() && (0.0..360.0).contains(value)
}

fn is_valid_dec_degrees(value: &f64) -> bool {
    value.is_finite() && (-90.0..=90.0).contains(value)
}

/// Parse sexagesimal hours or degrees into a decimal value.
pub fn parse_sexagesimal(value: &str) -> Option<f64> {
    let parts: Vec<&str> = value.split_whitespace().collect();
    if parts.len() >= 3 {
        if let (Ok(h), Ok(m), Ok(s)) = (
            parts[0].parse::<f64>(),
            parts[1].parse::<f64>(),
            parts[2].parse::<f64>(),
        ) {
            let sign = if h < 0.0 || value.starts_with('-') {
                -1.0
            } else {
                1.0
            };
            return Some(sign * (h.abs() + m / 60.0 + s / 3600.0));
        }
    }
    None
}
