use hidapi::HidDevice;

use crate::REPORT_ID;
use crate::models::command::Command;

pub fn get_usb_crc(buffer: &[u8]) -> u8 {
    let mut crc: i32 = buffer[0..buffer.len() - 1]
        .iter()
        .fold(0, |acc, e| acc + *e as i32);

    crc &= 0xFF;
    crc = 0x55 - crc;

    crc as u8
}

pub fn write_eeprom(
    device: &HidDevice,
    command: Command,
    address: u16,
    value: &[u8],
    length: u8,
) -> Result<usize, Box<dyn std::error::Error>> {
    let address_bytes = address.to_be_bytes();

    let mut buffer: [u8; 17] = [
        0x08,
        command.into(),
        0x00,
        *address_bytes.first().unwrap_or(&0),
        *address_bytes.get(1).unwrap_or(&0),
        length,
        0x00,
        0x00,
        0x00,
        0x00,
        0x00,
        0x00,
        0x00,
        0x00,
        0x00,
        0x00,
        0xEF,
    ];

    for (i, val) in value.iter().enumerate() {
        let Some(buff_val) = buffer.get_mut(i + 6) else {
            continue;
        };

        *buff_val = *val;
    }

    // buffer starts from 1 because the 0 is the reportId
    let crc = get_usb_crc(&buffer[1..]).saturating_sub(REPORT_ID);

    if let Some(val) = buffer.get_mut(16) {
        *val = crc;
    };

    let written_count = device.write(&buffer)?;

    Ok(written_count)
}

pub fn read(
    device: &HidDevice,
    command: Command,
    addr: u16,
    value: &[u8],
    buffer: &mut [u8],
) -> usize {
    #[allow(unused_must_use)]
    write_eeprom(&device, command, addr, value, value.len() as u8);
    let read_count = device.read_timeout(buffer, 50);

    read_count.unwrap_or(0)
}
