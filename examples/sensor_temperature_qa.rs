#[path = "shared/sensor_temperature_qa.rs"]
mod sensor_temperature_qa;

fn main() {
    if let Err(error) = sensor_temperature_qa::run() {
        eprintln!("Error: {error:#}");
        std::process::exit(1);
    }
}
