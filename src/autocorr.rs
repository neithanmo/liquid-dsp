use std::fmt;

use crate::liquid_dsp_sys as raw;
use libc::{c_uint};

use num::complex::{Complex32};
use std::mem::transmute;

pub(crate) trait ToRaw<T> {
    unsafe fn to_raw(&mut self) -> *mut T;
}

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
            let mut out = Complex32::default();
            let ptr = &mut out as *mut Complex32;
            // this is safe because Complex<T> reproduce c
            raw::autocorr_cccf_execute(self.inner, transmute::<*mut Complex32, *mut LiquidFloatComplex>(ptr));
            *ptr
        }
    }

    pub fn execute_block(&self, input: &[Complex32], output: &mut [Complex32]) {
        assert!(input.len() == output.len(), "Input and output buffers with different length");
        input.iter().zip(output.iter_mut()).for_each(|(isample, osample)| {
            self.push(*isample);
            unsafe {
                raw::autocorr_cccf_execute(self.inner, transmute::<*mut Complex32, *mut LiquidFloatComplex>(osample as *mut Complex32));
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

#[cfg(test)]
mod tests {
    use num::complex::{Complex32};
    use num::Zero;
    use super::{AutoCorrCccf, AutoCorrRrrf};

    #[test]
    fn test_autocorr_cccf() {
        let auto_cccf = AutoCorrCccf::create(16, 8);
        assert_eq!(&format!("{:?}", auto_cccf),"autocorr [16 window, 8 delay]");
    }

    #[test]
    fn test_autocorr_cccf_execute_block() {
        let mut input = Vec::with_capacity(4);
        let mut output = vec![Complex32::zero(); 4];
        for i in 0..4 {
            input.push(Complex32::new(0.0 + i as f32, 4.5 - i as f32 * (-1.0)));
        }
        let auto_cccf = AutoCorrCccf::create(4, 0);
        auto_cccf.execute_block(&input, &mut output);
        let solution = [Complex32::new(20.25, 0.0), Complex32::new(51.50, 0.0), Complex32::new(97.75, 0.0), Complex32::new(163.0, 0.0)];
        assert_eq!(&output, &solution);
    }

    #[test]
    fn test_autocorr_rrrf() {
        let auto_rrrf = AutoCorrRrrf::create(16, 8);
        assert_eq!(&format!("{:?}", auto_rrrf),"autocorr [16 window, 8 delay]");
    }

    #[test]
    fn test_autocorr_rrrf_execute_block() {
        let mut input = Vec::with_capacity(4);
        let mut output = vec![0.0f32; 4];
        let auto_rrrf = AutoCorrRrrf::create(4, 0);
        for i in 0..4 {
            input.push(4.5 - i as f32 * (-1.0));
        }
        auto_rrrf.execute_block(&input, &mut output);
        let solution = [20.25, 50.5, 92.75, 149.0];

        assert_eq!(&output, &solution);
    }
}

