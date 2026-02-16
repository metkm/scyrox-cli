mod device;
mod hid;
mod models;
mod mouse;
mod utils;

use clap::Parser;

pub const REPORT_ID: u8 = 0x08;

#[derive(Parser, Debug)]
struct Args {
    #[arg(short, long)]
    dpi_values: bool,

    #[arg(short, long)]
    current_dpi: bool,

    #[arg(short, long)]
    battery_level: bool,

    #[arg(short, long, default_value_t = false)]
    is_charging: bool,
}

fn main() {
    let args = Args::parse();

    let mouse = mouse::Mouse::new();
    let config =
        mouse::MouseConfig::new(&mouse.device).expect("Failed to read mouse configuration");

    if args.dpi_values {
        let values = config.dpi_values
            .iter()
            .map(|dpi| dpi.value.to_string())
            .collect::<Vec<String>>()
            .join(",");

        println!("{}", values);
    }

    if args.current_dpi {
        println!(
            "{:?}",
            config.dpi_values[config.current_dpi_index as usize].value
        );
    }

    if args.battery_level {
        let battery = mouse.get_battery();
        println!("{:?}", battery.level);
    }

    if args.is_charging {
        let battery = mouse.get_battery();
        println!("{:?}", battery.charging);
    }
}
