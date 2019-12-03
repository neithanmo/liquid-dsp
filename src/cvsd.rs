use libc::{c_uint};
use std::fmt;

use crate::liquid_dsp_sys as raw;

pub struct Cvsd {
    inner: raw::cvsd,
    num_bits: u32,
    zeta: f32,
    alpha: f32

}

impl Cvsd {
    pub fn create(num_bits: u32, zeta: f32, alpha: f32) -> Self {
        unsafe {
            Self {
                inner: raw::cvsd_create(num_bits as c_uint, zeta, alpha),
                num_bits,
                zeta,
                alpha,
            }
        }
    }

    pub fn encode(&self, audio_sample: f32) -> u8 {
        unsafe {
            raw::cvsd_encode(self.inner, audio_sample) as u8
        }
    }

    pub fn decode(&self, bit: u8) -> f32 {
        unsafe {
            raw::cvsd_decode(self.inner, bit)
        }
    }

    /// encode 8 audio samples
    pub fn encode8(&self, audio: &[f32]) -> u8 {
        assert!(
            audio.len() == 8usize,
            "Input must contain 8 audio samples"
        );
        unsafe {
            let data = &mut 0u8 as *mut _;
            raw::cvsd_encode8(self.inner, audio.as_ptr() as *mut _, data);
            *data
        }
    }

    /// decode u8 data into 8 audio samples
    pub fn cvsd_decode8(&self, data: u8, audio: &mut[f32]) {
        assert!(
            audio.len() == 8usize,
            "Audio must have capacity for  8 audio samples"
        ); 
        unsafe {
            raw::cvsd_decode8(self.inner, data, audio.as_mut_ptr());
        }
    }
}

impl fmt::Debug for Cvsd {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "cvsd codec: num_bits: {}, zeta: {}, alpha: {}", self.num_bits, self.zeta, self.alpha)
    }
}

impl Drop for Cvsd {
    fn drop(&mut self) {
        unsafe {
            raw::cvsd_destroy(self.inner);
        }
    }
}