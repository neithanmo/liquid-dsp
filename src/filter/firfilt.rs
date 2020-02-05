use num::complex::Complex32;

use crate::filter::FirdesFilterType;
use crate::liquid_dsp_sys as raw;
use crate::utils::{ToCPointer, ToCPointerMut, ToCValue};

use crate::errors::LiquidError;
use crate::LiquidResult;

pub struct FirFiltRrrf {
    inner: raw::firfilt_rrrf,
}

pub struct FirFiltCrcf {
    inner: raw::firfilt_crcf,
}

pub struct FirFiltCccf {
    inner: raw::firfilt_cccf,
}

macro_rules! firfilt_impl {
    ($obj:ty, ($create:expr,
        $recreate:expr, $reset:expr,
        $print:expr,
        $glen:expr,
        $freq_response:expr,
        $group_delay:expr,
        $rect:expr,
        $dc_blocker:expr,
        $kaiser:expr,
        $rnyquist:expr,
        $notch:expr,
        $setscale:expr, $getscale:expr,
        $push:expr, $write:expr,
        $execute:expr, $block:expr,
        $destroy:expr,
        $type:ty, $type2:ty)) => {
        impl $obj {
            // Creates firfilt object
            //  h      :  filter coefficients.
            pub fn create(h: &[$type]) -> LiquidResult<Self> {
                if h.is_empty() {
                    return Err(LiquidError::InvalidValue(
                        "filter length must be greater than zero".to_owned(),
                    ));
                }
                Ok(Self {
                    inner: unsafe { $create(h.to_ptr() as _, h.len() as _) },
                })
            }

            // re-create firfilt object
            //  h      :   new coefficients.
            pub fn recreate(self, h: &[$type]) -> LiquidResult<Self> {
                if h.is_empty() {
                    return Err(LiquidError::InvalidValue(
                        "filter length must be greater than zero".to_owned(),
                    ));
                }
                unsafe {
                    $recreate(self.inner, h.to_ptr() as _, h.len() as _);
                };
                Ok(self)
            }
            pub fn create_rect(n: usize) -> LiquidResult<Self> {
                if n == 0 {
                    return Err(LiquidError::InvalidValue(
                        "filter order must be greater than zero".to_owned(),
                    ));
                }

                Ok(Self {
                    inner: unsafe { $rect(n as _) },
                })
            }

            pub fn create_kaiser(n: usize, fc: f32, as_: f32, mu: f32) -> LiquidResult<Self> {
                if n == 0 {
                    return Err(LiquidError::InvalidValue(
                        "filter order must be greater than zero".to_owned(),
                    ));
                }

                Ok(Self {
                    inner: unsafe { $kaiser(n as _, fc, as_, mu) },
                })
            }

            pub fn create_rnyquist(
                ftype: FirdesFilterType,
                k: u32,
                m: u32,
                beta: f32,
                mu: f32,
            ) -> LiquidResult<Self> {
                if k < 2 {
                    return Err(LiquidError::InvalidValue(
                        "filter samples/symbol must be greater than 1".to_owned(),
                    ));
                } else if m == 0 {
                    return Err(LiquidError::InvalidValue(
                        "filter delay must be greater than zero".to_owned(),
                    ));
                } else if beta < 0f32 || beta > 1.0 {
                    return Err(LiquidError::InvalidValue(
                        "filter excess bandwith factor must be in [0, 1.0]".to_owned(),
                    ));
                } else if mu < -0.5 || mu > 0.5 {
                    return Err(LiquidError::InvalidValue(
                        "filter fractional sample offser factor must be in [-0.5, 0.5]".to_owned(),
                    ));
                } else {
                    let ftype: u8 = ftype.into();
                    Ok(Self {
                        inner: unsafe { $rnyquist(ftype as _, k as _, m as _, beta, mu) },
                    })
                }
            }

            pub fn create_notch(m: u16, as_: f32, f0: f32) -> LiquidResult<Self> {
                if m < 1 || m > 1000 {
                    return Err(LiquidError::InvalidValue(
                        "filter semi-length must be in [1, 1000]".to_owned(),
                    ));
                } else if as_ < 0f32 {
                    return Err(LiquidError::InvalidValue(
                        "filter prototype stop-band suppression be greater than zero".to_owned(),
                    ));
                } else if f0 < -0.5 || f0 > 0.5 {
                    return Err(LiquidError::InvalidValue(
                        "filter notch frequency must be in [-0.5, 0.5]".to_owned(),
                    ));
                } else {
                    Ok(Self {
                        inner: unsafe { $notch(m as _, as_, f0) },
                    })
                }
            }

            pub fn reset(&self) {
                unsafe { $reset(self.inner) }
            }

            pub fn print(&self) {
                unsafe { $print(self.inner) }
            }

            pub fn len(&self) -> usize {
                unsafe { $glen(self.inner) as _ }
            }

            pub fn freq_response(&self, fc: f32) -> Complex32 {
                let mut f = Complex32::default();
                unsafe {
                    $freq_response(self.inner, fc, f.to_ptr_mut());
                }
                f
            }

            pub fn group_delay(&self, fc: f32) -> f32 {
                unsafe { $group_delay(self.inner, fc) }
            }

            /// set output scaling for filter
            pub fn set_scale(&mut self, scale: $type) {
                unsafe {
                    $setscale(self.inner, scale.to_c_value());
                }
            }

            /// get output scaling for filter
            pub fn get_scale(&self) -> $type {
                unsafe {
                    let mut scale = <$type>::default();
                    $getscale(self.inner, scale.to_ptr_mut());
                    scale
                }
            }

            /// push sample into filter object's internal buffer
            ///  sample      :   input sample
            pub fn push(&mut self, sample: $type2) {
                unsafe {
                    $push(self.inner, sample.to_c_value());
                }
            }

            /// Write block of samples into filter object's internal buffer
            ///  samples      : buffer of input samples, [size: _n x 1]
            pub fn write(&mut self, samples: &[$type]) {
                unsafe {
                    $write(self.inner, samples.to_ptr() as _, samples.len() as _);
                }
            }

            /// compute output sample (dot product between internal
            /// filter coefficients and internal buffer)
            /// # returns
            ///  y      :   output sample
            pub fn execute(&self) -> $type2 {
                unsafe {
                    let mut y = <$type2>::default();
                    $execute(self.inner, y.to_ptr_mut());
                    y
                }
            }

            /// execute the filter on a block of input samples; the
            /// input and output buffers may be the same
            ///  x      : pointer to input array [size: _n x 1]
            ///  y      : pointer to output array [size: _n x 1]
            pub fn execute_block(&self, x: &[$type2], y: &mut [$type2]) {
                assert!(x.len() == y.len(), "x and y buffers must have the same len");
                unsafe {
                    $block(
                        self.inner,
                        x.to_ptr() as _,
                        x.len() as _,
                        y.to_ptr_mut(),
                    );
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

firfilt_impl!(
    FirFiltCccf,
    (
        raw::firfilt_cccf_create,
        raw::firfilt_cccf_recreate,
        raw::firfilt_cccf_reset,
        raw::firfilt_cccf_print,
        raw::firfilt_cccf_get_length,
        raw::firfilt_cccf_freqresponse,
        raw::firfilt_cccf_groupdelay,
        raw::firfilt_cccf_create_rect,
        raw::firfilt_cccf_create_dc_blocker,
        raw::firfilt_cccf_create_kaiser,
        raw::firfilt_cccf_create_rnyquist,
        raw::firfilt_cccf_create_notch,
        raw::firfilt_cccf_set_scale,
        raw::firfilt_cccf_get_scale,
        raw::firfilt_cccf_push,
        raw::firfilt_cccf_write,
        raw::firfilt_cccf_execute,
        raw::firfilt_cccf_execute_block,
        raw::firfilt_cccf_destroy,
        Complex32, Complex32
    )
);

firfilt_impl!(
    FirFiltCrcf,
    (
        raw::firfilt_crcf_create,
        raw::firfilt_crcf_recreate,
        raw::firfilt_crcf_reset,
        raw::firfilt_crcf_print,
        raw::firfilt_crcf_get_length,
        raw::firfilt_crcf_freqresponse,
        raw::firfilt_crcf_groupdelay,
        raw::firfilt_crcf_create_rect,
        raw::firfilt_crcf_create_dc_blocker,
        raw::firfilt_crcf_create_kaiser,
        raw::firfilt_crcf_create_rnyquist,
        raw::firfilt_crcf_create_notch,
        raw::firfilt_crcf_set_scale,
        raw::firfilt_crcf_get_scale,
        raw::firfilt_crcf_push,
        raw::firfilt_crcf_write,
        raw::firfilt_crcf_execute,
        raw::firfilt_crcf_execute_block,
        raw::firfilt_crcf_destroy,
        f32, Complex32
    )
);

firfilt_impl!(
    FirFiltRrrf,
    (
        raw::firfilt_rrrf_create,
        raw::firfilt_rrrf_recreate,
        raw::firfilt_rrrf_reset,
        raw::firfilt_rrrf_print,
        raw::firfilt_rrrf_get_length,
        raw::firfilt_rrrf_freqresponse,
        raw::firfilt_rrrf_groupdelay,
        raw::firfilt_rrrf_create_rect,
        raw::firfilt_rrrf_create_dc_blocker,
        raw::firfilt_rrrf_create_kaiser,
        raw::firfilt_rrrf_create_rnyquist,
        raw::firfilt_rrrf_create_notch,
        raw::firfilt_rrrf_set_scale,
        raw::firfilt_rrrf_get_scale,
        raw::firfilt_rrrf_push,
        raw::firfilt_rrrf_write,
        raw::firfilt_rrrf_execute,
        raw::firfilt_rrrf_execute_block,
        raw::firfilt_rrrf_destroy,
        f32, f32
    )
);
