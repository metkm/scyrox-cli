use hidapi::HidDevice;

use crate::device::{self, read_device_buffer};
use crate::hid::read;
use crate::models;
use crate::models::command::{Command, MouseEepromAddr};
use crate::models::dpi::DpiValue;
use crate::models::error::AppError;
use crate::utils::buffer_to_hex;

pub struct Mouse {
    pub device: HidDevice,
}

#[derive(Debug)]
pub struct MouseConfig {
    pub dpi_values: Vec<models::dpi::DpiValue>,
    pub current_dpi_index: u8,
}

impl MouseConfig {
    pub fn new(device: &HidDevice) -> Result<Self, AppError> {
        let buffer = read_device_buffer(device)?;

        Ok(Self {
            dpi_values: {
                let mut values: Vec<DpiValue> = Vec::with_capacity(8);

                for i in 0..8 {
                    let dpi_base_addr = i * 4 + MouseEepromAddr::DPIValue as usize;

                    let bytes: &[u8; 4] =
                        &buffer[dpi_base_addr..dpi_base_addr + 4].try_into().unwrap();

                    let color_base_add = dpi_base_addr + MouseEepromAddr::DPIColor as usize;
                    let color_bytes: &[u8; 3] = &buffer[color_base_add..(color_base_add + 3)]
                        .try_into()
                        .unwrap();

                    values.push(DpiValue {
                        value: DpiValue::bytes_to_value(bytes) as u32,
                        color: buffer_to_hex(color_bytes),
                    });
                }

                values
            },
            current_dpi_index: *buffer
                .get(MouseEepromAddr::CurrentDPI as usize)
                .unwrap_or(&0),
        })
    }
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
