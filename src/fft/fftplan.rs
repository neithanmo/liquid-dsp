use std::marker::PhantomData;
use libc::c_uint;

use num::complex::Complex32;

use crate::liquid_dsp_sys as raw;

use crate::utils::{ToCPointer, ToCPointerMut, ToCValue};
use crate::enums::{FftType};

pub struct FftPlan<'a> {
    inner: raw::fftplan,
    data: PhantomData<&'a Complex32>
}

impl<'a> FftPlan<'a> {
    pub fn create(x: &'a [Complex32], y: &'a mut [Complex32], direction: FftType) -> Result<Self, &'static str> {
        assert!(x.len() == y.len(), "x/y buffers must have the same size");
        if direction == FftType::FORDWARD || direction == FftType::BACKWARD {
            unsafe {
                return Ok(Self{
                    inner: raw::fft_create_plan(x.len() as _, x.to_c_ptr() as _, y.to_c_ptr_mut(), i8::from(direction) as _),
                })
            }
        }
        // TODO: check if this is really needed
        Err("Either FftType::FORWARD or FftType::BACKWARD are the only valid values for direction")
    }
    
    pub fn print(&self) {
        unsafe {
            raw::fft_print_plan(self.inner);
        }
    }
    
    pub fn execute(&self) {
        unsafe {
            raw::fft_execute(self.inner);
        }
    }


}

impl Drop for FftPlan {
    fn drop(&mut self) {
        unsafe {
            raw::fft_destroy_plan(self.inner);
        }
    }
}