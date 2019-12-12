use libc::{c_int, c_void};
use std::marker::PhantomData;
use std::mem::transmute;

use crate::liquid_dsp_sys as raw;

use crate::callbacks::Callbacks;
use crate::enums::{FirdespmBtype, FirdespmWtype};
use crate::errors::{ErrorKind, LiquidError};
use crate::utils::catch;

pub extern "C" fn firdespm_callback_f<'a>(
    frecuency: f64,
    userdata: *mut c_void,
    desired: *mut f64,
    weight: *mut f64,
) -> c_int {
    catch(|| unsafe {
        if let Some(fun) = (*(userdata as *mut Callbacks)).firdespm_callback.as_mut() {
            return fun(frecuency, &mut (*desired), &mut (*weight)) as c_int;
        }
        0
    })
    .unwrap()
}

///
/// fir (finite impulse response) filter design using Parks-McClellan
/// algorithm
///
/// Much of this program has been borrowed heavily from [McClellan:1973]
/// and [Janovetz:1998] with the exception of the Lagrange polynomial
/// interpolation formulas and the structured 'firdespm' object. Several
/// improvements have been made in the search algorithm to help maintain
/// stability and convergence.
///
/// References:
///  [Parks:1972] T. W. Parks and J. H. McClellan, "Chebyshev
///      Approximation for Nonrecursive Digital Filters with Linear
///      Phase," IEEE Transactions on Circuit Theory, vol. CT-19,
///      no. 2, March 1972.
///  [McClellan:1973] J. H. McClellan, T. W. Parks, L. R. Rabiner, "A
///      Computer Program for Designing Optimum FIR Linear Phase
///      Digital Filters," IEEE Transactions on Audio and
///      Electroacoustics, vol. AU-21, No. 6, December 1973.
///  [Rabiner:1975] L. R. Rabiner, J. H. McClellan, T. W. Parks, "FIR
///      Digital filter Design Techniques Using Weighted Chebyshev
///      Approximations," Proceedings of the IEEE, March 1975.
///  [Parks:1987] T. W. Parks and C. S. Burrus, "Digital Filter
///      Design," Upper Saddle River, NJ, John Wiley & Sons, Inc., 1987
///      (Section 3.3.3)
///  [Janovetz:1998] J. Janovetz, online: http://www.janovetz.com/jake/
pub struct Firdespm<'a> {
    inner: raw::firdespm,
    h_len: usize,
    callback: *mut Callbacks<'a>,
    phantom: PhantomData<&'a ()>,
}

impl<'a> Firdespm<'a> {

    fn validate_inputs_length(
        num_bands: usize,
        bands: &[f32],
        des: &[f32],
        weights: &[f32],
        wtype: &Option<&[FirdespmWtype]>
    ) -> Result<(), LiquidError> {
            
        if bands.len() == 0 || bands.len()/2 != num_bands {
            return Err(LiquidError::from(ErrorKind::InvalidLength{description: format!(
                "bands length: {} valid length: {}", bands.len(), num_bands * 2
            )}));
        }
        
        let invalid = if let Some(w) = wtype {
           num_bands != des.len() || num_bands != w.len() || num_bands != weights.len() || num_bands != w.len()
        } else {
            num_bands != des.len() || num_bands != weights.len()
        };
        
        if invalid {
            let description = format!(
                "des: {}, weights: {} == {}",
                des.len(),
                weights.len(),
                num_bands
            );
            return Err(LiquidError::from(ErrorKind::InvalidLength{description}));
        }

        Ok(())
    }

    /// create firdespm object
    ///  _h_len      :   length of filter (number of taps)
    ///  _bands      :   band edges, f in [0,0.5], [size: _num_bands x 2]
    ///  _des        :   desired response [size: _num_bands x 1]
    ///  _weights    :   response weighting [size: _num_bands x 1]
    ///  _wtype      :   weight types (e.g. LIQUID_FIRDESPM_FLATWEIGHT) [size: _num_bands x 1]
    ///  _btype      :   band type (e.g. LIQUID_FIRDESPM_BANDPASS)
    pub fn create(
        h_len: usize,
        num_bands: usize,
        bands: &[f32],
        des: &[f32],
        weights: &[f32],
        wtype: Option<&[FirdespmWtype]>,
        btype: FirdespmBtype,
    ) -> Result<Self, LiquidError> {
        Self::validate_inputs_length(num_bands, bands, des, weights, &wtype)?;
        
        let ptr = if let Some(w) = wtype {
            w.as_ptr()
        } else {
            std::ptr::null_mut()
        };

        unsafe {
            Ok(Self {
                inner: raw::firdespm_create(
                    h_len as _,
                    num_bands as _,
                    bands.as_ptr() as _,
                    des.as_ptr() as _,
                    weights.as_ptr() as _,
                    transmute::<*mut FirdespmWtype, *mut u32>(ptr as _),
                    u8::from(btype) as _,
                ),
                h_len,
                callback: std::ptr::null_mut() as _,
                phantom: PhantomData,
            })
        }
    }

    pub fn create_callback<F>(
        h_len: usize,
        num_bands: usize,
        bands: &[f32],
        btype: FirdespmBtype,
        callback: F,
    ) -> Result<Self, LiquidError>
    where
        F: FnMut(f64, &mut f64, &mut f64) -> i8 + 'a,
    {
        if num_bands == 0 || num_bands != bands.len() / 2 {
            return Err(LiquidError::from(ErrorKind::InvalidLength{description:format!(
                "bands length: {} valid length: {}", bands.len(), num_bands * 2
            )}));
        }
        let mut userdata = Callbacks::default();
        userdata.firdespm_callback = Some(Box::new(callback));
        let userdata = Box::into_raw(Box::new(userdata));
        unsafe {
            Ok(Self {
                inner: raw::firdespm_create_callback(
                    h_len as _,
                    num_bands as _,
                    bands.as_ptr() as _,
                    u8::from(btype) as _,
                    Some(firdespm_callback_f),
                    userdata as _,
                ),
                h_len,
                callback: userdata,
                phantom: PhantomData,
            })
        }
    }

    pub fn print(&self) {
        unsafe {
            raw::firdespm_print(self.inner);
        }
    }

    pub fn execute(&self, h: &mut [f32]) {
        assert!(h.len() == self.h_len, "h array len must be = h_len");
        unsafe {
            raw::firdespm_execute(self.inner, h.as_mut_ptr());
        }
    }

    /// run filter design (full life cycle of object)
    ///  num_bands  :   number of frequency bands
    ///  bands      :   band edges, f in [0,0.5], [size: _num_bands x 2]
    ///  des        :   desired response [size: _num_bands x 1]
    ///  weights    :   response weighting [size: _num_bands x 1]
    ///  wtype      :   weight types (e.g. LIQUID_FIRDESPM_FLATWEIGHT) [size: _num_bands x 1]
    ///  btype      :   band type (e.g. LIQUID_FIRDESPM_BANDPASS)
    ///  output      :   output coefficients array [size: _h_len x 1]
    pub fn run(
        num_bands: usize,
        bands: &[f32],
        des: &[f32],
        weights: &[f32],
        wtype: Option<&[FirdespmWtype]>,
        btype: FirdespmBtype,
        output: &mut[f32]
    ) -> Result<(), LiquidError> {
        Self::validate_inputs_length(num_bands, bands, des, weights, &wtype)?;
        
        let ptr = if let Some(w) = wtype {
            w.as_ptr()
        } else {
            std::ptr::null_mut()
        };
        
        unsafe {
            raw::firdespm_run(
                    output.len() as _,
                    num_bands as _,
                    bands.as_ptr() as _,
                    des.as_ptr() as _,
                    weights.as_ptr() as _,
                    transmute::<*mut FirdespmWtype, *mut u32>(ptr as _),
                    u8::from(btype) as _,
                    output.as_mut_ptr()
            );
        }
        Ok(())
    }

    pub fn lowpass(
        fc: f32,
        as_: f32,
        mu: f32,
        output: &mut[f32]
    ) -> Result<(), LiquidError> 
    {
        assert!(output.len() > 0, "filter length must be greater than zero");
        if mu < -0.5 || mu > 0.5 {
            return Err(LiquidError::from(ErrorKind::
                InvalidValue("mu out of range [-0.5,0.5]".to_owned())));
        } else if fc <0f32 || fc > 0.5 {
           return Err(LiquidError::from(ErrorKind::
                InvalidValue("cutoff frequency out of range (0, 0.5)".to_owned()))); 
        }
        unsafe {
            raw::firdespm_lowpass(output.len() as _, fc, as_, mu, output.as_mut_ptr());
        }
        Ok(())
    } 
}

impl<'a> Drop for Firdespm<'a> {
    fn drop(&mut self) {
        unsafe {
            raw::firdespm_destroy(self.inner);
            let _ = Box::from_raw(self.callback);
        }
    }
}
