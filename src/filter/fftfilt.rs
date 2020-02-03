//! *fftfilt* : finite impulse response (FIR) filter using fast Fourier
//!           transforms (FFTs)
use num::complex::Complex32;

use crate::liquid_dsp_sys as raw;
use crate::utils::{ToCPointer, ToCPointerMut, ToCValue};

use crate::errors::LiquidError;
use crate::LiquidResult;

pub struct FftFiltRrrf {
    inner: raw::fftfilt_rrrf,
}

pub struct FftFiltCrcf {
    inner: raw::fftfilt_crcf,
}

pub struct FftFiltCccf {
    inner: raw::fftfilt_cccf,
}

macro_rules! fftfilt_impl {
    ($obj:ty, ($print:expr,
        $reset:expr,
        $len:expr,
        $destroy:expr)) => {
        impl $obj {
            pub fn print(&self) {
                unsafe {
                    $print(self.inner);
                }
            }

            pub fn reset(&mut self) {
                unsafe {
                    $reset(self.inner);
                }
            }

            pub fn len(&self) -> usize {
                unsafe { $len(self.inner) as usize }
            }
        }

        impl Drop for $obj {
            fn drop(&mut self) {
                unsafe {
                    $destroy(self.inner);
                }
            }
        }
    };
}

impl FftFiltRrrf {
    /// create FFT-based FIR filter using external coefficients
    ///  h      : filter coefficients [size: _h_len x 1]
    ///  n      : block size = nfft/2, at least _h_len-1
    pub fn create(h: &[f32], n: usize) -> LiquidResult<Self> {
        if h.is_empty() {
            return Err(LiquidError::InvalidLength {
                description: "filter length must be greater than zero".to_owned(),
            });
        } else if n < h.len() - 1 {
            return Err(LiquidError::InvalidValue(
                "block length must be greater than h.len()-1".to_owned(),
            ));
        }

        Ok(Self {
            inner: unsafe { raw::fftfilt_rrrf_create(h.as_ptr() as _, h.len() as _, n as _) },
        })
    }

    /// set output scaling for filter
    pub fn set_scale(&mut self, scale: f32) {
        unsafe {
            raw::fftfilt_rrrf_set_scale(self.inner, scale);
        }
    }

    /// get output scaling for filter
    pub fn get_scale(&self) -> f32 {
        unsafe {
            let mut scale = 0f32;
            raw::fftfilt_rrrf_get_scale(self.inner, scale.to_ptr_mut());
            scale
        }
    }

    /// execute the filter on internal buffer and coefficients
    ///  x      : pointer to input data array  [size: _n x 1]
    ///  y      : pointer to output data array [size: _n x 1]
    pub fn execute(&self, x: &[f32], y: &mut [f32]) {
        assert!(x.len() == y.len(), "x and y must be the same length");
        unsafe {
            raw::fftfilt_rrrf_execute(self.inner, x.as_ptr() as _, y.as_mut_ptr());
        }
    }
}

impl FftFiltCrcf {
    /// create FFT-based FIR filter using external coefficients
    ///  h      : filter coefficients [size: _h_len x 1]
    ///  n      : block size = nfft/2, at least _h_len-1
    pub fn create(h: &[f32], n: usize) -> LiquidResult<Self> {
        if h.is_empty() {
            return Err(LiquidError::InvalidLength {
                description: "filter length must be greater than zero".to_owned(),
            });
        } else if n < h.len() - 1 {
            return Err(LiquidError::InvalidValue(
                "block length must be greater than h.len()-1".to_owned(),
            ));
        }

        Ok(Self {
            inner: unsafe { raw::fftfilt_crcf_create(h.as_ptr() as _, h.len() as _, n as _) },
        })
    }

    /// set output scaling for filter
    pub fn set_scale(&mut self, scale: f32) {
        unsafe {
            raw::fftfilt_crcf_set_scale(self.inner, scale);
        }
    }

    /// get output scaling for filter
    pub fn get_scale(&self) -> f32 {
        unsafe {
            let mut scale = 0f32;
            raw::fftfilt_crcf_get_scale(self.inner, scale.to_ptr_mut());
            scale
        }
    }

    /// execute the filter on internal buffer and coefficients
    ///  x      : pointer to input data array  [size: _n x 1]
    ///  y      : pointer to output data array [size: _n x 1]
    pub fn execute(&self, x: &[Complex32], y: &mut [Complex32]) {
        assert!(x.len() == y.len(), "x and y must be the same length");
        unsafe {
            raw::fftfilt_crcf_execute(self.inner, x.to_ptr() as _, y.to_ptr_mut());
        }
    }
}

impl FftFiltCccf {
    /// create FFT-based FIR filter using external coefficients
    ///  h      : filter coefficients [size: _h_len x 1]
    ///  n      : block size = nfft/2, at least _h_len-1
    pub fn create(h: &[Complex32], n: usize) -> LiquidResult<Self> {
        if h.is_empty() {
            return Err(LiquidError::InvalidLength {
                description: "filter length must be greater than zero".to_owned(),
            });
        } else if n < h.len() - 1 {
            return Err(LiquidError::InvalidValue(
                "block length must be greater than h.len()-1".to_owned(),
            ));
        }

        Ok(Self {
            inner: unsafe { raw::fftfilt_cccf_create(h.to_ptr() as _, h.len() as _, n as _) },
        })
    }

    /// set output scaling for filter
    pub fn set_scale(&mut self, scale: Complex32) {
        unsafe {
            raw::fftfilt_cccf_set_scale(self.inner, scale.to_c_value());
        }
    }

    /// get output scaling for filter
    pub fn get_scale(&self) -> Complex32 {
        unsafe {
            let mut scale = Complex32::default();
            raw::fftfilt_cccf_get_scale(self.inner, scale.to_ptr_mut());
            scale
        }
    }

    /// execute the filter on internal buffer and coefficients
    ///  x      : pointer to input data array  [size: _n x 1]
    ///  y      : pointer to output data array [size: _n x 1]
    pub fn execute(&self, x: &[Complex32], y: &mut [Complex32]) {
        assert!(x.len() == y.len(), "x and y must be the same length");
        unsafe {
            raw::fftfilt_cccf_execute(self.inner, x.to_ptr() as _, y.to_ptr_mut());
        }
    }
}

fftfilt_impl!(
    FftFiltCrcf,
    (
        raw::fftfilt_crcf_print,
        raw::fftfilt_crcf_reset,
        raw::fftfilt_crcf_get_length,
        raw::fftfilt_crcf_destroy
    )
);

fftfilt_impl!(
    FftFiltCccf,
    (
        raw::fftfilt_cccf_print,
        raw::fftfilt_cccf_reset,
        raw::fftfilt_cccf_get_length,
        raw::fftfilt_cccf_destroy
    )
);

fftfilt_impl!(
    FftFiltRrrf,
    (
        raw::fftfilt_rrrf_print,
        raw::fftfilt_rrrf_reset,
        raw::fftfilt_rrrf_get_length,
        raw::fftfilt_rrrf_destroy
    )
);
