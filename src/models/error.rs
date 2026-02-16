use thiserror::Error;

#[derive(Error, Debug)]
pub enum AppError {
    // #[error("device not found")]
    // DeviceNotFound,
    #[error(transparent)]
    HidError(#[from] hidapi::HidError),
    // #[error("invalid value")]
    // InvalidValue,
    // #[error("crc problem")]
    // CrcProblem,
}
