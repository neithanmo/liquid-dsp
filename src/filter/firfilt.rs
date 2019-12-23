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
        impl $obj {}

        impl Drop for $obj {
            fn drop(&mut self) {
                unsafe {
                    $destroy(self.inner);
                }
            }
        }
    };
}
