use libc::c_uint;
use std::fmt;

use num::complex::Complex32;
use std::mem::transmute;

use crate::liquid_dsp_sys as raw;

use crate::utils::{LiquidFloatComplex};


pub struct CbufferRf {
    inner: raw::cbufferf,
    num_elements:u32,
    len: u32,
}

pub struct CbufferCf {
    inner: raw::cbuffercf,
    num_elements:u32,
    len: u32,
}

macro_rules! cbuffer_xxx_impl {
    ($obj:ty, ($create:expr, $create_max:expr,
        $reset:expr, $size:expr,
        $max_size:expr,$max_read:expr,
        $space_available:expr,$is_full:expr,

        $destroy:expr)) => {
        impl $obj {
            pub fn create(max_size: u32) -> Self {
                Self {
                    inner: unsafe { $create(max_size as _) },
                    num_elements: 0,
                    len: 0,
                }
            }

            pub fn create_max(max_size: u32, max_read: u32,) -> Self {
                Self {
                    inner: unsafe { $create_max(max_size as _, max_read as _) },
                    num_elements: 0,
                    len: 0,
                }
            }

            pub fn reset(&mut self) {
                unsafe {
                    $reset(self.inner);
                }
            }

            pub fn size(&mut self) -> u32 {
                unsafe {
                    $size(self.inner) as u32
                }
            }

            pub fn max_size(&mut self) -> u32 {
                unsafe {
                    $max_size(self.inner) as u32
                }
            }

            pub fn max_read(&mut self) -> u32 {
                unsafe {
                    $max_read(self.inner) as u32
                }
            }

            pub fn space_available(&mut self) -> u32 {
                unsafe {
                    $space_available(self.inner) as u32
                }
            }

            // TODO check it
            pub fn is_full(&self) -> bool {
                unsafe {
                    $is_full(self.inner) == 1
                }
            }
        }

        impl fmt::Debug for $obj {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                write!(
                    f,
                    "agc [rssi: {}]:\n",
                    10
                )
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

cbuffer_xxx_impl!(
    CbufferRf,
    (
        raw::cbufferf_create,
        raw::cbufferf_create_max,
        raw::cbufferf_reset,
        raw::cbufferf_size,
        raw::cbufferf_max_size,
        raw::cbufferf_max_read,
        raw::cbufferf_space_available,
        raw::cbufferf_is_full,

        raw::cbufferf_destroy,
    )
);

cbuffer_xxx_impl!(
    CbufferRf,
    (
        raw::cbuffercf_create,
        raw::cbuffercf_create_max,
        raw::cbuffercf_reset,
        raw::cbuffercf_size,
        raw::cbuffercf_max_size,
        raw::cbuffercf_max_read,
        raw::cbuffercf_space_available,
        raw::cbuffercf_is_full,

        raw::cbufferf_destroy,
    )
);