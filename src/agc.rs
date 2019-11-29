
use crate::liquid_dsp_sys as raw;

pub struct AgcCrcf {
    inner: raw::agc_crcf,
}

impl AgcCrcf {
    pub fn create() -> Self {
        Self {
            inner: unsafe {
                raw::agc_crcf_create()
            },
        }
    }

    pub fn reset(&mut self) {
        unsafe {
            raw::agc_crcf_reset(self.inner);
        }
    }

    pub fn lock(&self) {
        unsafe {
            raw::agc_crcf_lock(self.inner);
        }
    }

    pub fn unlock(&self) {
        unsafe {
            raw::agc_crcf_unlock(self.inner);
        }
    }

    pub fn set_bandwidth(&mut self, b: f32) -> Result<(), &'static str> {
        if b < 0 as f32 {
            return Err("bandwith must be positive")
        }
        unsafe {
            raw::agc_crcf_set_bandwidth(self.inner, b);
        }
        Ok(())
    }

    pub fn get_bandwidth(&self) -> f32 {
        unsafe {
            raw::agc_crcf_get_bandwidth(self.inner)
        }      
    }

    pub fn get_signal_level(&self) -> f32 {
        unsafe {
            raw::agc_crcf_get_signal_level(self.inner)
        }
    }

    pub fn set_signal_level(&mut self, level: f32) -> Result<(), &'static str> {
        if level <= 0 as f32 {
            return Err("level must be greater than zero")
        }
        unsafe {
            raw::agc_crcf_set_signal_level(self.inner, level);
        }
        Ok(())
    }
    pub fn get_rssi(&self) -> f32 {
        unsafe {
            raw::agc_crcf_get_rssi(self.inner)
        }
    }

    pub fn set_rssi(&mut self, rssi: f32) {
        unsafe {
            raw::agc_crcf_set_rssi(self.inner, rssi);
        }
    }

    pub fn get_gain(&self) -> f32 {
        unsafe {
            raw::agc_crcf_get_gain(self.inner)
        }
    }

    pub fn set_gain(&mut self, gain: f32) -> Result<(), &'static str> {
        if gain <= 0 as f32 {
            return Err("gain must be greater than zero")
        }
        unsafe {
            raw::agc_crcf_set_gain(self.inner, gain);
        }
        Ok(())
    }

    pub fn get_scale(&self) -> f32 {
        unsafe {
            raw::agc_crcf_get_scale(self.inner)
        }
    }

    pub fn set_scale(&mut self, scale: f32) -> Result<(), &'static str> {
        if scale <= 0 as f32 {
            return Err("scale must be greater than zero")
        }
        unsafe{ 
            raw::agc_crcf_set_scale(self.inner, scale);
        }
        Ok(())
    }

    /* pub fn init(&mut self, input: &[Complex32]) -> Result<Ok(()), &'static str> {
        if input.len() == 0 {
            return Err("number of samples must be greater than zero")
        }
        unsafe {
            raw::agc_crcf_init(self.inner,)
        }
    } */
}