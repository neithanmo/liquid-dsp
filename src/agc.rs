use libc::c_uint;
use num::complex::Complex32;
use std::fmt;

use crate::enums::AgcSquelchMode;
use crate::liquid_dsp_sys as raw;

use crate::errors::LiquidError;
use crate::utils::{ToCPointer, ToCPointerMut, ToCValue};
use crate::LiquidResult;

pub struct AgcCrcf {
    inner: raw::agc_crcf,
    is_locked: bool,
}

pub struct AgcRrrf {
    inner: raw::agc_rrrf,
    is_locked: bool,
}

macro_rules! agc_xxx_impl {
    ($obj:ty, ($create:expr, $init:expr,
        $reset:expr,
        $lock:expr, $unlock:expr,
        $setband:expr, $getband:expr,
        $setsignal:expr, $getsignal:expr,
        $setrssi:expr, $getrssi:expr,
        $setgain:expr, $getgain:expr,
        $setscale:expr, $getscale:expr,
        $squelche:expr, $squelchd:expr,
        $squelch:expr,$setthres:expr,
        $getthres:expr, $settimeout:expr,
        $gettimeout:expr, $status:expr,
        $execute:expr, $block:expr,
        $destroy:expr,
        $type:ty, $type2:ty)) => {
        impl $obj {
            pub fn create() -> Self {
                Self {
                    inner: unsafe { $create() },
                    is_locked: false,
                }
            }


            /// initialize internal gain on input array
            ///  x      : input data array, [size: _n x 1]
            pub fn init(&mut self, x: &mut[$type2]) -> LiquidResult<()> {
                if x.is_empty() {
                    return Err(LiquidError::InvalidValue(
                        "number of samples must be greater than zero".to_owned(),
                    ));
                }
                unsafe {
                    $init(self.inner, x.to_ptr_mut(), x.len() as c_uint);
                }
                Ok(())
            }

            pub fn reset(&mut self) {
                unsafe {
                    $reset(self.inner);
                }
            }

            pub fn lock(&mut self) {
                unsafe {
                    $lock(self.inner);
                    self.is_locked = true;
                }
            }

            pub fn unlock(&mut self) {
                unsafe {
                    $unlock(self.inner);
                    self.is_locked = false;
                }
            }

            /// set agc loop bandwidth
            ///  b     :   bandwidth 0 <= b <= 1.0
            pub fn set_bandwidth(&mut self, b: f32) -> LiquidResult<()> {
                if b < 0f32 || b > 1f32 {
                    return Err(LiquidError::InvalidValue(
                        "b must be in [0, 1.0]".to_owned(),
                    ));
                }
                unsafe {
                    $setband(self.inner, b);
                }
                Ok(())
            }

            /// get agc loop bandwidth
            pub fn get_bandwidth(&self) -> f32 {
                unsafe { $getband(self.inner) }
            }

            pub fn get_signal_level(&self) -> f32 {
                unsafe { $getsignal(self.inner) }
            }

            pub fn set_signal_level(&mut self, level: f32) -> LiquidResult<()> {
                if level <= 0f32 {
                    return Err(LiquidError::InvalidValue(
                        "level must be greater than zero".to_owned(),
                    ));
                }
                unsafe {
                    $setsignal(self.inner, level);
                }
                Ok(())
            }

            /// get estimated signal level (dB)
            pub fn get_rssi(&self) -> f32 {
                unsafe { $getrssi(self.inner) }
            }

            /// set estimated signal level (dB)
            pub fn set_rssi(&mut self, rssi: f32) {
                unsafe {
                    $setrssi(self.inner, rssi);
                }
            }

            /// get internal gain
            pub fn get_gain(&self) -> f32 {
                unsafe { $getgain(self.inner) }
            }

            /// set internal gain
            pub fn set_gain(&mut self, gain: f32) -> LiquidResult<()> {
                if gain <= 0f32 {
                    return Err(LiquidError::InvalidValue(
                        "gain must be greater than zero".to_owned(),
                    ));
                }
                unsafe {
                    $setgain(self.inner, gain);
                }
                Ok(())
            }

            /// get scale
            pub fn get_scale(&self) -> f32 {
                unsafe { $getscale(self.inner) }
            }

            /// set scale
            pub fn set_scale(&mut self, scale: f32) -> LiquidResult<()> {
                if scale <= 0f32 {
                    return Err(LiquidError::InvalidValue(
                        "scale must be greater than zero".to_owned(),
                    ));
                }
                unsafe {
                    $setscale(self.inner, scale);
                }
                Ok(())
            }

            /// enable squelch mode
            pub fn squelch_enable(&mut self) {
                unsafe {
                    $squelche(self.inner);
                }
            }

            /// disable squelch mode
            pub fn squelch_disable(&mut self) {
                unsafe {
                    $squelchd(self.inner);
                }
            }

            /// is squelch enabled?
            pub fn squelch_is_enabled(&self) -> bool {
                unsafe { $squelch(self.inner) == 1 }
            }

            /// set squelch threshold
            ///  th:   threshold for enabling squelch [dB]
            pub fn squelch_set_threshold(&self, th: f32) {
                unsafe {
                    $setthres(self.inner, th);
                }
            }

            /// get squelch threshold [dB]
            pub fn squelch_get_threshold(&self) -> f32 {
                unsafe { $getthres(self.inner) }
            }

            /// set squelch timeout
            ///  timeout : timeout before enabling squelch [samples]
            pub fn squelch_set_timeout(&self, timeout: u64) {
                unsafe {
                    $settimeout(self.inner, timeout as c_uint);
                }
            }

            pub fn squelch_get_timeout(&self) -> u64 {
                unsafe { $gettimeout(self.inner) as u64 }
            }

            pub fn squelch_status(&self) -> AgcSquelchMode {
                unsafe { AgcSquelchMode::from_bits($status(self.inner) as u8).unwrap() }
            }

            /// execute automatic gain control loop
            ///  x      :   input sample
            /// # Returns
            /// output sample
            pub fn execute(&self, x: $type2) -> $type2 {
                let mut ret = <$type2>::default();
                unsafe {
                    $execute(self.inner, x.to_c_value(), ret.to_ptr_mut());
                    ret
                }
            }

            /// execute automatic gain control on block of samples
            ///  x      : input data array, [size: _n x 1]
            ///  y      : output data array, [size: _n x 1]
            pub fn execute_block(&self, x: &[$type2], y: &mut[$type2]) {
                assert!(
                    x.len() == y.len(),
                    "Input and output buffers with different length"
                );
                unsafe {
                    $block(
                        self.inner,
                        x.to_ptr() as _,
                        x.len() as c_uint,
                        y.to_ptr_mut(),
                    );
                }
            }
        }

        impl fmt::Debug for $obj {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                let bandwith = self.get_bandwidth();
                let locked = if self.is_locked { "yes" } else { "no" };
                let status = match self.squelch_status() {
                    AgcSquelchMode::DISABLED => "disabled",
                    _ => "enabled",
                };
                let rssi = self.get_rssi();
                let scale = self.get_scale();
                let gain = if scale > 0f32 {
                    10.0 * scale.log10()
                } else {
                    -100.0
                };
                write!(
                    f,
                    "agc [rssi: {} dB, output gain: {} dB, bw: {}, locked: {}, squelch: {}]:\n",
                    rssi, gain, bandwith, locked, status
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

agc_xxx_impl!(
    AgcCrcf,
    (
        raw::agc_crcf_create,
        raw::agc_crcf_init,
        raw::agc_crcf_reset,
        raw::agc_crcf_lock,
        raw::agc_crcf_unlock,
        raw::agc_crcf_set_bandwidth,
        raw::agc_crcf_get_bandwidth,
        raw::agc_crcf_set_signal_level,
        raw::agc_crcf_get_signal_level,
        raw::agc_crcf_set_rssi,
        raw::agc_crcf_get_rssi,
        raw::agc_crcf_set_gain,
        raw::agc_crcf_get_gain,
        raw::agc_crcf_set_scale,
        raw::agc_crcf_get_scale,
        raw::agc_crcf_squelch_enable,
        raw::agc_crcf_squelch_disable,
        raw::agc_crcf_squelch_is_enabled,
        raw::agc_crcf_squelch_set_threshold,
        raw::agc_crcf_squelch_get_threshold,
        raw::agc_crcf_squelch_set_timeout,
        raw::agc_crcf_squelch_get_timeout,
        raw::agc_crcf_squelch_get_status,
        raw::agc_crcf_execute,
        raw::agc_crcf_execute_block,
        raw::agc_crcf_destroy,
        f32, Complex32
    )
);

agc_xxx_impl!(
    AgcRrrf,
    (
        raw::agc_rrrf_create,
        raw::agc_rrrf_init,
        raw::agc_rrrf_reset,
        raw::agc_rrrf_lock,
        raw::agc_rrrf_unlock,
        raw::agc_rrrf_set_bandwidth,
        raw::agc_rrrf_get_bandwidth,
        raw::agc_rrrf_set_signal_level,
        raw::agc_rrrf_get_signal_level,
        raw::agc_rrrf_set_rssi,
        raw::agc_rrrf_get_rssi,
        raw::agc_rrrf_set_gain,
        raw::agc_rrrf_get_gain,
        raw::agc_rrrf_set_scale,
        raw::agc_rrrf_get_scale,
        raw::agc_rrrf_squelch_enable,
        raw::agc_rrrf_squelch_disable,
        raw::agc_rrrf_squelch_is_enabled,
        raw::agc_rrrf_squelch_set_threshold,
        raw::agc_rrrf_squelch_get_threshold,
        raw::agc_rrrf_squelch_set_timeout,
        raw::agc_rrrf_squelch_get_timeout,
        raw::agc_rrrf_squelch_get_status,
        raw::agc_rrrf_execute,
        raw::agc_rrrf_execute_block,
        raw::agc_rrrf_destroy,
        f32, f32
    )
);

#[cfg(test)]
mod tests {
    use super::AgcCrcf;
    use num::complex::Complex32;
    use num::Zero;

    #[test]
    fn test_agc_crcf_execute_block() {
        let mut input = Vec::with_capacity(4);
        let mut output = vec![Complex32::zero(); 4];
        for i in 0..4 {
            input.push(Complex32::new(2.0 + i as f32 * 2.0, -2.8 * 0.5 * i as f32));
        }
        let mut agc = AgcCrcf::create();
        agc.set_bandwidth(0.001).unwrap();
        agc.set_gain(0.5).unwrap();
        agc.set_scale(1.5).unwrap();
        agc.squelch_enable();
        agc.execute_block(&input, &mut output);
        let solution = [
            Complex32::new(1.5, -0.0),
            Complex32::new(3.0, -1.05),
            Complex32::new(4.4999924, -2.0999963),
            Complex32::new(5.9999495, -3.1499734),
        ];
        assert_eq!(&output, &solution);
    }

    #[test]
    fn test_agc_crcf_rssi() {
        let agc = AgcCrcf::create();
        agc.execute(Complex32::new(5.9999495, -3.1499734));
        let rssi = agc.get_rssi();
        assert_eq!(0.016113421, rssi);
    }
}
