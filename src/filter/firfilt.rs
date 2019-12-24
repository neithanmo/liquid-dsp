use num::complex::Complex32;

use crate::liquid_dsp_sys as raw;
use crate::utils::{ToCPointer, ToCPointerMut, ToCValue};
use filter::IirdesFilterType;

use crate::errors::{ErrorKind, LiquidError};
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
    ($obj:ty, ($reset:expr,
        $print:expr,
        $glen:expr,
        $freq_response:expr,
        $group_delay:expr,
        $rect:expr,
        $dc_blocker:expr,
        $kaiser:expr,
        $rnyquist:expr,
        $notch:expr,
        $destroy:expr)) => {
        impl $obj {
            pub fn reset(&self) {
                unsafe {
                    $reset(self.inner)
                }
            }

            pub fn print(&self) {
                unsafe {
                    $print(self.inner)
                }
            }

            pub fn len(&self) -> usize {
                unsafe {
                    $glen(self.inner) as _
                }
            }

            pub fn freq_response(&self, fc: f32) -> Complex32 {
                let mut f = Complex32::default();
                unsafe {
                    $freq_response(self.inner, fc, f.to_ptr_mut());
                }
                f
            }

            pub fn group_delay(&self, fc: f32) -> f32 {
                unsafe {
                    $group_delay(self.inner, fc)
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

