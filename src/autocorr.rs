use std::fmt;

use crate::liquid_dsp_sys as raw;
use libc::{c_char, c_uint, c_uchar, c_void, memcpy};

use num::complex::{Complex32};

pub(crate) type LiquidFloatComplex = raw::liquid_float_complex;

#[derive(Debug, Clone)]
pub(crate) struct LiquidComplex(LiquidFloatComplex);

impl From<Complex32> for LiquidComplex {
    fn from(value: Complex32) -> Self {
        Self(LiquidFloatComplex {
            re: value.re,
            im: value.im,
        })
    }
}

impl From<LiquidComplex> for Complex32 {
    fn from(value: LiquidComplex) -> Self {
        Self {
            re: value.0.re,
            im: value.0.im,
        }
    }
}

pub struct AutoCorrRrrf {
    inner: raw::autocorr_rrrf,
    window: u32,
    delay: u32,
}

pub struct AutoCorrCccf {
    inner: raw::autocorr_cccf,
    window: u32,
    delay: u32,
}

impl AutoCorrCccf {
    ///  creates and returns an autocorr object with a *window* size of N samples and a *delay* of d samples.
    pub fn create(N: u32, d: u32) -> Self {
        let inner = unsafe {
            raw::autocorr_cccf_create(N as c_uint, d as c_uint)
        };
        Self {
            inner,
            window: N,
            delay: d,
        }
    }

    pub fn reset(&mut self) {
        unsafe {
            raw::autocorr_cccf_reset(self.inner);
        }
    }

    pub fn push(&self, sample: Complex32) {
        unsafe {
            let complex = LiquidComplex::from(sample);
            raw::autocorr_cccf_push(self.inner, complex.0);
        }
    }

    pub fn execute(&self) -> Complex32 {
        unsafe {
            let out = &mut LiquidFloatComplex::default() as *mut _;
            raw::autocorr_cccf_execute(self.inner, out);
            Complex32::from(LiquidComplex(*out))
        }
    }

    pub fn execute_block(&self, input: &[Complex32], output: &mut [Complex32]) {
        assert!(input.len() == output.len(), "Input and output buffers with different length");
        input.iter().zip(output.iter_mut()).for_each(|(isample, osample)| {
            let out = &mut LiquidFloatComplex::default() as *mut _;
            self.push(*isample);
            unsafe {
                raw::autocorr_cccf_execute(self.inner, out);
                *osample = Complex32::from(LiquidComplex(*out));
            }    
        });
    }

    pub fn get_energy(&self) -> f32 {
        unsafe {
            raw::autocorr_cccf_get_energy(self.inner) as f32
        }
    }
}

impl fmt::Debug for AutoCorrCccf {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "autocorr [{} window, {} delay]", self.window, self.delay)
    }
}

impl Drop for AutoCorrCccf {
    fn drop(&mut self) {
        unsafe {
            raw::autocorr_cccf_destroy(self.inner);
        }
    }
}

impl AutoCorrRrrf {
    ///  creates and returns an autocorr object with a *window* size of N samples and a *delay* of d samples.
    pub fn create(N: u32, d: u32) -> Self {
        let inner = unsafe {
            raw::autocorr_rrrf_create(N as c_uint, d as c_uint)
        };
        Self {
            inner,
            window: N,
            delay: d,
        }
    }

    pub fn reset(&mut self) {
        unsafe {
            raw::autocorr_rrrf_reset(self.inner);
        }
    }

    pub fn push(&self, sample: f32) {
        unsafe {
            raw::autocorr_rrrf_push(self.inner, sample);
        }
    }

    pub fn execute(&self) -> f32 {
        unsafe {
            let out = &mut (0.0 as f32) as *mut f32;
            raw::autocorr_rrrf_execute(self.inner, out);
            *out
        }
    }

    pub fn execute_block(&self, input: &[f32], output: &mut [f32]) {
        assert!(input.len() == output.len(), "Input and output buffers with different length");
        input.iter().zip(output.iter_mut()).for_each(|(isample, osample)| {
            let out = &mut (0.0 as f32) as *mut f32;
            self.push(*isample);
            unsafe {
                raw::autocorr_rrrf_execute(self.inner, osample as *mut _);
            }    
        });
    }

    pub fn get_energy(&self) -> f32 {
        unsafe {
            raw::autocorr_rrrf_get_energy(self.inner) as f32
        }
    }
}

impl fmt::Debug for AutoCorrRrrf {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "autocorr [{} window, {} delay]", self.window, self.delay)
    }
}

impl Drop for AutoCorrRrrf {
    fn drop(&mut self) {
        unsafe {
            raw::autocorr_rrrf_destroy(self.inner);
        }
    }
}

