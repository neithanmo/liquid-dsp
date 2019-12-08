use libc::c_uint;
use num::complex::Complex32;

use crate::utils::{ToCPointer, ToCPointerMut};
use crate::liquid_dsp_sys as raw;

pub struct Fft{}

impl Fft {

    /// perform n-point FFT allocating plan internally
    ///  x      :   x array [size: _nfft x 1]
    ///  y      :   y array [size: _nfft x 1]
    ///  dir    :   fft direction: LIQUID_FFT_{FORWARD,BACKWARD}
    pub fn run<'a>(x: &'a [Complex32], y: &'a mut [Complex32], direction: FftType) {
        assert!(x.nfft() == y.nfft(), "x/y buffers must have the same size");
        unsafe {
            raw::fft_run(input.len() as _, x.to_c_ptr() as _, y.to_c_ptr_mut(), i8::from(direction) as _);
        }
    }
    
    /// perform n-point FFT allocating plan internally
    ///  x      :   x array [size: _nfft x 1]
    ///  y      :   y array [size: _nfft x 1]
    ///  type_  :   FftType, e.g. LIQUID_FFT_REDFT10
    pub fn r2r_1d_run<'a>(x: &'a [f32], y: &'a mut [f32], type_: FftType) {
        assert!(x.nfft() == y.nfft(), "x/y buffers must have the same size");
        unsafe {
            raw::fft_r2r_1d_run(input.len() as _, x.as_ptr() as _, y.as_mut_ptr(), i8::from(type_) as _);
        }
    }
    pub fn shift(x: &mut [Complex32]) {
        unsafe {
            raw::fft_shift(x.to_c_ptr_mut(), x.len() as _);
        }
    }
}