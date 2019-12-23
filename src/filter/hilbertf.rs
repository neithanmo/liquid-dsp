use num::complex::Complex32;

use crate::liquid_dsp_sys as raw;
use crate::utils::{ToCPointer, ToCPointerMut, ToCValue};
use filter::IirdesFilterType;

use crate::errors::{ErrorKind, LiquidError};
use crate::LiquidResult;

/// infinite impulse response (IIR) Hilbert transform
pub struct IirHilbt {
    inner: raw::iirhilbf,
}

/// finite impulse response (FIR) Hilbert transform
pub struct FirHilbt {
    inner: raw::firhilbf,
}

macro_rules! hilbertimpl {
    ($obj:ty, ($print:expr,$reset:expr,
        $r2c:expr,$decim_execute:expr,
        $decim_block:expr,$interp_execute:expr,
        $interp_block:expr,
        $destroy:expr)) => {
        impl $obj {
            /// print iirhilb object internals
            pub fn print(&self) {
                unsafe {
                    $print(self.inner);
                }
            }

            /// reset iirhilb object internal state
            pub fn reset(&mut self) {
                unsafe {
                    $reset(self.inner);
                }
            }

            /// execute Hilbert transform (real to complex)
            ///  x      :   real-valued input sample
            pub fn r2c_execute(&self, x: f32) -> Complex32 {
                let mut y = Complex32::default();
                unsafe {
                    $r2c(self.inner, x, y.to_ptr_mut());
                }
                y
            }

            /// execute Hilbert transform decimator (real to complex)
            ///  x      :   real-valued input array [size: 2 x 1]
            pub fn decim_execute(&self, x: (f32, f32)) -> Complex32 {
                let mut y = Complex32::default();
                let x: *const f32 = &x.0;
                unsafe {
                    $decim_execute(self.inner, x as _, y.to_ptr_mut());
                }
                y
            }

            /// execute Hilbert transform decimator (real to complex) on
            /// a block of samples
            ///  x      :   real-valued input array [size: 2*_n x 1]
            ///  y      :   complex-valued output array [size: _n x 1]
            pub fn decim_execute_block(&self, x: &[f32], y: &mut [Complex32]) {
                assert!(
                    x.len() == 2 * y.len(),
                    "x must be 2 times more elements than y"
                );
                unsafe {
                    $decim_block(self.inner, x.as_ptr() as _, y.len() as _, y.to_ptr_mut());
                }
            }

            /// execute Hilbert transform interpolator (complex to real)
            ///  x      :   real-valued output array [size: 2 x 1]
            pub fn interp_execute(&self, x: Complex32) -> (f32, f32) {
                unsafe {
                    let mut y = (0f32, 0f32);
                    let ptr: *mut f32 = &mut y.0;
                    $interp_execute(self.inner, x.to_c_value(), ptr);
                    y
                }
            }

            /// execute Hilbert transform interpolator (complex to real)
            /// on a block of samples
            ///  x      :   complex-valued input array [size: _n x 1]
            ///  y      :   real-valued output array [size: 2*_n x 1]
            pub fn interp_execute_block(&self, x: &[Complex32], y: &mut [f32]) {
                assert!(
                    y.len() >= 2 * x.len(),
                    "y must have 2 times more elements than x"
                );
                unsafe {
                    $interp_block(self.inner, x.to_ptr() as _, x.len() as _, y.as_mut_ptr());
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

hilbertimpl!(
    IirHilbt,
    (
        raw::iirhilbf_print,
        raw::iirhilbf_reset,
        raw::iirhilbf_r2c_execute,
        raw::iirhilbf_decim_execute,
        raw::iirhilbf_decim_execute_block,
        raw::iirhilbf_interp_execute,
        raw::iirhilbf_interp_execute_block,
        raw::iirhilbf_destroy
    )
);

hilbertimpl!(
    FirHilbt,
    (
        raw::firhilbf_print,
        raw::firhilbf_reset,
        raw::firhilbf_r2c_execute,
        raw::firhilbf_decim_execute,
        raw::firhilbf_decim_execute_block,
        raw::firhilbf_interp_execute,
        raw::firhilbf_interp_execute_block,
        raw::firhilbf_destroy
    )
);

impl IirHilbt {
    /// create iirhilb object
    ///  ftype  : filter type (e.g. LIQUID_IIRDES_BUTTER)
    ///  n      : filter order, _n > 0
    ///  ap     : pass-band ripple [dB], ap > 0
    ///  as_    : stop-band ripple [dB], as_ > 0
    pub fn create(ftype: IirdesFilterType, n: usize, ap: f32, as_: f32) -> LiquidResult<Self> {
        if n == 0 {
            return Err(LiquidError::from(ErrorKind::InvalidValue(
                "filter order must be greater than 0".to_owned(),
            )));
        }
        let ftype: u8 = ftype.into();
        Ok(Self {
            inner: unsafe { raw::iirhilbf_create(ftype as _, n as _, ap, as_) },
        })
    }

    /// Create a default iirhilb object with a particular filter order.
    ///  n      : filter order, n > 0
    pub fn create_default(n: usize) -> LiquidResult<Self> {
        if n == 0 {
            return Err(LiquidError::from(ErrorKind::InvalidValue(
                "filter order must be greater than 0".to_owned(),
            )));
        }
        Ok(Self {
            inner: unsafe { raw::iirhilbf_create_default(n as _) },
        })
    }

    /// execute Hilbert transform (complex to real)
    ///  x      :   complex-valued input sample
    pub fn c2r_execute(&self, x: Complex32) -> f32 {
        let mut y = 0f32;
        unsafe {
            raw::iirhilbf_c2r_execute(self.inner, x.to_c_value(), &mut y as _);
        }
        y
    }
}

impl FirHilbt {
    /// create firhilb object
    ///  m      :   filter semi-length m > 2. note: (delay: 2*m+1)
    ///  as_    : stop-band ripple [dB], as_ > 0
    pub fn create(m: u32, as_: f32) -> LiquidResult<Self> {
        if m < 2 {
            return Err(LiquidError::from(ErrorKind::InvalidValue(
                "filter order must be greater than 0".to_owned(),
            )));
        }
        Ok(Self {
            inner: unsafe { raw::firhilbf_create(m as _, as_) },
        })
    }

    /// execute Hilbert transform (complex to real)
    ///  x      :   complex-valued input sample
    /// # returns
    /// a tuple (y0, y1) where:  
    /// y0     :   real-valued output sample, lower side-band retained
    /// y1     :   real-valued output sample, upper side-band retained
    pub fn c2r_execute(&self, x: Complex32) -> (f32, f32) {
        let mut y = (0f32, 0f32);
        let ptr0 = &mut y.0 as *mut f32;
        let ptr1 = &mut y.1 as *mut f32;
        unsafe {
            raw::firhilbf_c2r_execute(self.inner, x.to_c_value(), ptr0, ptr1);
        }
        y
    }
}
