use num::complex::Complex32;

use crate::liquid_dsp_sys as raw;
use crate::utils::{ToCPointer, ToCPointerMut, ToCValue};
use filter::{IirdesBandType, IirdesFilterType, IirdesFormat};

use crate::errors::LiquidError;
use crate::LiquidResult;

pub struct IirFiltRrrf {
    inner: raw::iirfilt_rrrf,
}

pub struct IirFiltCrcf {
    inner: raw::iirfilt_crcf,
}

pub struct IirFiltCccf {
    inner: raw::iirfilt_cccf,
}

macro_rules! iirfilt_impl {
    ($obj:ty, ($create_prototype:expr,
        $create_lowpass:expr,
        $create_integrator:expr,
        $create_differentiator:expr,
        $create_dc_blocker:expr, 
        $create_pll:expr,
        $print:expr,
        $reset:expr, 
        $len:expr, 
        $freq_response:expr,
        $group_delay:expr,
        $destroy:expr)) => {
        impl $obj {
            pub fn create_prototype(
                ftype: IirdesFilterType,
                btype: IirdesBandType,
                format: IirdesFormat,
                order: usize,
                fc: f32,
                f0: f32,
                ap: f32,
                as_: f32,
            ) -> LiquidResult<Self> {
                if fc <= 0f32 || fc >= 0.5 {
                    return Err(LiquidError::InvalidValue(
                        "fc must be in (0, 0.5)".to_owned(),
                    ));
                } else if f0 < 0f32 || f0 > 0.5 {
                    return Err(LiquidError::InvalidValue(
                        "f0 must be in [0, 0.5]".to_owned(),
                    ));
                } else if ap <= 0f32 {
                    return Err(LiquidError::InvalidValue(
                        "ap must be greater than 0".to_owned(),
                    ));
                } else if as_ <= 0f32 {
                    return Err(LiquidError::InvalidValue(
                        "as(stop-band ripple) must be greater than 0".to_owned(),
                    ));
                } else if order == 0 {
                    return Err(LiquidError::InvalidValue(
                        "order must be greater than 0".to_owned(),
                    ));
                }
                let ftype: u8 = ftype.into();
                let btype: u8 = btype.into();
                let format: u8 = format.into();
                let filter = unsafe {
                    $create_prototype(
                        ftype as _,
                        btype as _,
                        format as _,
                        order as _,
                        fc,
                        f0,
                        ap,
                        as_,
                    )
                };

                Ok(Self { inner: filter })
            }

            pub fn create_lowpass(n: usize, fc: f32) -> LiquidResult<Self> {
                if fc <= 0f32 || fc >= 0.5 {
                    return Err(LiquidError::InvalidValue(
                        "fc must be in (0, 0.5)".to_owned(),
                    ));
                } else if n == 0 {
                    return Err(LiquidError::InvalidValue(
                        "order must be greater than 0".to_owned(),
                    ));
                }

                Ok(Self {
                    inner: unsafe { $create_lowpass(n as _, fc) },
                })
            }

            pub fn create_integrator() -> Self {
                Self {
                    inner: unsafe { $create_integrator() },
                }
            }

            pub fn create_differentiator() -> Self {
                Self {
                    inner: unsafe { $create_differentiator() },
                }
            }

            pub fn create_dc_blocker(alpha: f32) -> LiquidResult<Self> {
                if alpha <= 0f32 {
                    return Err(LiquidError::InvalidValue(
                        "alpha must be greater than 0".to_owned(),
                    ));
                }
                Ok(Self {
                    inner: unsafe { $create_dc_blocker(alpha) },
                })
            }

            pub fn create_pll(w: f32, zeta: f32, k: f32) -> LiquidResult<Self> {
                if w <= 0f32 || w >= 1.0 {
                    return Err(LiquidError::InvalidValue(
                        "bandwidth must be in (0, 1.0)".to_owned(),
                    ));
                } else if zeta <= 0f32 || zeta >= 1.0 {
                    return Err(LiquidError::InvalidValue(
                        "damping factor must be in (0, 0.5)".to_owned(),
                    ));
                } else if k <= 0f32 {
                    return Err(LiquidError::InvalidValue(
                        "loop gain must be greater than 0".to_owned(),
                    ));
                }
                Ok(Self {
                    inner: unsafe { $create_pll(w, zeta, k) },
                })
            }

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

            pub fn freq_response(&self, fc: f32) -> Complex32 {
                let mut freq = Complex32::default();
                unsafe {
                    $freq_response(self.inner, fc, freq.to_ptr_mut());
                }
                freq
            }

            pub fn group_delay(&self, fc: f32) -> f32 {
                unsafe { $group_delay(self.inner, fc) }
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

iirfilt_impl!(
    IirFiltCccf,
    (
        raw::iirfilt_cccf_create_prototype,
        raw::iirfilt_cccf_create_lowpass,
        raw::iirfilt_cccf_create_integrator,
        raw::iirfilt_cccf_create_differentiator,
        raw::iirfilt_cccf_create_dc_blocker,
        raw::iirfilt_cccf_create_pll,
        raw::iirfilt_cccf_print,
        raw::iirfilt_cccf_reset,
        raw::iirfilt_cccf_get_length,
        raw::iirfilt_cccf_freqresponse,
        raw::iirfilt_cccf_groupdelay,
        raw::iirfilt_cccf_destroy
    )
);

iirfilt_impl!(
    IirFiltCrcf,
    (
        raw::iirfilt_crcf_create_prototype,
        raw::iirfilt_crcf_create_lowpass,
        raw::iirfilt_crcf_create_integrator,
        raw::iirfilt_crcf_create_differentiator,
        raw::iirfilt_crcf_create_dc_blocker,
        raw::iirfilt_crcf_create_pll,
        raw::iirfilt_crcf_print,
        raw::iirfilt_crcf_reset,
        raw::iirfilt_crcf_get_length,
        raw::iirfilt_crcf_freqresponse,
        raw::iirfilt_crcf_groupdelay,
        raw::iirfilt_crcf_destroy
    )
);

iirfilt_impl!(
    IirFiltRrrf,
    (
        raw::iirfilt_rrrf_create_prototype,
        raw::iirfilt_rrrf_create_lowpass,
        raw::iirfilt_rrrf_create_integrator,
        raw::iirfilt_rrrf_create_differentiator,
        raw::iirfilt_rrrf_create_dc_blocker,
        raw::iirfilt_rrrf_create_pll,
        raw::iirfilt_rrrf_print,
        raw::iirfilt_rrrf_reset,
        raw::iirfilt_rrrf_get_length,
        raw::iirfilt_rrrf_freqresponse,
        raw::iirfilt_rrrf_groupdelay,
        raw::iirfilt_rrrf_destroy
    )
);

impl IirFiltRrrf {
    /// create iirfilt (infinite impulse response filter) object
    ///  b      :   numerator, feed-forward coefficients
    ///  a      :   denominator, feed-back coefficients
    pub fn create(a: &[f32], b: &[f32]) -> LiquidResult<Self> {
        if b.len() == 0 {
            return Err(LiquidError::InvalidValue(
                "numerator length cannot be zero".to_owned(),
            ));
        }
        if a.len() == 0 {
            return Err(LiquidError::InvalidValue(
                "denominator length cannot be zero".to_owned(),
            ));
        }

        Ok(Self {
            inner: unsafe {
                raw::iirfilt_rrrf_create(
                    b.as_ptr() as _,
                    b.len() as _,
                    a.as_ptr() as _,
                    a.len() as _,
                )
            },
        })
    }

    /// create iirfilt (infinite impulse response filter) object based
    /// on second-order sections form
    ///  b      :   numerator, feed-forward coefficients [size: _nsos x 3]
    ///  a      :   denominator, feed-back coefficients  [size: _nsos x 3]
    ///  nsos   :   number of second-order sections
    ///
    /// NOTE: The number of second-order sections can be computed from the
    /// filter's order, n, as such:
    ///   r = n % 2
    ///   L = (n-r)/2
    ///   nsos = L+r
    pub fn create_sos(a: &[f32], b: &[f32], nsos: usize) -> LiquidResult<Self> {
        if a.len() != b.len() {
            return Err(LiquidError::InvalidLength {
                description: "numerator and denominator slices must have the same size".to_owned(),
            });
        } else if a.len() == 0 || a.len() < (3 * nsos) {
            return Err(LiquidError::InvalidLength {
                description:
                    "numerator and denominator lengt cannot be zero or lesser than 3 * nsos"
                        .to_owned(),
            });
        } else {
            Ok(Self {
                inner: unsafe {
                    raw::iirfilt_rrrf_create_sos(b.as_ptr() as _, a.as_ptr() as _, a.len() as _)
                },
            })
        }
    }

    pub fn execute(&self, input: f32) -> f32 {
        let mut output = 0f32;
        unsafe {
            raw::iirfilt_rrrf_execute(self.inner, input, output.to_ptr_mut());
        }
        output
    }

    /// execute the filter on a block of input samples; the
    /// input and output buffers may be the same
    ///  input      : pointer to input array [size: _n x 1]
    ///  output      : pointer to output array [size: _n x 1]
    pub fn execute_block(&self, input: &[f32], output: &mut [f32]) {
        assert_eq!(input.len(), output.len());
        unsafe {
            raw::iirfilt_rrrf_execute_block(
                self.inner,
                input.as_ptr() as _,
                input.len() as _,
                output.as_mut_ptr(),
            );
        }
    }
}

impl IirFiltCrcf {
    /// create iirfilt (infinite impulse response filter) object
    ///  b      :   numerator, feed-forward coefficients
    ///  a      :   denominator, feed-back coefficients
    pub fn create(a: &[f32], b: &[f32]) -> LiquidResult<Self> {
        if b.len() == 0 {
            return Err(LiquidError::InvalidValue(
                "numerator length cannot be zero".to_owned(),
            ));
        }
        if a.len() == 0 {
            return Err(LiquidError::InvalidValue(
                "denominator length cannot be zero".to_owned(),
            ));
        }

        Ok(Self {
            inner: unsafe {
                raw::iirfilt_crcf_create(
                    b.as_ptr() as _,
                    b.len() as _,
                    a.as_ptr() as _,
                    a.len() as _,
                )
            },
        })
    }

    /// create iirfilt (infinite impulse response filter) object based
    /// on second-order sections form
    ///  b      :   numerator, feed-forward coefficients [size: _nsos x 3]
    ///  a      :   denominator, feed-back coefficients  [size: _nsos x 3]
    ///  nsos   :   number of second-order sections
    ///
    /// NOTE: The number of second-order sections can be computed from the
    /// filter's order, n, as such:
    ///   r = n % 2
    ///   L = (n-r)/2
    ///   nsos = L+r
    pub fn create_sos(a: &[f32], b: &[f32], nsos: usize) -> LiquidResult<Self> {
        if a.len() != b.len() {
            return Err(LiquidError::InvalidLength {
                description: "numerator and denominator slices must have the same size".to_owned(),
            });
        } else if a.len() == 0 || a.len() < (3 * nsos) {
            return Err(LiquidError::InvalidLength {
                description:
                    "numerator and denominator lengt cannot be zero or lesser than 3 * nsos"
                        .to_owned(),
            });
        } else {
            Ok(Self {
                inner: unsafe {
                    raw::iirfilt_crcf_create_sos(b.as_ptr() as _, a.as_ptr() as _, a.len() as _)
                },
            })
        }
    }

    /// execute iir filter, switching to type-specific function
    ///  input      :   input sample
    pub fn execute(&self, input: Complex32) -> Complex32 {
        let mut output = Complex32::default();
        unsafe {
            raw::iirfilt_crcf_execute(self.inner, input.to_c_value(), output.to_ptr_mut());
        }
        output
    }

    /// execute the filter on a block of input samples; the
    /// input and output buffers may be the same
    ///  input      : pointer to input array [size: _n x 1]
    ///  output      : pointer to output array [size: _n x 1]
    pub fn execute_block(&self, input: &[Complex32], output: &mut [Complex32]) {
        assert_eq!(input.len(), output.len());
        unsafe {
            raw::iirfilt_crcf_execute_block(
                self.inner,
                input.to_ptr() as _,
                input.len() as _,
                output.to_ptr_mut(),
            );
        }
    }
}

impl IirFiltCccf {
    /// create iirfilt (infinite impulse response filter) object
    ///  b      :   numerator, feed-forward coefficients
    ///  a      :   denominator, feed-back coefficients
    pub fn create(a: &[Complex32], b: &[Complex32]) -> LiquidResult<Self> {
        if b.len() == 0 {
            return Err(LiquidError::InvalidValue(
                "numerator length cannot be zero".to_owned(),
            ));
        }
        if a.len() == 0 {
            return Err(LiquidError::InvalidValue(
                "denominator length cannot be zero".to_owned(),
            ));
        }

        Ok(Self {
            inner: unsafe {
                raw::iirfilt_cccf_create(
                    b.to_ptr() as _,
                    b.len() as _,
                    a.to_ptr() as _,
                    a.len() as _,
                )
            },
        })
    }

    /// create iirfilt (infinite impulse response filter) object based
    /// on second-order sections form
    ///  b      :   numerator, feed-forward coefficients [size: _nsos x 3]
    ///  a      :   denominator, feed-back coefficients  [size: _nsos x 3]
    ///  nsos   :   number of second-order sections
    ///
    /// NOTE: The number of second-order sections can be computed from the
    /// filter's order, n, as such:
    ///   r = n % 2
    ///   L = (n-r)/2
    ///   nsos = L+r
    pub fn create_sos(a: &[Complex32], b: &[Complex32], nsos: usize) -> LiquidResult<Self> {
        if a.len() != b.len() {
            return Err(LiquidError::InvalidLength {
                description: "numerator and denominator slices must have the same size".to_owned(),
            });
        } else if a.len() == 0 || a.len() < (3 * nsos) {
            return Err(LiquidError::InvalidLength {
                description:
                    "numerator and denominator length cannot be zero or lesser than 3 * nsos"
                        .to_owned(),
            });
        } else {
            Ok(Self {
                inner: unsafe {
                    raw::iirfilt_cccf_create_sos(b.to_ptr() as _, a.to_ptr() as _, a.len() as _)
                },
            })
        }
    }

    /// execute iir filter, switching to type-specific function
    ///  input      :   input sample
    pub fn execute(&self, input: Complex32) -> Complex32 {
        let mut output = Complex32::default();
        unsafe {
            raw::iirfilt_cccf_execute(self.inner, input.to_c_value(), output.to_ptr_mut());
        }
        output
    }

    /// execute the filter on a block of input samples; the
    /// input and output buffers may be the same
    ///  input      : pointer to input array [size: _n x 1]
    ///  output      : pointer to output array [size: _n x 1]
    pub fn execute_block(&self, input: &[Complex32], output: &mut [Complex32]) {
        assert_eq!(input.len(), output.len());
        unsafe {
            raw::iirfilt_cccf_execute_block(
                self.inner,
                input.to_ptr() as _,
                input.len() as _,
                output.to_ptr_mut(),
            );
        }
    }
}
