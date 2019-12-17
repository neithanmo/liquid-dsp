use crate::enums::FirFilterType;
use crate::liquid_dsp_sys as raw;
use crate::errors::{LiquidError, ErrorKind};
use filter::FilterAnalysis;

use crate::utils::ToCPointerMut;
use crate::LiquidResult;

#[derive(Debug)]
pub struct Fir{
    h: Vec<f32>,
}

impl Fir {
    pub fn new(len: usize) -> Self {
        Self {
            h:  vec![0f32; len],
        }
    }

    pub fn len(&self) -> usize {
        self.h.len()
    }

    /// Compute group delay for a FIR filter
    ///  fc     : frequency at which delay is evaluated (-0.5 < _fc < 0.5)
    pub fn group_delay(&self,  fc: f32) -> LiquidResult<f32> {
        if fc < -0.5 || fc > 0.5 {
            return Err(LiquidError::from(ErrorKind::InvalidValue(
                format!("fc must be in [0, 0.5]")
            )))
        }
        unsafe {
            Ok(raw::fir_group_delay(self.as_ref().as_ptr() as _, self.len() as _, fc)) 
        }
    }
}

impl AsRef<[f32]> for Fir {
    fn as_ref(&self) -> &[f32] {
        self.h.as_slice()
    }
}

impl AsMut<[f32]> for Fir {
    fn as_mut(&mut self) -> &mut [f32] {
        self.h.as_mut_slice()
    }
}

impl FilterAnalysis for Fir {

    fn auto_corr(&self, lag: usize) -> f32 {
        unsafe {
            raw::liquid_filter_autocorr(self.as_ref().as_ptr() as _, self.as_ref().len() as _, lag as _)
        }
    }

    fn cross_corr(&self, filter: &Self, lag: usize) -> f32 {
        unsafe {
            raw::liquid_filter_crosscorr(self.as_ref().as_ptr() as _, self.as_ref().len() as _, filter.as_ref().as_ptr() as _, filter.as_ref().len() as _, lag as _)
        }
    }
    
    fn isi(&self, k: usize, m: usize,) ->  (f32, f32) {
        let mut rms = f32::default();
        let mut max = f32::default();
        unsafe {
            raw::liquid_filter_isi(self.as_ref().as_ptr() as _, k as _, m as _, rms.to_ptr_mut(), max.to_ptr_mut());
        }
        (rms, max)
    }
    
    fn energy(&self, fc: f32, nfft: usize) -> f32 {
        unsafe {
            raw::liquid_filter_energy(self.as_ref().as_ptr() as _, self.as_ref().len() as _, fc, nfft as _)
        }
    }
}


pub struct Firdes{}
impl Firdes {
    /// esimate required filter length given transition bandwidth and
    /// stop-band attenuation
    ///  df     :   transition bandwidth (0 < _df < 0.5)
    ///  as_    :   stopband suppression level [dB] (_As > 0)
    pub fn estimate_filter_len(df: f32, as_: f32) -> LiquidResult<usize> {
        if df > 0.5 || df <= 0f32 {
            return Err(LiquidError::from(ErrorKind::InvalidValue(
                format!("invalid bandwidth, valid values are (0, 0.5)")
            )))
        } else if as_ <= 0f32 {
            return Err(LiquidError::from(ErrorKind::InvalidValue(
                format!("invalid stopband level, as > 0"))))
        }
        unsafe {
            Ok(raw::estimate_req_filter_len(df, as_) as usize)
        }
    }

    /// estimate filter stop-band attenuation given
    ///  df     :   transition bandwidth (0 < _b < 0.5)
    ///  n      :   filter length
    pub fn estimate_filter_as(df: f32, n: usize) -> LiquidResult<f32> {
        if df > 0.5 || df <= 0f32 {
            return Err(LiquidError::from(ErrorKind::InvalidValue(
                format!("invalid bandwidth, valid values are (0, 0.5)")
            )))
        }
        unsafe {
            Ok(raw::estimate_req_filter_As(df, n as _))
        }

    }

    /// estimate filter transition bandwidth given
    ///  as     :   stop-band attenuation [dB], as > 0
    ///  n      :   filter length
    pub fn estimate_filter_df(as_: f32, n: usize) -> LiquidResult<f32> {
        if as_ <= 0f32 {
            return Err(LiquidError::from(ErrorKind::InvalidValue(
                format!("stop-band attenuation must be greater than 0")
            )))
        }
        unsafe {
            Ok(raw::estimate_req_filter_df(as_, n as _))
        }

    }

    /// Design (root-)Nyquist filter from prototype
    ///  type   : filter type (e.g. LIQUID_FIRFILT_RRRC)
    ///  k      : samples/symbol
    ///  m      : symbol delay
    ///  beta   : excess bandwidth factor, _beta in [0,1]
    ///  dt     : fractional sample delay
    pub fn prototype(type_: FirFilterType, k: usize, m: usize, beta: f32, dt: f32) -> Fir {
        let mut filter = Fir::new(2*k*m + 1);
        unsafe {
            let t: u8 = type_.into();
            raw::liquid_firdes_prototype(t as _, k as _, m as _, beta as _, dt, filter.as_mut().as_mut_ptr());
        }
        filter
    }

    /// Design finite impulse response notch filter
    ///  m      : filter semi-length, m in [1,1000]
    ///  f0     : filter notch frequency (normalized), -0.5 <= _fc <= 0.5
    ///  as_    : stop-band attenuation [dB], _As > 0
    pub fn notch(m: usize, f0: f32, as_: f32) -> LiquidResult<Fir> {
        if m < 1 || m > 1000 {
            return Err(LiquidError::from(ErrorKind::InvalidValue(
                format!("m: {} out of range [1,1000]", m)
            )))
        } else if f0 < -0.5 || f0 > 0.5 {
            return Err(LiquidError::from(ErrorKind::InvalidValue(
                format!("notch frequency {} must be in [-0.5,0.5]", f0)
            )))
        } else if as_ <= 0f32 {
            return Err(LiquidError::from(ErrorKind::InvalidValue(
                format!("as_ stop-band suppression must be greater than zero")
            )))
        }
        let mut filter = Fir::new(2*m + 1);
        unsafe {
            raw::liquid_firdes_notch(m  as _, f0, as_, filter.as_mut().as_mut_ptr());
        }
        Ok(filter)
    }

    /// Design FIR using kaiser window
    ///  n      : filter length, _n > 0
    ///  fc     : cutoff frequency, 0 < _fc < 0.5
    ///  As     : stop-band attenuation [dB], _As > 0
    ///  mu     : fractional sample offset, -0.5 < _mu < 0.5
    pub fn kaiser(n: usize, fc: f32, as_: f32, mu: f32) -> LiquidResult<Fir> {
        if mu < -0.5 || mu > 0.5 {
            return Err(LiquidError::from(ErrorKind::InvalidValue(
                "mu out of range [-0.5,0.5]".to_owned()
            )))
        } else if fc < 0f32 || fc > 0.5 {
            return Err(LiquidError::from(ErrorKind::InvalidValue(
                "cutoff frequency out of range (0, 0.5)".to_owned()
            )))
        } else if n == 0 {
            return Err(LiquidError::from(ErrorKind::InvalidValue(
                "filter length must be greater than zero".to_owned()
            )))
        }
        let mut filter = Fir::new(n);
        unsafe {
            raw::liquid_firdes_kaiser(n as _, fc, as_, mu, filter.as_mut().as_mut_ptr());
        }
        Ok(filter)
    }
    /// Design frequency-shifted root-Nyquist filter based on
    /// the Kaiser-windowed sinc.
    ///
    ///  k      :   filter over-sampling rate (samples/symbol)
    ///  m      :   filter delay (symbols)
    ///  beta   :   filter excess bandwidth factor (0,1)
    ///  dt     :   filter fractional sample delay
    pub fn rkaiser(k: usize, m: usize, beta: f32, dt: f32) -> LiquidResult<Fir> {
        if k < 2 {
            return Err(LiquidError::from(ErrorKind::InvalidValue(
                "k must be at least 2".to_owned()
            )))
        } else if m < 1 {
            return Err(LiquidError::from(ErrorKind::InvalidValue(
                "m must be at least 1".to_owned()
            )))
        } else if beta <= 0.0 || beta >= 1.0 {
            return Err(LiquidError::from(ErrorKind::InvalidValue(
                "beta must be in (0,1)".to_owned()
            )))
        } else if dt < -1.0 || dt > 1.0 {
            return Err(LiquidError::from(ErrorKind::InvalidValue(
                "dt must be in [-1,1]".to_owned()
            )))
        }
        let mut filter = Fir::new((2*k*m+1) as usize);
        unsafe {
            raw::liquid_firdes_rkaiser(k as _, m as _, beta, dt, filter.as_mut().as_mut_ptr());
        }
        Ok(filter)
    }

    /// Design frequency-shifted root-Nyquist filter based on
    /// the Kaiser-windowed sinc using approximation for rho.
    ///
    ///  k      :   filter over-sampling rate (samples/symbol)
    ///  m      :   filter delay (symbols)
    ///  beta   :   filter excess bandwidth factor (0,1)
    ///  dt     :   filter fractional sample delay
    pub fn arkaiser(k: usize, m: usize, beta: f32, dt: f32) -> LiquidResult<Fir> {
        if k < 2 {
            return Err(LiquidError::from(ErrorKind::InvalidValue(
                "k must be at least 2".to_owned()
            )))
        } else if m < 1 {
            return Err(LiquidError::from(ErrorKind::InvalidValue(
                "m must be at least 1".to_owned()
            )))
        } else if beta <= 0.0 || beta >= 1.0 {
            return Err(LiquidError::from(ErrorKind::InvalidValue(
                "beta must be in (0,1)".to_owned()
            )))
        } else if dt < -1.0 || dt > 1.0 {
            return Err(LiquidError::from(ErrorKind::InvalidValue(
                "dt must be in [-1,1]".to_owned()
            )))
        }
        let mut filter = Fir::new((2*k*m+1) as usize);
        unsafe {
            raw::liquid_firdes_arkaiser(k as _, m as _, beta, dt, filter.as_mut().as_mut_ptr());
        }
        Ok(filter)
    }

    /// Design FIR doppler filter
    ///  n      : filter length
    ///  fd     : normalized doppler frequency (0 < fd < 0.5)
    ///  k      : Rice fading factor (k >= 0)
    ///  theta  : LoS component angle of arrival
    pub fn doppler(n: usize, fd: f32, k: f32, theta: f32) -> LiquidResult<Fir> {
        if fd <= 0f32 || fd > 0.5 {
            return Err(LiquidError::from(ErrorKind::InvalidValue(
                "fd must be in (0, 0.5)".to_owned()
            )))
        } else if k < 0f32 {
            return Err(LiquidError::from(ErrorKind::InvalidValue(
                "k must be greater than 0".to_owned()
            )))
        }
        
        // there seem not to be an FirFilterType for this kinf of filter
        // we use the kaiser, because internally it uses  kaiser window
        let mut filter = Fir::new(n);
        unsafe {
            raw::liquid_firdes_doppler(n as _, fd, k, theta, filter.as_mut().as_mut_ptr());
        }

        Ok(filter)
    }
   
    /// Design Nyquist raised-cosine filter
    ///  k      : samples/symbol
    ///  m      : symbol delay
    ///  beta   : rolloff factor (0 < beta <= 1)
    ///  dt     : fractional sample delay
    ///  _h      : output coefficient buffer (length: 2*k*m+1)
    pub fn rcos(k: usize, m: usize, beta: f32, dt: f32) -> LiquidResult<Fir> {
        if k < 1  {
            return Err(LiquidError::from(ErrorKind::InvalidValue(
                "k must be greater than 0".to_owned()
            )))
        } else if m < 1  {
            return Err(LiquidError::from(ErrorKind::InvalidValue(
                "m must be greater than 0".to_owned()
            )))
        } else if beta < 0f32 || beta > 1.0 {
            return Err(LiquidError::from(ErrorKind::InvalidValue(
                "beta must be in [0, 1.0]".to_owned()
            )))
        }
        let mut filter = Fir::new(2*k*m + 1);
        unsafe {
            raw::liquid_firdes_rcos(k as _, m as _ , beta, dt, filter.as_mut().as_mut_ptr());
        }
        Ok(filter)
    }
    
    // Design root-Nyquist raised-cosine filter
    //  k      : samples/symbol
    //  m      : symbol delay
    //  beta   : rolloff factor (0 < beta <= 1)
    //  dt     : fractional sample delay
    pub fn rrcos(k: usize, m: usize, beta: f32, dt: f32) -> LiquidResult<Fir> {
        if k < 1  {
            return Err(LiquidError::from(ErrorKind::InvalidValue(
                "k must be greater than 0".to_owned()
            )))
        } else if m < 1  {
            return Err(LiquidError::from(ErrorKind::InvalidValue(
                "m must be greater than 0".to_owned()
            )))
        } else if beta < 0f32 || beta > 1.0 {
            return Err(LiquidError::from(ErrorKind::InvalidValue(
                "beta must be in [0, 1.0]".to_owned()
            )))
        }
        let mut filter = Fir::new(2*k*m + 1);
        unsafe {
            raw::liquid_firdes_rrcos(k as _, m as _ , beta, dt, filter.as_mut().as_mut_ptr());
        }
        Ok(filter)
    }
    
    pub fn hm3(k: usize, m: usize, beta: f32, dt: f32) -> LiquidResult<Fir> {
        if k < 2  {
            return Err(LiquidError::from(ErrorKind::InvalidValue(
                "k must be greater than 1".to_owned()
            )))
        } else if m < 1  {
            return Err(LiquidError::from(ErrorKind::InvalidValue(
                "m must be greater than 0".to_owned()
            )))
        } else if beta < 0f32 || beta > 1.0 {
            return Err(LiquidError::from(ErrorKind::InvalidValue(
                "beta must be in [0, 1.0]".to_owned()
            )))
        }
        let mut filter = Fir::new(2*k*m + 1);
        unsafe {
            raw::liquid_firdes_hM3(k as _, m as _, beta, dt, filter.as_mut().as_mut_ptr());
        }
        Ok(filter)
    } 
    
    /// Design GMSK transmit filter
    ///  k      : samples/symbol
    ///  m      : symbol delay
    ///  beta   : rolloff factor (0 < beta <= 1)
    ///  dt     : fractional sample delay
    pub fn gmsktx(k: usize, m: usize, beta: f32, dt: f32) -> LiquidResult<Fir> {
        if k < 1  {
            return Err(LiquidError::from(ErrorKind::InvalidValue(
                "k must be greater than 0".to_owned()
            )))
        } else if m < 1  {
            return Err(LiquidError::from(ErrorKind::InvalidValue(
                "m must be greater than 0".to_owned()
            )))
        } else if beta < 0f32 || beta > 1.0 {
            return Err(LiquidError::from(ErrorKind::InvalidValue(
                "beta must be in [0, 1.0]".to_owned()
            )))
        }
        let mut filter = Fir::new(2*k*m + 1);
        unsafe {
            raw::liquid_firdes_gmsktx(k as _, m as _, beta, dt, filter.as_mut().as_mut_ptr());
        }
        Ok(filter)
    } 
    
    /// Design GMSK receive filter
    ///  k      : samples/symbol
    ///  m      : symbol delay
    ///  beta   : rolloff factor (0 < beta <= 1)
    ///  dt     : fractional sample delay
    pub fn gmskrx(k: usize, m: usize, beta: f32, dt: f32) -> LiquidResult<Fir> {
        if k < 1  {
            return Err(LiquidError::from(ErrorKind::InvalidValue(
                "k must be greater than 0".to_owned()
            )))
        } else if m < 1  {
            return Err(LiquidError::from(ErrorKind::InvalidValue(
                "m must be greater than 0".to_owned()
            )))
        } else if beta < 0f32 || beta > 1.0 {
            return Err(LiquidError::from(ErrorKind::InvalidValue(
                "beta must be in [0, 1.0]".to_owned()
            )))
        }
        let mut filter = Fir::new(2*k*m + 1);
        unsafe {
            raw::liquid_firdes_gmskrx(k as _, m as _, beta, dt, filter.as_mut().as_mut_ptr());
        }
        Ok(filter)
    } 
    
    /// Design fexp Nyquist filter
    ///  k      : samples/symbol
    ///  m      : symbol delay
    ///  beta   : rolloff factor (0 < beta <= 1)
    ///  dt     : fractional sample delay
    pub fn fexp(k: usize, m: usize, beta: f32, dt: f32) -> LiquidResult<Fir> {
        if beta < 0f32 || beta > 1.0 {
            return Err(LiquidError::from(ErrorKind::InvalidValue(
                "beta must be in (0, 1.0]".to_owned()
            )))
        }
        let mut filter = Fir::new(2*k*m + 1);
        unsafe {
            raw::liquid_firdes_fexp(k as _, m as _, beta, dt, filter.as_mut().as_mut_ptr());
        }
        Ok(filter)
    } 
    
    /// Design fexp square-root Nyquist filter
    ///  k      : samples/symbol
    ///  m      : symbol delay
    ///  beta   : rolloff factor (0 < beta <= 1)
    ///  dt     : fractional sample delay
    pub fn rfexp(k: usize, m: usize, beta: f32, dt: f32) -> LiquidResult<Fir> {
        if beta < 0f32 || beta > 1.0 {
            return Err(LiquidError::from(ErrorKind::InvalidValue(
                "beta must be in (0, 1.0]".to_owned()
            )))
        }
        let mut filter = Fir::new(2*k*m + 1);
        unsafe {
            raw::liquid_firdes_rfexp(k as _, m as _, beta, dt, filter.as_mut().as_mut_ptr());
        }
        Ok(filter)
    } 
   
    /// Design fsech Nyquist filter
    ///  k      : samples/symbol
    ///  m      : symbol delay
    ///  beta   : rolloff factor (0 < beta <= 1)
    ///  dt     : fractional sample delay
    pub fn fsech(k: usize, m: usize, beta: f32, dt: f32) -> LiquidResult<Fir> {
        if beta < 0f32 || beta > 1.0 {
            return Err(LiquidError::from(ErrorKind::InvalidValue(
                "beta must be in (0, 1.0]".to_owned()
            )))
        }
        let mut filter = Fir::new(2*k*m + 1);
        unsafe {
            raw::liquid_firdes_fsech(k as _, m as _, beta, dt, filter.as_mut().as_mut_ptr());
        }
        Ok(filter)
    } 
    
    /// Design fsech square-root Nyquist filter
    ///  k      : samples/symbol
    ///  m      : symbol delay
    ///  beta   : rolloff factor (0 < beta <= 1)
    ///  dt     : fractional sample delay
    pub fn rfsech(k: usize, m: usize, beta: f32, dt: f32) -> LiquidResult<Fir> {
        if beta < 0f32 || beta > 1.0 {
            return Err(LiquidError::from(ErrorKind::InvalidValue(
                "beta must be in (0, 1.0]".to_owned()
            )))
        }
        let mut filter = Fir::new(2*k*m + 1);
        unsafe {
            raw::liquid_firdes_rfsech(k as _, m as _, beta, dt, filter.as_mut().as_mut_ptr());
        }
        Ok(filter)
    } 
    
    /// Design farcsech Nyquist filter
    ///  k      : samples/symbol
    /// m      : symbol delay
    ///  beta   : rolloff factor (0 < beta <= 1)
    ///  dt     : fractional sample delay
    pub fn farcsech(k: usize, m: usize, beta: f32, dt: f32) -> LiquidResult<Fir> {
        if beta < 0f32 || beta > 1.0 {
            return Err(LiquidError::from(ErrorKind::InvalidValue(
                "beta must be in (0, 1.0]".to_owned()
            )))
        }
        let mut filter = Fir::new(2*k*m + 1);
        unsafe {
            raw::liquid_firdes_farcsech(k as _, m as _, beta, dt, filter.as_mut().as_mut_ptr());
        }
        Ok(filter)
    } 
    
    /// Design farcsech square-root Nyquist filter
    ///  k      : samples/symbol
    ///  m      : symbol delay
    ///  beta   : rolloff factor (0 < beta <= 1)
    ///  dt     : fractional sample delay
    pub fn rfarcsech(k: usize, m: usize, beta: f32, dt: f32) -> LiquidResult<Fir> {
        if beta < 0f32 || beta > 1.0 {
            return Err(LiquidError::from(ErrorKind::InvalidValue(
                "beta must be in (0, 1.0]".to_owned()
            )))
        }
        let mut filter = Fir::new(2*k*m + 1);
        unsafe {
            raw::liquid_firdes_rfarcsech(k as _, m as _, beta, dt, filter.as_mut().as_mut_ptr());
        }
        Ok(filter)
    } 
}


#[cfg(test)]
mod tests {
    use super::{Firdes};
    use crate::filter::FilterAnalysis;

    #[test]
    fn test_firdes_filter_autocorr() {
        let f1 = Firdes::fexp(10, 2, 0.2, 0.5).unwrap();
        assert_eq!(f1.auto_corr(5), 6.012687);
    }
    
    #[test]
    fn test_filter_crosscorr() {
        let f1 = Firdes::fexp(10, 2, 0.2, 0.5).unwrap();
        let f2 = Firdes::fexp(5, 1, 0.3, 0.1).unwrap();
        assert_eq!(f1.cross_corr(&f2, 5), 0.14224437);
    }
    
    #[test]
    fn test_filter_group_delay() {
        let f1 = Firdes::rfarcsech(5, 20, 0.8, 0.5).unwrap();
        let delay = f1.group_delay(-0.2).unwrap();
        assert_eq!(delay, 100.00711);
    }

}
