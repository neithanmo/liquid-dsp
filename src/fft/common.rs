use num::complex::Complex32;

use crate::enums::FftType;
use crate::liquid_dsp_sys as raw;
use crate::utils::{ToCPointer, ToCPointerMut};

pub struct Fft {}

impl Fft {
    /// perform n-point FFT allocating plan internally
    ///  x      :   x array [size: n]
    ///  y      :   y array [size: n]
    ///  dir    :   fft direction: LIQUID_FFT_{FORWARD,BACKWARD}
    pub fn run<'a>(x: &'a [Complex32], y: &'a mut [Complex32], direction: FftType) {
        assert!(x.len() == y.len(), "x/y buffers must have the same size");
        unsafe {
            raw::fft_run(
                x.len() as _,
                x.to_ptr() as _,
                y.to_ptr_mut(),
                i8::from(direction) as _,
                0,
            );
        }
    }

    /// perform n-point FFT allocating plan internally
    ///  x      :   x array [size: n]
    ///  y      :   y array [size: n]
    ///  type_  :   FftType, e.g. LIQUID_FFT_REDFT10
    pub fn r2r_1d_run<'a>(x: &'a [f32], y: &'a mut [f32], type_: FftType) {
        assert!(x.len() == y.len(), "x/y buffers must have the same size");
        unsafe {
            raw::fft_r2r_1d_run(
                x.len() as _,
                x.as_ptr() as _,
                y.as_mut_ptr(),
                i8::from(type_) as _,
                0,
            );
        }
    }

    pub fn shift(x: &mut [Complex32]) {
        unsafe {
            raw::fft_shift(x.to_ptr_mut(), x.len() as _);
        }
    }
}
