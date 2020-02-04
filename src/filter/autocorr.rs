use std::fmt;

use crate::liquid_dsp_sys as raw;
use num::complex::Complex32;

use crate::utils::{ToCPointerMut, ToCValue};

pub struct AutoCorrRrrf {
    inner: raw::autocorr_rrrf,
    window: u32,
    delay: u32,
}

pub struct AutoCorrCccf {
    inner: raw::autocorr_cccf,
    window: u32,
    delay: u32,
}

macro_rules! autocorr_xxx_impl {
    ($obj:ty, ($create:expr, $reset:expr,
        $destroy:expr,
        $push:expr, $execute:expr,
        $block:expr,$energy:expr,
        $type:ty, $type2:ty)) => {
        impl $obj {
            /// create auto-correlator object                            
            ///  n    : size of the correlator window         
            ///  d    : correlator delay [samples]
            pub fn create(n: u32, d: u32) -> Self {
                Self {
                    inner: unsafe { $create(n, d) },
                    window: n,
                    delay: d,
                }
            }

            pub fn reset(&mut self) {
                unsafe {
                    $reset(self.inner);
                }
            }

            /// push sample into auto-correlator object
            pub fn push(&self, sample: $type2) {
                unsafe {
                    $push(self.inner, sample.to_c_value());
                }
            }

            /// compute auto-correlation output
            pub fn execute(&self) -> $type2 {
                unsafe {
                    let mut out = <$type2>::default();
                    $execute(self.inner, out.to_ptr_mut());
                    out
                }
            }

            /// compute auto-correlation on block of samples; the input
            /// and output arrays may have the same pointer
            ///  input      :   input array [size: _n x 1]
            ///  output     :   input array [size: _n x 1]
            pub fn execute_block(&self, input: &[$type2], output: &mut [$type2]) {
                assert!(
                    input.len() == output.len(),
                    "Input and output buffers with different length"
                );
                input
                    .iter()
                    .zip(output.iter_mut())
                    .for_each(|(isample, osample)| {
                        self.push(*isample);
                        unsafe {
                            $execute(self.inner, osample.to_ptr_mut());
                        }
                    });
            }

            pub fn get_energy(&self) -> $type {
                unsafe { $energy(self.inner) }
            }
        }

        /// return sum of squares of buffered samples
        impl fmt::Debug for $obj {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                write!(f, "autocorr [{} window, {} delay]", self.window, self.delay)
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

autocorr_xxx_impl!(
    AutoCorrCccf,
    (
        raw::autocorr_cccf_create,
        raw::autocorr_cccf_reset,
        raw::autocorr_cccf_destroy,
        raw::autocorr_cccf_push,
        raw::autocorr_cccf_execute,
        raw::autocorr_cccf_execute_block,
        raw::autocorr_cccf_get_energy,
        f32, Complex32
    )
);

autocorr_xxx_impl!(
    AutoCorrRrrf,
    (
        raw::autocorr_rrrf_create,
        raw::autocorr_rrrf_reset,
        raw::autocorr_rrrf_destroy,
        raw::autocorr_rrrf_push,
        raw::autocorr_rrrf_execute,
        raw::autocorr_rrrf_execute_block,
        raw::autocorr_rrrf_get_energy,
        f32, f32
    )
);

#[cfg(test)]
mod tests {
    use super::{AutoCorrCccf, AutoCorrRrrf};
    use num::complex::Complex32;
    use num::Zero;

    #[test]
    fn test_autocorr_cccf() {
        let auto_cccf = AutoCorrCccf::create(16, 8);
        assert_eq!(&format!("{:?}", auto_cccf), "autocorr [16 window, 8 delay]");
    }

    #[test]
    fn test_autocorr_cccf_execute_block() {
        let mut input = Vec::with_capacity(4);
        let mut output = vec![Complex32::zero(); 4];
        for i in 0..4 {
            input.push(Complex32::new(0.0 + i as f32, 4.5 - i as f32 * (-1.0)));
        }
        let auto_cccf = AutoCorrCccf::create(4, 0);
        auto_cccf.execute_block(&input, &mut output);
        let solution = [
            Complex32::new(20.25, 0.0),
            Complex32::new(51.50, 0.0),
            Complex32::new(97.75, 0.0),
            Complex32::new(163.0, 0.0),
        ];
        assert_eq!(&output, &solution);
    }

    #[test]
    fn test_autocorr_rrrf() {
        let auto_rrrf = AutoCorrRrrf::create(16, 8);
        assert_eq!(&format!("{:?}", auto_rrrf), "autocorr [16 window, 8 delay]");
    }

    #[test]
    fn test_autocorr_rrrf_execute_block() {
        let mut input = Vec::with_capacity(4);
        let mut output = vec![0.0f32; 4];
        let auto_rrrf = AutoCorrRrrf::create(4, 0);
        for i in 0..4 {
            input.push(4.5 - i as f32 * (-1.0));
        }
        auto_rrrf.execute_block(&input, &mut output);
        let solution = [20.25, 50.5, 92.75, 149.0];

        assert_eq!(&output, &solution);
    }
}
