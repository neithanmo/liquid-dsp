use libc::c_uint;
use std::ptr;

use num::complex::Complex32;

use crate::liquid_dsp_sys as raw;

use crate::utils::{LiquidFloatComplex, ToCPointer, ToCPointerMut, ToCValue};

pub struct ChannelCccf {
    inner: raw::channel_cccf,
}

impl ChannelCccf {
    /// create structured channel object with default parameters
    pub fn create() -> Self {
        unsafe {
            Self {
                inner: raw::channel_cccf_create(),
            }
        }
    }

    /// print channel object
    pub fn print(&self) {
        unsafe {
            raw::channel_cccf_print(self.inner);
        }
    }

    /// apply additive white Gausss noise impairment
    ///  N0dB : noise floor power spectral density
    ///  SNRdB: signal-to-noise ratio [dB]
    pub fn add_awgn(&mut self, N0dB: f32, SNRdB: f32) {
        unsafe {
            raw::channel_cccf_add_awgn(self.inner, N0dB, SNRdB);
        }
    }

    /// apply carrier offset impairment
    ///  frequency  : carrier frequency offse [radians/sample]
    ///  phase      : carrier phase offset    [radians]
    pub fn add_carrier_offset(&mut self, frequency: f32, phase: f32) {
        unsafe {
            raw::channel_cccf_add_carrier_offset(self.inner, frequency, phase);
        }
    }

    /// apply multi-path channel impairment
    ///  h : channel coefficients
    pub fn add_multipath(&mut self, h: &[Complex32]) {
        assert!(
            h.len() > 0 && h.len() <= 1000,
            "The number of coeficients must be > 0 and <= 1000"
        );
        unsafe {
            raw::channel_cccf_add_multipath(self.inner, h.to_ptr() as *mut _, h.len() as c_uint);
        }
    }

    /// apply multi-path channel impairment
    ///  len : number of auto-generated ramdom coeficients
    pub fn add_multipath_random(&mut self, len: u32) {
        assert!(
            len > 0 && len <= 1000,
            "The number of coeficients must be > 0 and <= 1000"
        );
        let ptr: *mut LiquidFloatComplex = ptr::null_mut();
        unsafe {
            raw::channel_cccf_add_multipath(self.inner, ptr, len as c_uint);
        }
    }

    /// apply slowly-varying shadowing impairment
    ///  sigma      : std. deviation for log-normal shadowing
    ///  fd         : Doppler frequency, _fd in (0,0.5)
    pub fn add_shadowing(&mut self, sigma: f32, fd: f32) {
        assert!(
            sigma <= 0f32,
            "standard deviation less than or equal to zero"
        );

        assert!(
            fd <= 0f32 || fd >= 0.5,
            " Doppler frequency must be in (0,0.5)"
        );
        unsafe {
            raw::channel_cccf_add_shadowing(self.inner, sigma, fd);
        }
    }

    /// apply channel impairments on single input sample
    pub fn execute(&self, sample: Complex32) -> Complex32 {
        let mut out = Complex32::default();
        unsafe {
            raw::channel_cccf_execute(self.inner, sample.to_c_value(), out.to_ptr_mut());
        }
        out
    }

    pub fn execute_block(&self, input: &[Complex32], output: &mut [Complex32]) {
        assert!(
            input.len() == output.len(),
            "buffers must have the same lenght"
        );
        unsafe {
            raw::channel_cccf_execute_block(
                self.inner,
                input.to_ptr() as *mut _,
                input.len() as c_uint,
                output.to_ptr_mut(),
            );
        }
    }
}

impl Drop for ChannelCccf {
    fn drop(&mut self) {
        unsafe {
            raw::channel_cccf_destroy(self.inner);
        }
    }
}
