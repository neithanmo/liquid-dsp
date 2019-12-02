use num::complex::Complex32;

use crate::liquid_dsp_sys as raw;

pub(crate) trait ToRaw<T> {
    unsafe fn to_raw(&mut self) -> *mut T;
}

pub(crate) type LiquidFloatComplex = raw::liquid_float_complex;

#[derive(Debug, Clone)]
pub(crate) struct LiquidComplex(pub(crate) LiquidFloatComplex);

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
