# File Organization Tokens

This document lists the NINA path tokens used in the standard file organization pattern and their availability in astro-metadata.

## Standard File Organization Pattern

```
$$TARGETNAME$$\$$CAMERA$$\SESSION_$$DATEMINUS12$$\TELESCOPE_$$TELESCOPE$$\BIN_$$BINNING$$\GAIN_$$GAIN$$\$$FILTER$$\$$IMAGETYPE$$\DATETIME_$$DATETIME$$__IMAGETYPE_$$IMAGETYPE$$__FILTER_$$FILTER$$__EXPOSURE_$$EXPOSURETIME$$s__BIN_$$BINNING$$__GAIN_$$GAIN$$__FRAMENR_$$FRAMENR$$
```

## Token Availability

| Token | Available | Source in astro_metadata |
|-------|-----------|--------------------------|
| `$$TARGETNAME$$` | ✅ | `exposure.object_name` |
| `$$CAMERA$$` | ✅ | `detector.camera_name` |
| `$$DATEMINUS12$$` | ✅ | `exposure.session_date` |
| `$$TELESCOPE$$` | ✅ | `equipment.telescope_name` |
| `$$BINNING$$` | ✅ | `detector.binning_x` and `detector.binning_y` |
| `$$GAIN$$` | ✅ | `detector.gain` |
| `$$FILTER$$` | ✅ | `filter.name` |
| `$$IMAGETYPE$$` | ✅ | `exposure.frame_type` |
| `$$DATETIME$$` | ✅ | `exposure.date_obs` (formatted) |
| `$$EXPOSURETIME$$` | ✅ | `exposure.exposure_time` |
| `$$FRAMENR$$` | ✅ | `exposure.frame_number` |

All tokens used in the standard file organization pattern are fully supported by the astro-metadata library.