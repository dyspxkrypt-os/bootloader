use r_efi::base::Status;
use std::error::Error;
use std::fmt::{Display, Formatter, Result};

#[derive(Debug)]
pub struct UefiStopError(pub(crate) Status);

impl Display for UefiStopError {
    fn fmt(&self, f: &mut Formatter) -> Result {
        f.write_str("UEFI error: ")?;

        match self.0 {
            Status::DEVICE_ERROR => write!(f, "DEVICE_ERROR"),
            Status::UNSUPPORTED => write!(f, "UNSUPPORTED"),
            _ => write!(f, "unknown error"),
        }?;

        writeln!(f, " ({:8x})", self.0.as_usize())
    }
}

impl Error for UefiStopError {}
