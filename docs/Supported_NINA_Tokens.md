# Supported NINA Path Tokens

This document lists all NINA path tokens that are supported by the astro-metadata library.

| Token | Available | Source in astro_metadata |
|-------|-----------|--------------------------|
| `$$DATE$$` | ✅ | `exposure.date_obs` (date part) |
| `$$DATEMINUS12$$` | ✅ | `exposure.session_date` |
| `$$DATETIME$$` | ✅ | `exposure.date_obs` (formatted) |
| `$$DATEUTC$$` | ✅ | `exposure.date_obs` (date part) |
| `$$TIME$$` | ✅ | `exposure.date_obs` (time part) |
| `$$TIMEUTC$$` | ✅ | `exposure.date_obs` (time part) |
| `$$BINNING$$` | ✅ | `detector.binning_x` and `detector.binning_y` |
| `$$CAMERA$$` | ✅ | `detector.camera_name` |
| `$$GAIN$$` | ✅ | `detector.gain` |
| `$$OFFSET$$` | ✅ | `detector.offset` |
| `$$READOUTMODES$$` | ✅ | `detector.readout_mode` |
| `$$SENSORTEMP$$` | ✅ | `detector.temperature` |
| `$$TEMPERATURESETPOINT$$` | ✅ | `detector.temp_setpoint` |
| `$$USBLIMIT$$` | ✅ | `detector.usb_limit` |
| `$$EXPOSURETIME$$` | ✅ | `exposure.exposure_time` |
| `$$FRAMENR$$` | ✅ | `exposure.frame_number` |
| `$$IMAGETYPE$$` | ✅ | `exposure.frame_type` |
| `$$SEQUENCETITLE$$` | ✅ | `exposure.sequence_id` |
| `$$TARGETNAME$$` | ✅ | `exposure.object_name` |
| `$$FILTER$$` | ✅ | `filter.name` |
| `$$FOCUSERPOSITION$$` | ✅ | `equipment.focuser_position` |
| `$$FOCUSERTEMP$$` | ✅ | `equipment.focuser_temperature` |
| `$$PEAKDEC$$` | ✅ | `mount.peak_dec_error` |
| `$$PEAKRA$$` | ✅ | `mount.peak_ra_error` |
| `$$RMS$$` | ✅ | `mount.guide_rms` |
| `$$ROTATORANGLE$$` | ✅ | `detector.rotator_angle` |
| `$$SQM$$` | ✅ | `environment.sqm` |
| `$$TELESCOPE$$` | ✅ | `equipment.telescope_name` |
| `$$TSPROJECTNAME$$` | ✅ | `exposure.project_name` |
| `$$TSSESSIONID$$` | ✅ | `exposure.session_id` |

## Not Available Tokens

| Token | Available | Reason |
|-------|-----------|--------|
| `$$APPLICATIONSTARTDATE$$` | ❌ | Application-specific, not in image metadata |
| `$$ECCENTRICITY$$` | ❌ | Calculated by Hocus Focus, not in metadata |
| `$$FWHM$$` | ❌ | Calculated by Hocus Focus, not in metadata |
| `$$HFR$$` | ❌ | Calculated value, not in metadata |
| `$$STARCOUNT$$` | ❌ | Calculated value, not in metadata |
| `$$PEAKDECARCSEC$$` | ❌ | Would need to be calculated from `mount.peak_dec_error` and plate scale |
| `$$PEAKRAARCSEC$$` | ❌ | Would need to be calculated from `mount.peak_ra_error` and plate scale |
| `$$RMSARCSEC$$` | ❌ | Would need to be calculated from `mount.guide_rms` and plate scale |