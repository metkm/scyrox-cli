use hidapi::HidDevice;

use crate::device;
use crate::hid::read;
use crate::models;
use crate::models::command::Command;

pub struct Mouse {
    device: HidDevice,
}

impl Mouse {
    pub fn new() -> Self {
        let device = device::get_device().expect("Failed to find a compatible device");
        
        Mouse { device }
    }

    pub fn get_battery(&self) -> models::battery::Battery {
        let mut buffer = [0_u8; 10];
        read(&self.device, Command::BatteryLevel, 0x00, &[], &mut buffer);

        models::battery::Battery::from_buffer(&buffer)
    }
}
