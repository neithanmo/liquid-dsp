use std::ffi::{CString, NulError};
use std::str;

use num::complex::Complex32;

use crate::errors::LiquidError;
use crate::liquid_dsp_sys as raw;
use crate::utils::{ToCPointer, ToCPointerMut, ToCValue};

pub struct AsgramCf {
    inner: raw::asgramcf,
    ascii: Vec<u8>,
}

pub struct AsgramRf {
    inner: raw::asgramf,
    ascii: Vec<u8>,
}

macro_rules! asgram_xxx_impl {
    ($obj:ty, (
        $create:expr, $reset:expr,
        $setscale:expr, $setdisplay:expr,
        $print:expr,
        $push:expr,
        $write:expr,
        $execute:expr,
        $destroy:expr,
        $type:ty, $type2:ty)) => {
        impl $obj {
            pub fn create(nfft: u32) -> Result<Self, LiquidError> {
                if nfft < 2 {
                    return Err(LiquidError::InvalidValue(format!(
                        "nfft size must be at least {}",
                        2
                    )));
                }
                Ok(Self {
                    inner: unsafe { $create(nfft as _) },
                    ascii: vec![b'\n'; nfft as usize + 1],
                })
            }

            pub fn reset(&mut self) {
                unsafe {
                    $reset(self.inner);
                }
            }

            pub fn set_scale(&mut self, ref_: f32, div: f32) {
                assert!(div > 0f32, "div must be greater than zero");
                unsafe {
                    $setscale(self.inner, ref_, div);
                }
            }

            pub fn set_display(&mut self, ascii: &str) -> Result<(), NulError> {
                CString::new(ascii).and_then(|c| unsafe {
                    $setdisplay(self.inner, c.as_ptr() as *const _);
                    Ok(())
                })
            }

            /// print asgram object properties and internal state
            pub fn print(&self) {
                unsafe {
                    $print(self.inner);
                }
            }

            pub fn push(&mut self, x: $type2) {
                unsafe {
                    $push(self.inner, x.to_c_value());
                }
            }

            pub fn write(&mut self, x: &[$type2]) {
                unsafe {
                    $write(self.inner, x.to_ptr() as _, x.len() as _);
                }
            }

            // TODO: Need a deep check
            /// compute spectral periodogram output from current buffer contents
            ///  _ascii      :   output ASCII string [size: _nfft x 1]
            ///  _peakval    :   value at peak (returned value)
            ///  _peakfreq   :   frequency at peak (returned value)
            pub fn execute(&mut self) -> (&str, f32, f32) {
                let mut peak = 0f32;
                let mut peakf = 0f32;
                unsafe {
                    $execute(
                        self.inner,
                        self.ascii.as_mut_ptr() as _,
                        peak.to_ptr_mut(),
                        peakf.to_ptr_mut(),
                    );
                }
                let string = str::from_utf8(&self.ascii).unwrap_or(" ");
                (string, peak, peakf)
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

asgram_xxx_impl!(
    AsgramCf,
    (
        raw::asgramcf_create,
        raw::asgramcf_reset,
        raw::asgramcf_set_scale,
        raw::asgramcf_set_display,
        raw::asgramcf_print,
        raw::asgramcf_push,
        raw::asgramcf_write,
        raw::asgramcf_execute,
        raw::asgramcf_destroy,
        f32, Complex32
    )
);

asgram_xxx_impl!(
    AsgramRf,
    (
        raw::asgramf_create,
        raw::asgramf_reset,
        raw::asgramf_set_scale,
        raw::asgramf_set_display,
        raw::asgramf_print,
        raw::asgramf_push,
        raw::asgramf_write,
        raw::asgramf_execute,
        raw::asgramf_destroy,
        f32, f32
    )
);

/* #[cfg(test)]
mod tests {
    use super::{AsgramRf};
    extern crate rand;
    use rand::Rng;

    #[test]
    fn test_asgramr_execute() {
        let mut rng = rand::thread_rng();
        // options
         let nfft    =   72;    // FFT size (display)
         let buf_len = 3456;    // input buffer size

        // create spectral periodogram and set scale
        let mut g = AsgramRf::create(nfft);
        g.set_scale(-80.0, 5.0);
        g.set_display("...++++###"); // set custom display characters

        // allocated memory arrays
        let mut buff = vec![0f32; buf_len as usize];
        for i in buff.iter_mut() {
            *i = rng.gen::<f32>() + rng.gen::<f32>();
        }
        // ... initialize input ...

        // write block of samples to spectral periodogram object
        g.write(&buff);
        println!("{:?}", buff);

        // print result to screen
        g.print();

        let (s, peak, peakf) = g.execute();
        println!("ascii: {}, peak: {}, freq: {}", s, peak, peakf);

    }
} */
