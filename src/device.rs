use hidapi::HidDevice;

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

pub fn get_device() -> Option<HidDevice> {
    let api = hidapi::HidApi::new().unwrap();

    let devices = api.device_list().filter(|device| {
        FILTERS.iter().any(|filter| {
            device.vendor_id() == filter.vendor_id && device.product_id() == filter.product_id
        })
    });


    for device_info in devices {
        let Ok(device) = device_info.open_device(&api) else {
            println!("Failed to open device with VID: {:04x} and PID: {:04x}", device_info.vendor_id(), device_info.product_id());
            continue;
        };

        let mut buffer = [0x00; 512];
        let Ok(read_descriptor_count) = device.get_report_descriptor(&mut buffer) else {
            println!("Failed to get report descriptor for device with VID: {:04x} and PID: {:04x}", device_info.vendor_id(), device_info.product_id());
            continue;
        };

        let Ok(report_descriptor) =
            hidparser::parse_report_descriptor(&buffer[0..read_descriptor_count])
        else {
            println!("Failed to parse report descriptor for device with VID: {:04x} and PID: {:04x}", device_info.vendor_id(), device_info.product_id());
            continue;
        };

        if report_descriptor
            .output_reports
            .first()
            .and_then(|report| report.report_id)
            .is_some_and(|report_id| u32::from(report_id) == REPORT_ID as u32)
        {
            return Some(device);
        }
    }

    None
}
