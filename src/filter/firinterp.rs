
#![allow(non_camel_case_types, non_upper_case_globals, non_snake_case)]
use num::complex::Complex32;

use crate::liquid_dsp_sys as raw;
use crate::utils::{ToCPointer, ToCPointerMut, ToCValue};

use crate::errors::LiquidError;
use crate::filter::enums::FirdesFilterType;
use crate::LiquidResult;

pub struct FirInterpRrrf {
    inner: raw::firinterp_rrrf,
    len: usize,
}

pub struct FirInterpCrcf {
    inner: raw::firinterp_crcf,
    len: usize,
}

pub struct FirInterpCccf {
    inner: raw::firinterp_cccf,
    len: usize,
}

macro_rules! firinterp_impl {
    ($obj:ty, ($create:expr, $prototype:expr,
        $kaiser:expr,
        $print:expr,$reset:expr,
        $scale:expr, $get_scale:expr,
        $execute:expr, $block:expr,
        $destroy:expr,
        $type:ty, $type2:ty)) => {
        impl $obj {
            /// create interpolator
            ///  m      :   interpolation factor
            ///  h      :   filter coefficients array, size >= m
            pub fn create(m: u32, h: &[$type2]) -> LiquidResult<$obj> {
                // check params
                if m < 2 {
                    return Err(LiquidError::InvalidValue(
                        "interp factor must be greater than 2".to_owned(),
                    ));
                } else if h.len() < m as usize {
                    return Err(LiquidError::InvalidValue(
                        "filter length cannot be less than interp factor".to_owned(),
                    ));
                }
                Ok(Self {
                    inner: unsafe { $create(m as _, h.to_ptr() as _, h.len() as _) },
                    len: h.len(),
                })
            }

            /// create prototype (root-)Nyquist interpolator
            ///  type_  :   filter type (e.g. LIQUID_NYQUIST_RCOS)
            ///  k      :   samples/symbol,          k > 1
            ///  m      :   filter delay (symbols),  m > 0
            ///  beta   :   excess bandwidth factor, beta < 1
            ///  dt     :   fractional sample delay, dt in (-1, 1)
            pub fn create_prototype(
                type_: FirdesFilterType,
                k: u32,
                m: u32,
                beta: f32,
                dt: f32,
            ) -> LiquidResult<Self> {
                // check params
                if k < 1 {
                    return Err(LiquidError::InvalidValue(
                        "interp factor must be greater than 1".to_owned(),
                    ));
                } else if m <= 0 {
                    return Err(LiquidError::InvalidValue(
                        "filter delay must be greater than 0".to_owned(),
                    ));
                } else if beta < 0f32 || beta > 1f32 {
                    return Err(LiquidError::InvalidValue(
                        "filter excess bandwidth factor must be in [0,1]".to_owned(),
                    ));
                } else if dt < -1f32 || dt > 1f32 {
                    return Err(LiquidError::InvalidValue(
                        "filter fractional sample delay must be in [-1,1]".to_owned(),
                    ));
                }
                unsafe {
                    let t: u8 = type_.into();
                    Ok(Self {
                        inner: $prototype(t as _, k as _, m as _, beta, dt),
                        len: (2 * k * m + 1) as usize,
                    })
                }
            }

            /// create interpolator from Kaiser prototype
            ///  M      :   interpolation factor , m > 2
            ///  m      :   symbol delay, m > 0
            ///  as_    :   stop-band attenuation [dB], as_ > 0
            pub fn create_kaiser(M: u32, m: u32, as_: f32) -> LiquidResult<Self> {
                // check params
                if M < 2 {
                    return Err(LiquidError::InvalidValue(
                        "interp factor must be greater than 2".to_owned(),
                    ));
                } else if m <= 0 {
                    return Err(LiquidError::InvalidValue(
                        "filter delay must be greater than 0".to_owned(),
                    ));
                } else if as_ < 0f32 {
                    return Err(LiquidError::InvalidValue(
                        "stop-band attenuation must be positive".to_owned(),
                    ));
                }

                Ok(Self {
                    inner: unsafe { $kaiser(M as _, m as _, as_) },
                    len: (2 * M * m + 1) as usize,
                })
            }

            /// print to stdout a firinterp object internals
            pub fn print(&self) {
                unsafe {
                    $print(self.inner);
                }
            }

            /// reset firinterp object internal state
            pub fn reset(&mut self) {
                unsafe {
                    $reset(self.inner);
                }
            }

            pub fn set_scale(&mut self, scale: $type2) -> LiquidResult<()> {
                unsafe {
                    $scale(self.inner, scale.to_c_value() as _);
                    Ok(())
                }
            }

            pub fn get_scale(&self) -> $type2 {
                let mut res = <$type2>::default();
                unsafe {
                    $get_scale(self.inner, res.to_ptr_mut());
                }
                res
            }

            /// Get the filters length
            pub fn len(&self) -> usize {
                self.len
            }

            // execute interpolator
            //  q      : interpolator object
            //  x      : input sample
            //  y      : output array
            pub fn execute(&self, x: $type, y: &mut [$type]) {
                assert!(
                    y.len() == self.len,
                    "y.len() is not equal to the filter length"
                );
                unsafe {
                    $execute(self.inner, x.to_c_value(), y.to_ptr_mut());
                }
            }

            /// execute interpolation on block of input samples
            pub fn execute_block(&self, x: &[$type], y: &mut [$type]) {
                assert!(x.len() == y.len(), "x and y must have same length");
                unsafe {
                    $block(self.inner, x.to_ptr() as _, x.len() as _, y.to_ptr_mut());
                }
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

firinterp_impl!(
    FirInterpRrrf,
    (
        raw::firinterp_rrrf_create,
        raw::firinterp_rrrf_create_prototype,
        raw::firinterp_rrrf_create_kaiser,
        raw::firinterp_rrrf_print,
        raw::firinterp_rrrf_reset,
        raw::firinterp_rrrf_set_scale,
        raw::firinterp_rrrf_get_scale,
        raw::firinterp_rrrf_execute,
        raw::firinterp_rrrf_execute_block,
        raw::firinterp_rrrf_destroy,
        f32,
        f32
    )
);

firinterp_impl!(
    FirInterpCrcf,
    (
        raw::firinterp_crcf_create,
        raw::firinterp_crcf_create_prototype,
        raw::firinterp_crcf_create_kaiser,
        raw::firinterp_crcf_print,
        raw::firinterp_crcf_reset,
        raw::firinterp_crcf_set_scale,
        raw::firinterp_crcf_get_scale,
        raw::firinterp_crcf_execute,
        raw::firinterp_crcf_execute_block,
        raw::firinterp_crcf_destroy,
        Complex32,
        f32
    )
);

firinterp_impl!(
    FirInterpCccf,
    (
        raw::firinterp_cccf_create,
        raw::firinterp_cccf_create_prototype,
        raw::firinterp_cccf_create_kaiser,
        raw::firinterp_cccf_print,
        raw::firinterp_cccf_reset,
        raw::firinterp_cccf_set_scale,
        raw::firinterp_cccf_get_scale,
        raw::firinterp_cccf_execute,
        raw::firinterp_cccf_execute_block,
        raw::firinterp_cccf_destroy,
        Complex32,
        Complex32
    )
);

#[cfg(test)]
mod tests {
    use super::FirInterpRrrf;

    #[test]
    fn test_execute_rrrf() {
        let h = [2.0; 6];
        let firinterp_rrrf = FirInterpRrrf::create(6, &h).unwrap();
        let mut res = vec![0f32; firinterp_rrrf.len()];
        firinterp_rrrf.execute(0.5, &mut res);
        println!("res {:?}", res);
        assert_eq!(res, vec![1.0; firinterp_rrrf.len()]);
    }
}
