use std::{fmt, io};

#[derive(Debug)]
pub enum DeviceParseError {
    IoError (io::Error),
    FormatError,
}

impl fmt::Display for DeviceParseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            DeviceParseError::FormatError =>
                write!(f, "The output format is not supported yet"),
            DeviceParseError::IoError(inner) =>
                write!(f, "{}", inner),
        }
    }
}

impl From<io::Error> for DeviceParseError {
    fn from(err: io::Error) -> DeviceParseError {
        DeviceParseError::IoError(err)
    }
}