use libc::c_uint;
use std::fmt;

use crate::liquid_dsp_sys as raw;
use crate::errors::LiquidError;
use crate::LiquidResult;
/// CVSD: continuously variable slope delta
pub struct Cvsd {
    inner: raw::cvsd,
    num_bits: u32,
    zeta: f32,
    alpha: f32,
}

impl Cvsd {

    /// create cvsd object
    ///  num_bits   :   number of adjacent bits to observe
    ///  zeta       :   slope adjustment multiplier
    ///  alpha      :   pre-/post-emphasis filter coefficient (0.9 recommended)
    /// NOTE: _alpha must be in [0,1]
    pub fn create(num_bits: u32, zeta: f32, alpha: f32) -> LiquidResult<Self> {
        if num_bits == 0 {
            return Err(LiquidError::InvalidValue(
                "num_bits must be positive".to_owned(),
            ));
        } else if zeta <= 1f32 {
            return Err(LiquidError::InvalidValue(
                "zeta must be greater than 1".to_owned(),
            ));
        } else if alpha < 0f32 || alpha > 1f32 {
            return Err(LiquidError::InvalidValue(
                "alpha must be in [0,1]".to_owned(),
            ));
        }
        unsafe {
            Ok(Self {
                inner: raw::cvsd_create(num_bits as c_uint, zeta, alpha),
                num_bits,
                alpha,
                zeta,
            })
        }
    }

    /// encode single sample
    pub fn encode(&self, audio_sample: f32) -> u8 {
        unsafe { raw::cvsd_encode(self.inner, audio_sample) as u8 }
    }

    /// decode single sample
    pub fn decode(&self, bit: u8) -> f32 {
        unsafe { raw::cvsd_decode(self.inner, bit) }
    }

    /// encode 8 audio samples
    pub fn encode8(&self, audio: &[f32]) -> u8 {
        assert!(audio.len() == 8usize, "Input must contain 8 audio samples");
        unsafe {
            let data = &mut 0u8 as *mut _;
            raw::cvsd_encode8(self.inner, audio.as_ptr() as *mut _, data);
            *data
        }
    }

    /// decode u8 data into 8 audio samples
    pub fn cvsd_decode8(&self, data: u8, audio: &mut [f32]) {
        assert!(
            audio.len() == 8usize,
            "Audio must have capacity for  8 audio samples"
        );
        unsafe {
            raw::cvsd_decode8(self.inner, data, audio.as_mut_ptr());
        }
    }
}

impl fmt::Debug for Cvsd {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "cvsd codec: num_bits: {}, zeta: {}, alpha: {}",
            self.num_bits, self.zeta, self.alpha
        )
    }
}

impl Drop for Cvsd {
    fn drop(&mut self) {
        unsafe {
            raw::cvsd_destroy(self.inner);
        }
    }
}
