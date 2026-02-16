use thiserror::Error;

#[derive(Error, Debug)]
pub enum AppError {
    // #[error("device not found")]
    // DeviceNotFound,
    #[error("hid error")]
    HidError(#[from] hidapi::HidError),
    // #[error("invalid value")]
    // InvalidValue,
    // #[error("crc problem")]
    // CrcProblem,
}
