use hidapi::HidDevice;

use crate::{
    hid::write_eeprom, models::{command::Command, error::AppError}
};

struct Filter {
    pub vendor_id: u16,
    pub product_id: u16,
}

const REPORT_ID: u8 = 0x08;

const FILTERS: [Filter; 2] = [
    Filter {
        vendor_id: 13652,
        product_id: 62967,
    },
    Filter {
        vendor_id: 13652,
        product_id: 62966,
    },
];

pub fn get_device() -> Result<Option<HidDevice>, AppError> {
    let api = hidapi::HidApi::new().unwrap();

    let devices = api.device_list().filter(|device| {
        FILTERS.iter().any(|filter| {
            device.vendor_id() == filter.vendor_id && device.product_id() == filter.product_id
        })
    });

    for device_info in devices {
        let device = device_info.open_device(&api)?;

        let mut buffer = [0x00; 512];
        let Ok(read_descriptor_count) = device.get_report_descriptor(&mut buffer) else {
            println!(
                "Failed to get report descriptor for device with VID: {:04x} and PID: {:04x}",
                device_info.vendor_id(),
                device_info.product_id()
            );
            continue;
        };

        let Ok(report_descriptor) =
            hidparser::parse_report_descriptor(&buffer[0..read_descriptor_count])
        else {
            println!(
                "Failed to parse report descriptor for device with VID: {:04x} and PID: {:04x}",
                device_info.vendor_id(),
                device_info.product_id()
            );
            continue;
        };

        if report_descriptor
            .output_reports
            .first()
            .and_then(|report| report.report_id)
            .is_some_and(|report_id| u32::from(report_id) == REPORT_ID as u32)
        {
            return Ok(Some(device));
        }
    }

    Ok(None)
}

pub fn read_device_buffer(device: &HidDevice) -> Result<Vec<u8>, AppError> {
    let mut full_buffer: [u8; 255] = [0x00; 255];
    let mut chunk_buffer: [u8; 17] = [0x00; 17];

    let mut addr = 0;

    while addr < 0x100 {
        write_eeprom(device, Command::ReadFlashData, addr, &[], 10)?;

        device.read_timeout(&mut chunk_buffer, 50)?;
        let buff_without_report_id = &chunk_buffer[1..];

        for i in 0..10 {
            let Some(source) = buff_without_report_id.get(5 + i) else {
                continue;
            };

            let Some(target) = full_buffer.get_mut(addr as usize + i) else {
                continue;
            };

            *target = *source;
        }

        addr += 10;
    }

    Ok(full_buffer.to_vec())
}
