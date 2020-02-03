use num::complex::Complex32;
use std::panic::{self, AssertUnwindSafe};

use crate::liquid_dsp_sys as raw;

pub(crate) type LiquidFloatComplex = raw::liquid_float_complex;

pub(crate) trait ToCPointer {
    type Output;
    fn to_ptr(&self) -> Self::Output;
}

pub(crate) trait ToCPointerMut {
    type Output;
    fn to_ptr_mut(&mut self) -> Self::Output;
}

pub(crate) trait ToCValue {
    type Output;
    fn to_c_value(self) -> Self::Output;
}

impl ToCPointer for Complex32 {
    type Output = *const LiquidFloatComplex;
    fn to_ptr(&self) -> Self::Output {
        self as *const _ as _
    }
}

impl ToCPointerMut for Complex32 {
    type Output = *mut LiquidFloatComplex;
    fn to_ptr_mut(&mut self) -> Self::Output {
        self as *mut _ as _
    }
}

impl ToCPointer for [Complex32] {
    type Output = *const LiquidFloatComplex;
    fn to_ptr(&self) -> Self::Output {
        self.as_ptr() as _
    }
}

impl ToCPointerMut for [Complex32] {
    type Output = *mut LiquidFloatComplex;
    fn to_ptr_mut(&mut self) -> Self::Output {
        self.as_mut_ptr() as _
    }
}

impl ToCValue for Complex32 {
    type Output = LiquidFloatComplex;
    fn to_c_value(self) -> Self::Output {
        LiquidFloatComplex {
            re: self.re,
            im: self.im,
        }
    }
}

impl ToCValue for f32 {
    type Output = Self;
    fn to_c_value(self) -> f32 {
        self
    }
}

impl ToCPointer for f32 {
    type Output = *const f32;
    fn to_ptr(&self) -> Self::Output {
        self as *const _
    }
}

impl ToCPointerMut for f32 {
    type Output = *mut f32;
    fn to_ptr_mut(&mut self) -> Self::Output {
        self as _
    }
}

impl ToCPointer for [f32] {
    type Output = *const f32;
    fn to_ptr(&self) -> Self::Output {
        self.as_ptr()
    }
}

impl ToCPointerMut for [f32] {
    type Output = *mut f32;
    fn to_ptr_mut(&mut self) -> Self::Output {
        self.as_mut_ptr()
    }
}

pub(crate) fn catch<T, F: FnOnce() -> T>(f: F) -> Option<T> {
    match panic::catch_unwind(AssertUnwindSafe(f)) {
        Ok(ret) => Some(ret),
        Err(_) => {
            std::process::exit(-1);
        }
    }
}
