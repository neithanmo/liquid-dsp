use std::error;
use std::fmt;

pub struct LiquidError {
    pub(crate) err: ErrorKind,
}

pub enum ErrorKind {
    /*     FftSize,
    WindowSize, // when window size is higher than FftSize
    NullWindowSize, // case window size is == 0
    NonPositiveValue, // case when a value is negative */
    EmptyBuffer,
    InvalidCrcScheme,
    InvalidFecScheme,
    InvalidValue(String), // when a value does not fullfill certain restrictions
    Unknown,
}

impl ErrorKind {
    pub(crate) fn as_str(&self) -> &str {
        match self {
            Self::InvalidFecScheme => "cannot validate with FecScheme of type UNKNOWN",
            Self::EmptyBuffer => "Buffer is already empty",
            Self::InvalidCrcScheme => "cannot validate with CRC type UNKNOWN",
            Self::InvalidValue(ref detail) => detail,
            Self::Unknown => "liquid unknown error",
        }
    }
}

impl fmt::Display for ErrorKind {
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(fmt, "{}", self.as_str())
    }
}

impl fmt::Debug for ErrorKind {
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

impl fmt::Display for LiquidError {
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.err.fmt(fmt)
    }
}

impl fmt::Debug for LiquidError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.err.fmt(f)
    }
}

impl From<ErrorKind> for LiquidError {
    fn from(kind: ErrorKind) -> Self {
        Self { err: kind }
    }
}

impl error::Error for LiquidError {
    fn description(&self) -> &str {
        self.err.as_str()
    }
}
