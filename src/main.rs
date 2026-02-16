mod device;
mod models;
mod hid;
mod mouse;

pub const REPORT_ID: u8 = 0x08;

fn main() {
    let mouse = mouse::Mouse::new();
    let battery_level = mouse.get_battery();

    println!("Battery level: {}% - is charging: {:?}", battery_level.level, battery_level.charging);
}
