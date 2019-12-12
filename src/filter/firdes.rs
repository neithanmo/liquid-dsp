use crate::enums::FirFilterType;
use crate::liquid_dsp_sys as raw;
use crate::errors::{LiquidError, ErrorKind};

pub struct FiR{
    h: Vec<f32>,
}

impl FiR {
    pub fn new(len: usize) -> Self {
        Self {
            h:  vec![0f32; len],
        }
    }

    pub fn len(&self) -> usize {
        self.h.len()
    }

    pub fn as_ref(&self) -> &[f32] {
        self.h.as_slice()
    }

    pub fn as_ref_mut(&mut self) -> &mut[f32] {
        self.h.as_mut_slice()
    }
}

pub struct FiRDes{}
impl FiRDes {
    /// esimate required filter length given transition bandwidth and
    /// stop-band attenuation
    ///  df     :   transition bandwidth (0 < _df < 0.5)
    ///  as_    :   stopband suppression level [dB] (_As > 0)
    pub fn estimate_filter_len(df: f32, as_: f32) -> Result<usize, LiquidError> {
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
    pub fn estimate_filter_As(df: f32, n: usize) -> Result<f32, LiquidError> {
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
    pub fn estimate_filter_df(as_: f32, n: usize) -> Result<f32, LiquidError> {
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
    ///  h      : output coefficient buffer (length: 2*k*m+1)
    pub fn prototype(type_: FirFilterType, k: u32, m: u32, beta: f32, dt: f32) -> FiR {
        let mut filter = FiR::new((2*k*m + 1) as usize);
        unsafe {
            let t: u8 = type_.into();
            raw::liquid_firdes_prototype(t as _, k as _, m as _, beta as _, dt, filter.as_ref_mut().as_mut_ptr());
        }
        filter
    }

    /// Design finite impulse response notch filter
    ///  m      : filter semi-length, m in [1,1000]
    ///  f0     : filter notch frequency (normalized), -0.5 <= _fc <= 0.5
    ///  as_    : stop-band attenuation [dB], _As > 0
    ///  h      : output coefficient buffer, [size: 2*_m+1 x 1] 
    pub fn notch(m: u32, f0: f32, as_: f32, h: &mut[f32]) -> Result<FiR, LiquidError> {
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
        let mut filter = FiR::new((2*m + 1) as usize);
        unsafe {
            raw::liquid_firdes_notch(m  as _, f0, as_, filter.as_ref_mut().as_mut_ptr());
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
    pub fn kaiser(k: u32, m: u32, beta: f32, dt: f32) -> Result<FiR, LiquidError> {
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
        let mut filter = FiR::new((2*k*m+1) as usize);
        unsafe {
            raw::liquid_firdes_rkaiser(k as _, m as _, beta, dt, filter.as_ref_mut().as_mut_ptr());
        }
        Ok(filter)
    }


    /// Design FIR doppler filter
    ///  n      : filter length
    ///  fd     : normalized doppler frequency (0 < fd < 0.5)
    ///  k      : Rice fading factor (k >= 0)
    ///  theta  : LoS component angle of arrival
    pub fn doppler(n: usize, fd: f32, k: f32, theta: f32) -> Result<FiR, LiquidError> {
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
        let mut filter = FiR::new(n);
        unsafe {
            raw::liquid_firdes_doppler(n as _, fd, k, theta, filter.as_ref_mut().as_mut_ptr());
        }

        Ok(filter)
    }
}