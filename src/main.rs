mod device;
mod hid;
mod models;
mod mouse;
mod utils;

pub const REPORT_ID: u8 = 0x08;

fn main() {
    let mouse = mouse::Mouse::new();
    let battery_level = mouse.get_battery();

    let config =
        mouse::MouseConfig::new(&mouse.device).expect("Failed to read mouse configuration");

    println!(
        "Current DPI: {} - DPI values: {:?}",
        config.dpi_values[config.current_dpi_index as usize].value,
        config.dpi_values
    );
    println!(
        "Battery level: {}% - is charging: {:?}",
        battery_level.level, battery_level.charging
    );
}
