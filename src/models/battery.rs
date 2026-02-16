const VOLTAGES: [i16; 21] = [
    3050, 3420, 3480, 3540, 3600, 3660, 3720, 3760, 3800, 3840, 3880, 3920, 3940, 3960, 3980, 4000,
    4020, 4040, 4060, 4080, 4110,
];

pub fn voltage_to_level(voltage: i16) -> u8 {
    if let Some(last) = VOLTAGES.last()
        && voltage > *last
    {
        return 100;
    };

    let Some(voltage_index) = VOLTAGES.iter().position(|rvolt| *rvolt > voltage) else {
        return 0;
    };

    if voltage_index == 0 {
        return 0;
    }

    let Some(rounded_voltage) = VOLTAGES.get(voltage_index) else {
        return 0;
    };

    let Some(rounded_voltage_down) = VOLTAGES.get(voltage_index.saturating_sub(1)) else {
        return 0;
    };

    let interval = (rounded_voltage - rounded_voltage_down) / 5;
    let level = (voltage - *rounded_voltage_down) / interval + (voltage_index as i16 - 1) * 5;

    level.try_into().unwrap_or(0)
}


#[derive(Debug)]
pub struct Battery {
    pub charging: bool,
    pub level: u8,
}

impl Battery {
    pub fn from_buffer(buffer: &[u8]) -> Battery {
        let voltage =
            i16::from_be_bytes([*buffer.get(8).unwrap_or(&0), *buffer.get(9).unwrap_or(&0)]);
        let level = voltage_to_level(voltage);

        Battery {
            charging: buffer.get(7).unwrap_or(&0) == &1,
            level,
        }
    }
}
