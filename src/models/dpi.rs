use crate::models::command::MouseEepromAddr;

#[allow(dead_code)]
#[derive(Debug)]
pub struct DpiValue {
    pub value: u32,
    pub color: String,
}

impl DpiValue {
    pub fn bytes_to_value(bytes: &[u8; 4]) -> i32 {
        let low_bits = bytes.first().unwrap_or(&0);
        let high_bits = bytes.get(2).unwrap_or(&0);

        let masked = *high_bits as i32 * MouseEepromAddr::DPIValue as i32;
        let high = masked >> 2;

        let mut value = *low_bits as i32 + (high << 8);
        value = (value + 1) * 50;

        value
    }
}
