//! Least mean-squares (LMS) equalizer 

use num::complex::Complex32;
use crate::liquid_dsp_sys as raw;

use crate::utils::{ToCPointer, ToCPointerMut, ToCValue};

use crate::errors::LiquidError;
use crate::LiquidResult;
use std::slice;

pub struct EqlmsCccf(raw::eqlms_cccf);
pub struct EqlmsRrrf(raw::eqlms_rrrf);

macro_rules! eqlms_impl {
    ($obj:ty, ($create:expr,
        $lowpass:expr,
        $recreate:expr,
        $reset:expr,
        $print:expr,
        $getbw:expr,
        $setbw:expr,
        $push:expr,
        $block:expr,
        $execute:expr,
        $exeblock:expr,
        $step:expr,
        $stepblind:expr,
        $getweights:expr,
        $train:expr,
        $destroy:expr,
        $type:ty)) => {
        impl $obj {
            /// create least mean-squares (LMS) equalizer object
            ///  h      :   initial coefficients [size: _h_len x 1], default if NULL
            pub fn create(
                h: &[$type],
            ) -> LiquidResult<$obj> {
                if h.is_empty() {
                    return Err(LiquidError::InvalidValue(format!(
                        "initials coefficients must not be empty",
                    )));
                } 
                Ok(unsafe { Self($create(h.to_pointer() as _)) })
            }

            /// create LMS EQ initialized with low-pass filter
            ///  _n    : filter length
            ///   fc   : filter cut-off, _fc in (0,0.5]
            pub fn create_lowpass(n: u32, fc: f32) -> LiquidResult<$obj> {
                if n == 0 {
                    return Err(LiquidError::InvalidLength {
                        description: "filter length must be greater than zero".to_owned(),
                    });
                } else if fc <= 0f32 || fc > 0.5 {
                    return Err(LiquidError::InvalidValue(
                        "filter cutoff must be in (0,0.5]".to_owned(),
                    ));
                }
                unsafe {
                    Ok(Self {
                        $lowpass(n as _, fc)
                    })
                }
            }
            
            pub fn recreate(self, h: &[$type]) -> $obj {
                Ok(unsafe { self.0 = $recreate(h.to_pointer() as _) })
            }

            pub fn reset(&self) {
                unsafe { $reset(self.0) }
            }

            pub fn print(&self) {
                unsafe { $print(self.0) }
            }

            pub get_bw(&self) -> f32 {
                unsafe {
                    $getbw(self.0)
                }
            }

            /// set learning rate of equalizer
            ///  lambda     :   LMS learning rate (should be near 0), 0 < _mu < 1
            pub fn set_bw(&mut self, lambda: f32) -> LiquidResult<()> {
                if lambda < 0 {
                    return Err(LiquidError::InvalidValue (
                        "learning rate cannot be less than zero".to_owned(),
                    ));
                }
                unsafe{
                    Ok($setbw(self.0, lambda))
                }
            }

            /// push sample into equalizer internal buffer
            pub fn push(&mut self, x: $type) {
                unsafe {
                    $push(self.0, x.to_c_value());
                }
            }

            /// push sample into equalizer internal buffer as block
            ///  x      :   input sample array
            pub fn push_block(&mut self, x: &[$type]) {
                unsafe {
                    $block(self.0, x.to_ptr() as _);
                }
            }

            /// execute internal dot product
            pub fn execute(&self) -> $type {
                unsafe {
                    let mut out = <$type>::default();
                    $execute(self.0, out.to_ptr_mut());
                    out
                }
            }

            /// execute equalizer with block of samples using constant
            /// modulus algorithm, operating on a decimation rate of _k
            /// samples.
            ///  k      :   down-sampling rate
            ///  x      :   input sample array [size: _n x 1]
            ///  y      :   output sample array [size: _n x 1]
            pub fn execute_block(&self, k: i32, x: &[$type], y: &mut[$type]) -> LiquidResult<()> {
                unsafe {
                    assert!(x.len() == y.len());
                    if k == 0 {
                        return Err(LiquidError::InvalidValue (
                            "down-sampling rate 'k' must be greater than 0".to_owned(),
                        )); 
                    }
                    Ok($exeblock(self.0, k as _, x.to_ptr() as _, x.len() as _, y.to_ptr_mut()))
                }
            }

            /// step through one cycle of equalizer training
            ///  d      :   desired output
            ///  d_hat  :   filtered output
            pub fn step(&mut self, d: $type, d_hat: $type) {
                unsafe {
                    $step(self.0, d.to_c_value(), d_hat.to_c_value());
                }
            }

            /// step through one cycle of equalizer training
            ///  d_hat  :   filtered output
            pub fn step_blind(&mut self, d_hat) {
                unsafe {
                    $stepblind(self.0, d_hat.to_c_value());
                }

            }

            pub fn get_weights(&self) -> &[$type] {
                
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

eqlms_impl!(
    EqlmsRrrf,
    (
        raw::cpfskdem_create,
/*         raw::cpfskdem_reset,
        raw::cpfskdem_print,
        raw::cpfskdem_get_delay,
        raw::cpfskdem_destroy, */
        f32
    )
);

eqlms_impl!(
    EqlmsCccf,
    (
        raw::eqlms_cccf_create,
        raw::eqlms_cccf_lowpass,
        raw::eqlms_cccf_recreate,
        raw::eqlms_cccf_reset,
        raw::eqlms_cccf_print,
        raw::eqlms_cccf_destroy,
        Complex32
    )
);