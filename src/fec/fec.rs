use libc::{c_uint, c_void};
use std::ptr;

use crate::enums::FecScheme;
use crate::liquid_dsp_sys as raw;

pub struct Fec {
    inner: raw::fec,
}

impl Fec {
    pub fn create(scheme: FecScheme) -> Self {
        let ptr: *mut c_void = ptr::null_mut();
        unsafe {
            Self {
                inner: raw::fec_create(u8::from(scheme) as c_uint, ptr),
            }
        }
    }

    /// return the encoded message length using a particular error-
    /// correction scheme (object-independent method)
    ///  scheme     :   forward error-correction scheme (FecScheme)
    ///  msg_len    :   raw uncoded message length
    pub fn get_enc_msg_length(scheme: FecScheme, msg_len: u32) -> u32 {
        unsafe { raw::fec_get_enc_msg_length(u8::from(scheme) as c_uint, msg_len as c_uint) as u32 }
    }

    /// get the theoretical rate of a particular forward error-
    /// correction scheme (object-independent method)
    pub fn get_rate(scheme: FecScheme) -> f32 {
        unsafe { raw::fec_get_rate(u8::from(scheme) as _) }
    }

    /// recreate a fec object
    ///  scheme :   new scheme (FecScheme)
    pub fn recreate(mut self, scheme: FecScheme) -> Self {
        let ptr: *mut c_void = ptr::null_mut();
        unsafe {
            self.inner = raw::fec_recreate(self.inner, u8::from(scheme) as c_uint, ptr);
        }
        self
    }

    /// print channel object
    pub fn print(&self) {
        unsafe {
            raw::fec_print(self.inner);
        }
    }

    /// encode a block of data using a fec scheme
    ///  raw        :   decoded message
    ///  encoded    :   encoded message
    pub fn encode(&self, raw: &[u8], encoded: &mut [u8]) {
        unsafe {
            raw::fec_encode(
                self.inner,
                raw.len() as c_uint,
                raw.as_ptr() as *mut _,
                encoded.as_mut_ptr() as _,
            );
        }
    }

    /// decode a block of data using a fec scheme
    ///  encoded    :   encoded message
    ///  raw        :   decoded message
    pub fn decode(&self, encoded: &[u8], raw: &mut [u8]) {
        unsafe {
            raw::fec_decode(
                self.inner,
                raw.len() as c_uint,
                encoded.as_ptr() as _,
                raw.as_mut_ptr() as _,
            );
        }
    }

    pub fn decode_soft(&self, encoded: &[u8], raw: &mut [u8]) {
        unsafe {
            raw::fec_decode_soft(
                self.inner,
                raw.len() as c_uint,
                encoded.as_ptr() as _,
                raw.as_mut_ptr() as _,
            );
        }
    }
}

impl Drop for Fec {
    fn drop(&mut self) {
        unsafe {
            raw::fec_destroy(self.inner);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Fec;
    use crate::enums::FecScheme;

    #[test]
    fn test_encode_decode() {
        let len = 4;
        let raw: &mut [u8] = &mut [0x67, 0xC6, 0x69, 0x73];
        let mut decoded_data = vec![0u8; len as usize];
        let expected_result: &[u8] = &[0xCC, 0x3D, 0xE6, 0x6C, 0xC6, 0x47, 0xC3];

        let enc_len = Fec::get_enc_msg_length(FecScheme::HAMMING74, 4);

        let mut encoded_data = vec![0u8; enc_len as usize];
        let fec = Fec::create(FecScheme::HAMMING74);

        fec.encode(&raw, &mut encoded_data);

        assert_eq!(expected_result, encoded_data.as_slice());

        fec.decode(&encoded_data, &mut decoded_data);

        assert_eq!(raw, decoded_data.as_slice());

        let bad_encoded_data: &[u8] = &[0xC8, 0x3D, 0xE6, 0x6C, 0xC6, 0x47, 0xC3];
        fec.decode(bad_encoded_data, &mut decoded_data);

        assert_eq!(raw, decoded_data.as_slice());
    }
}
