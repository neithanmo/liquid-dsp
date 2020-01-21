use crate::liquid_dsp_sys as raw;

use crate::LiquidResult;
use crate::errors::LiquidError;

pub use filter::{FilterAnalysis, IirdesBandType, IirdesFilterType};

pub struct Iir {
    a: Vec<f32>,
    b: Vec<f32>,
}

pub struct Iirdes {}

impl Iir {
    fn new(na: usize, nb: usize) -> Self {
        Self {
            a: vec![0f32; na],
            b: vec![0f32; nb],
        }
    }

    pub fn is_stable(&self) -> bool {
        unsafe {
            raw::iirdes_isstable(
                self.b.as_ptr() as _,
                self.a.as_ptr() as _,
                self.a.len() as _,
            ) == 1
        }
    }
}

impl Iirdes {
    /// Compute frequency pre-warping factor.  See [Constantinides:1967]
    ///  btype  :   band type (e.g. IirdesBandType::HIGHPASS)
    ///  fc     :   low-pass cutoff frequency
    ///  f0     :   center frequency (band-pass|stop cases only)
    pub fn freq_prewarp(btype: IirdesBandType, fc: f32, f0: f32) -> LiquidResult<f32> {
        if fc <= 0f32 {
            return Err(LiquidError::InvalidValue(
                "fc must  be higher than zero".to_owned(),
            ));
        }
        let btype: u8 = btype.into();
        unsafe { Ok(raw::iirdes_freqprewarp(btype as _, fc, f0)) }
    }

    /// design 2nd-order IIR filter (active lag)
    ///          1 + t2 * s
    ///  F(s) = ------------
    ///          1 + t1 * s
    ///
    ///  w      :   filter bandwidth
    ///  zeta   :   damping factor (1/sqrt(2) suggested)
    ///  K      :   loop gain (1000 suggested)
    pub fn pll_active_lag(w: f32, zeta: f32, k: f32) -> LiquidResult<Iir> {
        if w <= 0f32 {
            return Err(LiquidError::InvalidValue(
                "w must be greater than zero".to_owned(),
            ));
        } else if zeta <= 0f32 {
            return Err(LiquidError::InvalidValue(
                "zeta must be greater than zero".to_owned(),
            ));
        } else if k <= 0f32 {
            return Err(LiquidError::InvalidValue(
                "k must be greater than zero".to_owned(),
            ));
        }

        let mut iir = Iir::new(3, 3);
        unsafe {
            raw::iirdes_pll_active_lag(w, zeta, k, iir.b.as_mut_ptr(), iir.a.as_mut_ptr());
        }
        Ok(iir)
    }

    /// design 2nd-order IIR filter (active PI)
    ///          1 + t2 * s
    ///  F(s) = ------------
    ///           t1 * s
    ///
    ///  w      :   filter bandwidth
    ///  zeta   :   damping factor (1/sqrt(2) suggested)
    ///  K      :   loop gain (1000 suggested)
    pub fn pll_active_pi(w: f32, zeta: f32, k: f32) -> LiquidResult<Iir> {
        if w <= 0f32 {
            return Err(LiquidError::InvalidValue(
                "w must be greater than zero".to_owned(),
            ));
        } else if zeta <= 0f32 {
            return Err(LiquidError::InvalidValue(
                "zeta must be greater than zero".to_owned(),
            ));
        } else if k <= 0f32 {
            return Err(LiquidError::InvalidValue(
                "k must be greater than zero".to_owned(),
            ));
        }
        let mut iir = Iir::new(3, 3);
        unsafe {
            raw::iirdes_pll_active_PI(w, zeta, k, iir.b.as_mut_ptr(), iir.a.as_mut_ptr());
        }
        Ok(iir)
    }
}
