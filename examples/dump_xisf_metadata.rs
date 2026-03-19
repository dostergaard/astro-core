use astro_metadata::xisf_parser::extract_metadata_from_path;

#[path = "shared/metadata_dump.rs"]
mod metadata_dump;

fn main() {
    metadata_dump::run_metadata_dump(
        "XISF",
        "<xisf_file_path>",
        "Raw XISF FITS Keywords",
        extract_metadata_from_path,
    );
}
