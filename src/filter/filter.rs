use crate::liquid_dsp_sys as raw;
use crate::utils::ToCPointerMut;


pub trait FilterAnalysis {

    /// Compute auto-correlation of filter at a specific lag.
    ///
    ///  lag    :   auto-correlation lag (samples)
    fn auto_corr(&self, lag: usize) -> f32;
    
    /// Compute cross-correlation of two filters at a specific lag.
    ///
    ///  filter      :   filter coefficients
    ///  lag    :   cross-correlation lag (samples)
    /// # returns
    /// the cross correlation between both filters 
    fn cross_corr(&self, filter: &Self, lag: usize) -> f32;
    
    /// Compute inter-symbol interference (ISI)--both RMS and
    /// maximum--for the filter _h.
    ///
    ///  k      :   filter over-sampling rate (samples/symbol)
    ///  m      :   filter delay (symbols)
    /// # returns
    ///  rms    :   output root mean-squared ISI
    ///  max    :   maximum ISI
    fn isi(&self, k: usize, m: usize,) ->  (f32, f32);

    /// Compute relative out-of-band energy
    ///
    ///  fc     :   analysis cut-off frequency
    ///  nfft   :   fft size
    fn energy(&self, fc: f32, nfft: usize) -> f32;

}

impl<T> FilterAnalysis for T where T: AsRef<[f32]> {

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
