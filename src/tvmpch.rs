use libc::c_uint;
use num::complex::Complex32;

use crate::liquid_dsp_sys as raw;

use crate::utils::{ToCPointer, ToCPointerMut, ToCValue};


pub struct TvmpchCccf {
    inner: raw::tvmpch_cccf,
}

impl TvmpchCccf {

    /// create time-varying multi-path channel emulator object
    ///  n      :   number of coefficients
    ///  std    :   standard deviation 
    ///  tau    :   coherence time
    pub fn create(n: u32, std: f32, tau: f32) -> Self {
        assert!(n > 0, "filter length must be greater than one");
        assert!(std > 0f32, "standard deviation must be positive");
        assert!(tau > 0f32 && tau < 1f32, "coherence time must be in [0,1]");
        unsafe {
            Self {
                inner: raw::tvmpch_cccf_create(n, std, tau),
            }
        }
    }

    pub fn reset(&mut self) {
        unsafe {
            raw::tvmpch_cccf_reset(self.inner);
        }
    }

    /// print channel object
    pub fn print(&self) {
        unsafe {
            raw::tvmpch_cccf_print(self.inner);
        }
    }

    /// push sample into filter object's internal buffer
    ///  sample      :   input sample
    pub fn push(&mut self, sample: Complex32) {
        unsafe {
            raw::tvmpch_cccf_push(self.inner, sample.to_c_value())
        }
    }

    /// Returns a compute output sample 
    /// 
    /// (dot product between internal
    /// filter coefficients and internal buffer)
    pub fn execute(&self) -> Complex32 { 
        let mut s = Complex32::default();
        unsafe {
            raw::tvmpch_cccf_execute(self.inner, s.to_ptr_mut());
            // TODO: should this function return an option???
            s
        }
    }

    /// execute the filter on a block of input samples; the
    /// input and output buffers may be the same
    ///  samples     : input array [size: _n x 1]
    ///  output      : output array [size: _n x 1]
    pub fn execute_block(&self, samples:&[Complex32], output: &mut[Complex32]) {
        assert!(
            samples.len() == output.len(),
            "Input and output buffers with different length"
        );
        unsafe {
            raw::tvmpch_cccf_execute_block(self.inner, samples.to_ptr() as *mut _, samples.len() as c_uint, 
            output.to_ptr_mut());
        }
    }
}

impl Drop for TvmpchCccf {
    fn drop(&mut self) {
        unsafe {
            raw::tvmpch_cccf_destroy(self.inner);
        }
    }
}