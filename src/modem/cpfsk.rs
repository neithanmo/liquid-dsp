use libc::{c_int, c_uint};
use std::fmt;

use num::complex::Complex32;

use modem::AmpModemType;
use crate::liquid_dsp_sys as raw;

use crate::utils::{ToCPointer, ToCPointerMut, ToCValue};

use crate::errors::LiquidError;
use crate::LiquidResult;

pub struct CpfskDem(raw::cpfskdem);
pub struct CpfskMod(raw::cpfskmod);

impl CpfskDem {

    /// demodulate array of samples
    ///  y      :   input sample array [size: _k x 1]
    /// # Returns
    /// Demodulated symbol
    pub fn demodulate(&self, y: &[Complex32]) -> u32 {
        unsafe {
            raw::cpfskdem_demodulate(self.0, y.to_ptr() as _) as _
        }
    }
}


impl CpfskMod {

    /// modulate sample
    ///  s      :   input symbol
    ///  y      :   output sample array [size: _k x 1]
    pub fn modulate(&self, s: u32, y: &mut [Complex32]) {
        unsafe {
            raw::cpfskmod_modulate(self.0, s as _, y.to_ptr_mut() as _);
        }
    }
}





macro_rules! cpfsk_impl {
    ($obj:ty, ($create:expr,
        $reset:expr,
        $print:expr,
        $delay:expr,
        $destroy:expr)) => {
        impl $obj {
            /// create cpfsk object (frequency demodulator)
            ///  bps    :   bits per symbol, _bps > 0
            ///  h      :   modulation index, _h > 0
            ///  k      :   samples/symbol, _k > 1, _k even
            ///  m      :   filter delay (symbols), _m > 0
            ///  beta   :   filter bandwidth parameter, _beta > 0
            ///  type_  :   filter type (e.g. LIQUID_CPFSK_SQUARE)
            pub fn create(bps: u32, h: f32, k: u32,  m: u32, beta: f32, type_: i32) -> LiquidResult<$obj> {
                if bps == 0 || m == 0 {
                    return Err(LiquidError::InvalidValue(format!("bps: {} and m: {} must be higher that 0", bps, m)))
                } else if k < 2 || k % 2 != 0 {
                    return Err(LiquidError::InvalidValue(format!("k {}  must be higher than 2 and even", k)))
                } else if beta <= 0f32 || beta >= 1f32 {
                    return Err(LiquidError::InvalidValue(format!("beta: {} must be in (0, 1.0)", beta)))
                } else if h <= 0.0 {
                    return Err(LiquidError::InvalidValue(format!("h: {}  must be higher than 0", h)))
                }
            
                Ok(
                    unsafe {
                        Self($create(bps as _, h, k as _, m as _, beta, type_ as _))
                    }
                )
            }

            pub fn reset(&self) {
                unsafe { $reset(self.0) }
            }

            pub fn print(&self) {
                unsafe { $print(self.0) }
            }

            pub fn get_delay(&self) -> usize {
                unsafe { $delay(self.0) as _ }
            }
        }


        impl Drop for $obj {
            fn drop(&mut self) {
                unsafe {
                    $destroy(self.0);
                }
            }
        }
    };
}

cpfsk_impl!(
    CpfskDem,
    (
        raw::cpfskdem_create,
        raw::cpfskdem_reset,
        raw::cpfskdem_print,
        raw::cpfskdem_get_delay,
        raw::cpfskdem_destroy
    )
);

cpfsk_impl!(
    CpfskMod,
    (
        raw::cpfskmod_create,
        raw::cpfskmod_reset,
        raw::cpfskmod_print,
        raw::cpfskmod_get_delay,
        raw::cpfskmod_destroy
    )
);