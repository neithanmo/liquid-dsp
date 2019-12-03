use libc::{c_int, c_uint};
use std::fmt;

use num::complex::Complex32;
use std::mem::transmute;

use crate::enums::{AmpModemType};
use crate::liquid_dsp_sys as raw;

use crate::utils::{LiquidFloatComplex};


pub struct AmpModem {
    inner: raw::ampmodem,
    suppressed_carrier: bool,
    index: f32,
    modem_type: AmpModemType,
}

impl AmpModem {
    pub fn create(index: f32, modem_type: AmpModemType, suppressed_carrier: i32) -> Self {
        unsafe {
            Self {
                inner: raw::ampmodem_create(index, u32::from(modem_type) as c_uint, suppressed_carrier as c_int),
                index,
                suppressed_carrier: suppressed_carrier != 0,
                modem_type, 
            }
        }
    }

    pub fn reset(&mut self) {
        unsafe {
            raw::ampmodem_reset(self.inner);
        }
    }

    pub fn get_delay_mod(&self) -> u32 {
        unsafe {
            raw::ampmodem_get_delay_mod(self.inner) as u32
        }
    }

    pub fn get_delay_demod(&self) -> u32 {
        unsafe {
            raw::ampmodem_get_delay_demod(self.inner) as u32
        }
    }

    pub fn modulate(&self, sample: f32) -> Complex32 {
        let mut out = Complex32::default();
        let ptr = &mut out as *mut Complex32;
        unsafe {
            raw::ampmodem_modulate(self.inner, sample, ptr as *mut LiquidFloatComplex);
            *ptr
        }
    }

    pub fn modulate_block(&self, samples:&[f32], output: &mut[Complex32]) {
        assert!(
            samples.len() == output.len(),
            "Input and output buffers with different length"
        );
        unsafe {
            raw::ampmodem_modulate_block(self.inner, samples.as_ptr() as *mut f32, samples.len() as c_uint, 
                transmute::<*mut Complex32, *mut LiquidFloatComplex>(output.as_mut_ptr()));
        }
    }

    pub fn demodulate(&self, sample: Complex32) -> f32 {
        let ptr = &mut 0f32 as *mut f32;
        unsafe {
            raw::ampmodem_demodulate(self.inner, *transmute::<*const Complex32, *const LiquidFloatComplex>(&sample as *const _), ptr);
            *ptr
        }
    }
    
    pub fn demodulate_block(&self, samples: &mut[Complex32], output: &mut [f32]) {
        assert!(
            samples.len() == output.len(),
            "Input and output buffers with different length"
        );
        unsafe {
            raw::ampmodem_demodulate_block(self.inner, transmute::<*mut Complex32, *mut LiquidFloatComplex>(samples.as_mut_ptr()), 
                samples.len() as c_uint, output.as_mut_ptr());
        }
    }

}

impl fmt::Debug for AmpModem {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "ampmodem [index: {} , type: {:?} , suppressed_carrier: {}]:\n",
             self.index, self.modem_type, self.suppressed_carrier
        )
    }
}

impl Drop for AmpModem {
    fn drop(&mut self) {
        unsafe {
            raw::ampmodem_destroy(self.inner);
        }
    }
}