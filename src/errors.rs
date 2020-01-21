use std::error;
use std::fmt;

pub enum LiquidError {
    /*     FftSize,
    WindowSize, // when window size is higher than FftSize
    NullWindowSize, // case window size is == 0
    NonPositiveValue, // case when a value is negative */
    EmptyBuffer,
    InvalidLength { description: String },
    InvalidCrcScheme,
    InvalidFecScheme,
    InvalidValue(String), // when a value does not fullfill certain restrictions
    Unknown,
}

impl LiquidError {
    pub(crate) fn as_str(&self) -> &str {
        match self {
            Self::InvalidFecScheme => "cannot validate with FecScheme of type UNKNOWN",
            Self::EmptyBuffer => "Buffer is already empty",
            Self::InvalidLength { ref description } => description,
            Self::InvalidCrcScheme => "cannot validate with CRC type UNKNOWN",
            Self::InvalidValue(ref detail) => detail,
            Self::Unknown => "liquid unknown error",
        }
    }
}

impl fmt::Display for LiquidError {
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(fmt, "{}", self.as_str())
    }
}

impl fmt::Debug for LiquidError {
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            fmt,
            "{} in file {} line {}",
            self.as_str(),
            file!(),
            line!()
        )
    }
}

impl error::Error for LiquidError {
    fn description(&self) -> &str {
        self.as_str()
    }
}
