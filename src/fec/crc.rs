use std::ffi::{CString, NulError};

use crate::enums::CrcScheme;
use crate::errors::LiquidError;

use crate::liquid_dsp_sys as raw;

impl CrcScheme {
    /// Print compact list of existing and available crc schemes
    pub fn print_crc_schemes() {
        unsafe {
            raw::liquid_print_crc_schemes();
        }
    }

    pub fn getopt_str2crc(s: &str) -> Result<Self, NulError> {
        unsafe {
            CString::new(s)
                .and_then(|c| Ok(CrcScheme::from(raw::liquid_getopt_str2crc(c.as_ptr()) as u8)))
        }
    }

    /// get length of CRC (bytes)
    pub fn get_length(&self) -> usize {
        unsafe { raw::crc_get_length(u8::from(self.clone()) as _) as usize }
    }

    /// generates error-detection key
    ///  msg        :   input data message,
    pub fn generate_key<T: AsRef<[u8]>>(&self, msg: T) -> Result<usize, LiquidError> {
        match self {
            Self::CRC_UNKNOWN => return Err(LiquidError::InvalidCrcScheme),
            _ => {
                let i = unsafe {
                    raw::crc_generate_key(
                        u8::from(self.clone()) as _,
                        msg.as_ref().as_ptr() as _,
                        msg.as_ref().len() as _,
                    ) as usize
                };
                Ok(i)
            }
        }
    }

    // TODO: check if there are lenght constrains
    /// generate error-detection key and append to end of message
    ///  msg        :   input data message
    pub fn append_key<T: AsMut<[u8]>>(&self, mut msg: T) {
        unsafe {
            raw::crc_append_key(
                u8::from(self.clone()) as _,
                msg.as_mut().as_ptr() as _,
                msg.as_mut().len() as _,
            );
        }
    }

    /// validate message using error-detection key
    ///  msg        :   input data message
    ///  key        :   error-detection key
    pub fn crc_validate_message<T: AsRef<[u8]>>(
        &self,
        msg: T,
        key: usize,
    ) -> Result<bool, LiquidError> {
        match self {
            Self::CRC_UNKNOWN => return Err(LiquidError::InvalidCrcScheme),
            _ => unsafe {
                Ok(raw::crc_validate_message(
                    u8::from(self.clone()) as _,
                    msg.as_ref().as_ptr() as _,
                    msg.as_ref().len() as _,
                    key as _,
                ) == 1)
            },
        }
    }

    /// check message with key appended to end of array
    ///  msg        :   input data message, [size: _n+p x 1], input data message size (excluding key at end)
    pub fn check_key<T: AsRef<[u8]>>(&self, msg: T) -> Result<bool, LiquidError> {
        match self {
            Self::CRC_UNKNOWN => return Err(LiquidError::InvalidCrcScheme),
            _ => unsafe {
                Ok(raw::crc_check_key(
                    u8::from(self.clone()) as _,
                    msg.as_ref().as_ptr() as _,
                    msg.as_ref().len() as _,
                ) == 1)
            },
        }
    }

    /// get size of key (bytes)
    pub fn sizeof_key(scheme: CrcScheme) -> Result<usize, LiquidError> {
        unsafe {
            match scheme {
                Self::CRC_UNKNOWN => return Err(LiquidError::InvalidCrcScheme),
                _ => Ok(raw::crc_sizeof_key(u8::from(scheme) as _) as usize),
            }
        }
    }
}
