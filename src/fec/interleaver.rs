use libc::{c_uint};

use crate::liquid_dsp_sys as raw;

pub struct Interleaver {
    inner: raw::interleaver,
}

impl Interleaver {

    pub fn create(n: u32) -> Self {
        unsafe {
            Self {
                inner: raw::interleaver_create(n as c_uint),
            }
        }
    }
    
    pub fn print(&self) {
        unsafe {
            raw::interleaver_print(self.inner);
        }
    }
    
    pub fn set_depth(&mut self, depth: u32) {
        unsafe {
            raw::interleaver_set_depth(self.inner, depth as c_uint);
        }
    }
    pub fn encode(&self, raw: &[u8], encoded: &mut[u8]) {
        assert!(raw.len() == encoded.len(), "buffers must have the same len");
        unsafe {
            raw::interleaver_encode(self.inner, raw.as_ptr() as _, encoded.as_mut_ptr() as _);
        }
    }
    
    pub fn encode_soft(&self, raw: &[u8], encoded: &mut[u8]) {
        assert!(raw.len() == encoded.len(), "buffers must have the same len");
        unsafe {
            raw::interleaver_encode_soft(self.inner, raw.as_ptr() as _, encoded.as_mut_ptr() as _);
        }
    }

    pub fn decode(&self, encoded: &[u8], raw: &mut [u8]) {
        assert!(raw.len() == encoded.len(), "buffers must have the same len");
        unsafe {
            raw::interleaver_decode(self.inner, encoded.as_ptr() as _, raw.as_mut_ptr() as _);
        }
    }
    
    pub fn decode_soft(&self, encoded: &[u8], raw: &mut [u8]) {
        assert!(raw.len() == encoded.len(), "buffers must have the same len");
        unsafe {
            raw::interleaver_decode_soft(self.inner, encoded.as_ptr() as _, raw.as_mut_ptr() as _);
        }
    }
}


impl Drop for Interleaver {
    fn drop(&mut self) {
        unsafe {
            raw::interleaver_destroy(self.inner);
        }
    }
}