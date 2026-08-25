use anyhow::{bail, Context, Result};
use ravensky_astro::metadata::{fits_parser, xisf_parser, AstroMetadata};
use std::env;
use std::ffi::{OsStr, OsString};
use std::fs::{self, File, OpenOptions};
use std::io::{self, BufRead, Write};
use std::path::{Path, PathBuf};
use walkdir::WalkDir;

const MAX_FAILURES_TO_DISPLAY: usize = 10;

pub fn run() -> Result<()> {
    let raw_args: Vec<OsString> = env::args_os().collect();
    let program = raw_args
        .first()
        .map(|arg| arg.to_string_lossy().into_owned())
        .unwrap_or_else(|| "sensor_temperature_qa".to_owned());

    let command = parse_args(raw_args)?;
    let options = match command {
        Command::Help => {
            println!("{}", usage(&program));
            return Ok(());
        }
        Command::Run(options) => {
            resolve_options(options, &mut io::stdin().lock(), &mut io::stdout())?
        }
    };

    run_with_options(&options)
}

#[derive(Debug, PartialEq)]
enum Command {
    Help,
    Run(CliInputs),
}

#[derive(Debug, Default, PartialEq)]
struct CliInputs {
    input_path: Option<PathBuf>,
    output_path: Option<PathBuf>,
    max_difference: Option<f32>,
}

#[derive(Debug)]
struct Options {
    input_path: PathBuf,
    output_path: PathBuf,
    max_difference: f32,
}

#[derive(Debug, Default)]
struct Discovery {
    image_paths: Vec<PathBuf>,
    permission_skips: Vec<PathBuf>,
}

#[derive(Debug, Clone, Copy, PartialEq)]
struct TemperatureReading {
    ccd_temp: f32,
    set_temp: f32,
    difference: f32,
}

#[derive(Debug)]
struct Candidate {
    source: PathBuf,
    destination: PathBuf,
    temperature: TemperatureReading,
}

#[derive(Debug)]
struct FileFailure {
    path: PathBuf,
    message: String,
}

#[derive(Debug, Default)]
struct EvaluationSummary {
    evaluated: usize,
    missing_temperatures: usize,
    candidates: Vec<Candidate>,
    failures: Vec<FileFailure>,
}

fn parse_args<I>(args: I) -> Result<Command>
where
    I: IntoIterator<Item = OsString>,
{
    let mut positional_args = Vec::new();

    for arg in args.into_iter().skip(1) {
        match arg.to_str() {
            Some("-h" | "--help") => return Ok(Command::Help),
            Some(flag) if flag.starts_with('-') => bail!("Unknown option: {flag}"),
            _ => positional_args.push(arg),
        }
    }

    if positional_args.len() > 3 {
        bail!("Expected at most three positional values: input-dir output-dir max-difference");
    }

    let max_difference = positional_args
        .get(2)
        .map(|value| parse_max_difference(&value.to_string_lossy()))
        .transpose()?;

    Ok(Command::Run(CliInputs {
        input_path: positional_args.first().map(PathBuf::from),
        output_path: positional_args.get(1).map(PathBuf::from),
        max_difference,
    }))
}

fn usage(program: &str) -> String {
    format!(
        "Usage: {program} [input-dir] [output-dir] [max-difference]\n\
         \nSearches input-dir recursively for FITS and XISF files. Files whose absolute\n\
         CCD-TEMP minus SET-TEMP difference is greater than max-difference are previewed\n\
         and, after confirmation, moved to output-dir with their relative paths preserved.\n\
         \nMissing positional values are requested interactively."
    )
}

fn resolve_options<R: BufRead, W: Write>(
    inputs: CliInputs,
    reader: &mut R,
    writer: &mut W,
) -> Result<Options> {
    let input_path = match inputs.input_path {
        Some(path) => path,
        None => PathBuf::from(prompt_text(reader, writer, "Input directory")?),
    };
    let output_path = match inputs.output_path {
        Some(path) => path,
        None => PathBuf::from(prompt_text(reader, writer, "Output directory")?),
    };
    let max_difference = match inputs.max_difference {
        Some(value) => value,
        None => loop {
            let value = prompt_text(reader, writer, "Maximum temperature difference")?;
            match parse_max_difference(&value) {
                Ok(value) => break value,
                Err(err) => writeln!(writer, "{err}")?,
            }
        },
    };

    validate_options(input_path, output_path, max_difference)
}

fn prompt_text<R: BufRead, W: Write>(
    reader: &mut R,
    writer: &mut W,
    label: &str,
) -> Result<String> {
    loop {
        write!(writer, "{label}: ")?;
        writer.flush()?;
        let mut value = String::new();
        if reader.read_line(&mut value)? == 0 {
            bail!("Input ended before {label} was provided");
        }

        let value = strip_matching_quotes(value.trim());
        if !value.is_empty() {
            return Ok(value.to_owned());
        }
        writeln!(writer, "{label} cannot be empty.")?;
    }
}

fn strip_matching_quotes(value: &str) -> &str {
    let bytes = value.as_bytes();
    if bytes.len() >= 2
        && matches!(
            (bytes[0], bytes[bytes.len() - 1]),
            (b'\'', b'\'') | (b'"', b'"')
        )
    {
        &value[1..value.len() - 1]
    } else {
        value
    }
}

fn parse_max_difference(value: &str) -> Result<f32> {
    let value: f32 = value.parse().with_context(|| {
        format!("Maximum temperature difference must be a decimal number, got {value:?}")
    })?;
    if !value.is_finite() || value < 0.0 {
        bail!(
            "Maximum temperature difference must be a finite value greater than or equal to zero"
        );
    }
    Ok(value)
}

fn validate_options(
    input_path: PathBuf,
    output_path: PathBuf,
    max_difference: f32,
) -> Result<Options> {
    if !input_path.is_dir() {
        bail!("Input path is not a directory: {}", input_path.display());
    }

    let input_path = input_path.canonicalize().with_context(|| {
        format!(
            "Failed to resolve input directory: {}",
            input_path.display()
        )
    })?;
    let output_path = absolute_path(&output_path)?;

    if output_path == input_path || output_path.starts_with(&input_path) {
        bail!(
            "Output directory must not be the input directory or a directory within it: {}",
            output_path.display()
        );
    }

    Ok(Options {
        input_path,
        output_path,
        max_difference,
    })
}

fn absolute_path(path: &Path) -> Result<PathBuf> {
    if path.is_absolute() {
        Ok(path.to_path_buf())
    } else {
        Ok(env::current_dir()
            .context("Failed to determine the current directory")?
            .join(path))
    }
}

fn run_with_options(options: &Options) -> Result<()> {
    println!(
        "Discovering FITS and XISF files under {}...",
        options.input_path.display()
    );
    let discovery = discover_images(&options.input_path)?;
    println!("Found {} supported file(s).", discovery.image_paths.len());

    let mut summary = EvaluationSummary::default();
    for (index, path) in discovery.image_paths.iter().enumerate() {
        print!(
            "\rEvaluating file {}/{}",
            index + 1,
            discovery.image_paths.len()
        );
        io::stdout().flush()?;
        evaluate_path(path, options, &mut summary);
    }
    if !discovery.image_paths.is_empty() {
        println!();
    }

    print_summary(&summary, &discovery.permission_skips);
    if summary.candidates.is_empty() {
        return Ok(());
    }

    if !prompt_to_proceed(&mut io::stdin().lock(), &mut io::stdout())? {
        println!("No files were moved.");
        return Ok(());
    }

    move_candidates(&summary.candidates)
}

fn discover_images(root: &Path) -> Result<Discovery> {
    let mut discovery = Discovery::default();
    for entry in WalkDir::new(root).follow_links(false) {
        match entry {
            Ok(entry) if entry.file_type().is_file() && is_supported_image_path(entry.path()) => {
                discovery.image_paths.push(entry.into_path());
            }
            Ok(_) => {}
            Err(error) if is_walkdir_permission_denied(&error) => {
                if let Some(path) = error.path() {
                    discovery.permission_skips.push(path.to_path_buf());
                }
            }
            Err(error) => return Err(error).context("Failed while discovering image files"),
        }
    }
    discovery.image_paths.sort_unstable();
    discovery.permission_skips.sort_unstable();
    discovery.permission_skips.dedup();
    Ok(discovery)
}

fn is_supported_image_path(path: &Path) -> bool {
    matches!(
        path.extension()
            .and_then(OsStr::to_str)
            .map(str::to_ascii_lowercase)
            .as_deref(),
        Some("fits" | "fit" | "fts" | "xisf")
    )
}

fn is_walkdir_permission_denied(error: &walkdir::Error) -> bool {
    error
        .io_error()
        .is_some_and(|error| error.kind() == io::ErrorKind::PermissionDenied)
}

fn evaluate_path(path: &Path, options: &Options, summary: &mut EvaluationSummary) {
    match extract_metadata_for_path(path) {
        Ok(metadata) => {
            summary.evaluated += 1;
            match evaluate_temperatures(
                metadata.detector.temperature,
                metadata.detector.temp_setpoint,
                options.max_difference,
            ) {
                Some(temperature) => {
                    match destination_path(path, &options.input_path, &options.output_path) {
                        Ok(destination) if destination.exists() => {
                            summary.failures.push(FileFailure {
                                path: path.to_path_buf(),
                                message: format!(
                                    "destination already exists: {}",
                                    destination.display()
                                ),
                            })
                        }
                        Ok(destination) => summary.candidates.push(Candidate {
                            source: path.to_path_buf(),
                            destination,
                            temperature,
                        }),
                        Err(error) => summary.failures.push(FileFailure {
                            path: path.to_path_buf(),
                            message: error.to_string(),
                        }),
                    }
                }
                None => {
                    summary.missing_temperatures += usize::from(
                        metadata.detector.temperature.is_none()
                            || metadata.detector.temp_setpoint.is_none(),
                    )
                }
            }
        }
        Err(error) => summary.failures.push(FileFailure {
            path: path.to_path_buf(),
            message: error.to_string(),
        }),
    }
}

fn extract_metadata_for_path(path: &Path) -> Result<AstroMetadata> {
    match path
        .extension()
        .and_then(OsStr::to_str)
        .map(str::to_ascii_lowercase)
        .as_deref()
    {
        Some("fits" | "fit" | "fts") => fits_parser::extract_metadata_from_path(path),
        Some("xisf") => xisf_parser::extract_metadata_from_path(path),
        _ => bail!("Unsupported image format: {}", path.display()),
    }
}

fn evaluate_temperatures(
    ccd_temp: Option<f32>,
    set_temp: Option<f32>,
    max_difference: f32,
) -> Option<TemperatureReading> {
    let (ccd_temp, set_temp) = (ccd_temp?, set_temp?);
    let difference = (ccd_temp - set_temp).abs();
    (difference > max_difference).then_some(TemperatureReading {
        ccd_temp,
        set_temp,
        difference,
    })
}

fn destination_path(source: &Path, input_root: &Path, output_root: &Path) -> Result<PathBuf> {
    let relative_path = source.strip_prefix(input_root).with_context(|| {
        format!(
            "Source path is outside input directory: {}",
            source.display()
        )
    })?;
    Ok(output_root.join(relative_path))
}

fn print_summary(summary: &EvaluationSummary, permission_skips: &[PathBuf]) {
    println!("\nTemperature QA summary:");
    println!("  Metadata evaluated: {}", summary.evaluated);
    println!(
        "  Missing CCD-TEMP or SET-TEMP: {}",
        summary.missing_temperatures
    );
    println!("  Files selected for moving: {}", summary.candidates.len());

    for candidate in &summary.candidates {
        println!(
            "  {} -> {} (CCD-TEMP {:.3}, SET-TEMP {:.3}, difference {:.3})",
            candidate.source.display(),
            candidate.destination.display(),
            candidate.temperature.ccd_temp,
            candidate.temperature.set_temp,
            candidate.temperature.difference
        );
    }

    print_failures("Skipped files", &summary.failures);
    if !permission_skips.is_empty() {
        eprintln!(
            "Skipped {} path(s) due to permissions.",
            permission_skips.len()
        );
    }
}

fn print_failures(label: &str, failures: &[FileFailure]) {
    if failures.is_empty() {
        return;
    }
    eprintln!("{label}: {}.", failures.len());
    for failure in failures.iter().take(MAX_FAILURES_TO_DISPLAY) {
        eprintln!("  {}: {}", failure.path.display(), failure.message);
    }
    if failures.len() > MAX_FAILURES_TO_DISPLAY {
        eprintln!(
            "  ... and {} more",
            failures.len() - MAX_FAILURES_TO_DISPLAY
        );
    }
}

fn prompt_to_proceed<R: BufRead, W: Write>(reader: &mut R, writer: &mut W) -> Result<bool> {
    write!(writer, "Proceed with moving these files? [Y/n] ")?;
    writer.flush()?;
    let mut response = String::new();
    if reader.read_line(&mut response)? == 0 {
        return Ok(false);
    }
    Ok(matches!(
        response.trim().to_ascii_lowercase().as_str(),
        "" | "y" | "yes"
    ))
}

fn move_candidates(candidates: &[Candidate]) -> Result<()> {
    let mut moved = 0;
    let mut failures = Vec::new();
    for (index, candidate) in candidates.iter().enumerate() {
        print!("\rMoving file {}/{}", index + 1, candidates.len());
        io::stdout().flush()?;
        let result = candidate
            .destination
            .parent()
            .context("Destination path has no parent directory")
            .and_then(|parent| {
                fs::create_dir_all(parent).context("Failed to create destination directory")
            })
            .and_then(|_| {
                if candidate.destination.exists() {
                    bail!(
                        "Destination already exists: {}",
                        candidate.destination.display()
                    );
                }
                move_file(&candidate.source, &candidate.destination)
            });
        match result {
            Ok(()) => moved += 1,
            Err(error) => failures.push(FileFailure {
                path: candidate.source.clone(),
                message: error.to_string(),
            }),
        }
    }
    println!();
    println!("Moved {moved} of {} selected file(s).", candidates.len());
    print_failures("Files not moved", &failures);
    Ok(())
}

fn move_file(source: &Path, destination: &Path) -> Result<()> {
    move_file_with_rename(source, destination, |source, destination| {
        fs::rename(source, destination)
    })
}

fn move_file_with_rename<F>(source: &Path, destination: &Path, rename: F) -> Result<()>
where
    F: FnOnce(&Path, &Path) -> io::Result<()>,
{
    match rename(source, destination) {
        Ok(()) => Ok(()),
        Err(error) if error.kind() == io::ErrorKind::CrossesDevices => {
            copy_file_then_remove_source(source, destination).with_context(|| {
                format!(
                    "Cross-volume move from {} to {} failed after rename was unavailable",
                    source.display(),
                    destination.display()
                )
            })
        }
        Err(error) => Err(error).with_context(|| {
            format!(
                "Failed to move {} to {} with rename",
                source.display(),
                destination.display()
            )
        }),
    }
}

fn copy_file_then_remove_source(source: &Path, destination: &Path) -> Result<()> {
    let mut source_file = File::open(source)
        .with_context(|| format!("Failed to open source file: {}", source.display()))?;
    let mut destination_file = OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(destination)
        .with_context(|| {
            format!(
                "Failed to create destination file without overwriting: {}",
                destination.display()
            )
        })?;
    let copy_result = io::copy(&mut source_file, &mut destination_file)
        .with_context(|| {
            format!(
                "Failed to copy {} to {}",
                source.display(),
                destination.display()
            )
        })
        .and_then(|_| {
            destination_file.sync_all().with_context(|| {
                format!(
                    "Failed to flush copied destination file: {}",
                    destination.display()
                )
            })
        });

    if let Err(error) = copy_result {
        drop(destination_file);
        if let Err(cleanup_error) = fs::remove_file(destination) {
            return Err(error).context(format!(
                "Also failed to remove incomplete destination file {}: {cleanup_error}",
                destination.display()
            ));
        }
        return Err(error);
    }
    drop(destination_file);

    fs::remove_file(source).with_context(|| {
        format!(
            "Copied {} but failed to remove the original source file",
            source.display()
        )
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Cursor;
    use std::time::{SystemTime, UNIX_EPOCH};

    #[test]
    fn selects_only_temperature_differences_strictly_above_the_limit() {
        let reading = evaluate_temperatures(Some(-9.4), Some(-10.0), 0.5).unwrap();
        assert_eq!(reading.ccd_temp, -9.4);
        assert_eq!(reading.set_temp, -10.0);
        assert!((reading.difference - 0.6).abs() < 0.000_01);
        assert_eq!(evaluate_temperatures(Some(-9.5), Some(-10.0), 0.5), None);
        assert_eq!(evaluate_temperatures(Some(-10.2), Some(-10.0), 0.5), None);
    }

    #[test]
    fn skips_files_without_both_temperature_values() {
        assert_eq!(evaluate_temperatures(None, Some(-10.0), 0.5), None);
        assert_eq!(evaluate_temperatures(Some(-9.0), None, 0.5), None);
    }

    #[test]
    fn parse_args_accepts_partial_inputs_for_interactive_completion() {
        let command = parse_args(vec![OsString::from("qa"), OsString::from("input")]).unwrap();
        assert_eq!(
            command,
            Command::Run(CliInputs {
                input_path: Some(PathBuf::from("input")),
                output_path: None,
                max_difference: None,
            })
        );
    }

    #[test]
    fn parses_decimal_maximum_difference_and_rejects_invalid_values() {
        assert_eq!(parse_max_difference("0.25").unwrap(), 0.25);
        assert!(parse_max_difference("-0.1").is_err());
        assert!(parse_max_difference("nan").is_err());
    }

    #[test]
    fn recognizes_supported_extensions_case_insensitively() {
        assert!(is_supported_image_path(Path::new("frame.FITS")));
        assert!(is_supported_image_path(Path::new("frame.xisf")));
        assert!(is_supported_image_path(Path::new("frame.FtS")));
        assert!(!is_supported_image_path(Path::new("frame.tiff")));
    }

    #[test]
    fn preserves_source_relative_path_in_destination() {
        assert_eq!(
            destination_path(
                Path::new("/input/Lights/M31/frame.fit"),
                Path::new("/input"),
                Path::new("/output"),
            )
            .unwrap(),
            PathBuf::from("/output/Lights/M31/frame.fit")
        );
    }

    #[test]
    fn confirmation_defaults_to_yes_and_accepts_no() {
        let mut output = Vec::new();
        assert!(prompt_to_proceed(&mut Cursor::new("\n"), &mut output).unwrap());
        assert!(!prompt_to_proceed(&mut Cursor::new("n\n"), &mut output).unwrap());
    }

    #[test]
    fn interactive_paths_accept_matching_shell_style_quotes() {
        let mut output = Vec::new();
        assert_eq!(
            prompt_text(
                &mut Cursor::new("'/Volumes/ap_projects/!1stage/REJECTED'\n"),
                &mut output,
                "Input directory"
            )
            .unwrap(),
            "/Volumes/ap_projects/!1stage/REJECTED"
        );
    }

    #[test]
    fn moving_a_candidate_creates_parent_directories_and_preserves_the_file() -> Result<()> {
        let unique_id = SystemTime::now().duration_since(UNIX_EPOCH)?.as_nanos();
        let root = env::temp_dir().join(format!("sensor_temperature_qa_test_{unique_id}"));
        let source = root.join("input/Lights/M31/frame.fit");
        let destination = root.join("output/Lights/M31/frame.fit");
        fs::create_dir_all(source.parent().unwrap())?;
        fs::write(&source, b"test image")?;

        let result = move_candidates(&[Candidate {
            source: source.clone(),
            destination: destination.clone(),
            temperature: TemperatureReading {
                ccd_temp: -9.0,
                set_temp: -10.0,
                difference: 1.0,
            },
        }]);

        assert!(result.is_ok());
        assert!(!source.exists());
        assert_eq!(fs::read(&destination)?, b"test image");
        fs::remove_dir_all(root)?;
        Ok(())
    }

    #[test]
    fn cross_volume_rename_falls_back_to_copy_then_removes_the_source() -> Result<()> {
        let unique_id = SystemTime::now().duration_since(UNIX_EPOCH)?.as_nanos();
        let root = env::temp_dir().join(format!("sensor_temperature_qa_test_{unique_id}"));
        let source = root.join("input/frame.fit");
        let destination = root.join("output/frame.fit");
        fs::create_dir_all(source.parent().unwrap())?;
        fs::create_dir_all(destination.parent().unwrap())?;
        fs::write(&source, b"cross-volume test image")?;

        move_file_with_rename(&source, &destination, |_, _| {
            Err(io::Error::from(io::ErrorKind::CrossesDevices))
        })?;

        assert!(!source.exists());
        assert_eq!(fs::read(&destination)?, b"cross-volume test image");
        fs::remove_dir_all(root)?;
        Ok(())
    }
}
