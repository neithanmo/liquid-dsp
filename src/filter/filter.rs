pub trait FilterAnalysis
where
    Self: AsRef<[f32]>,
{
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
    fn isi(&self, k: usize, m: usize) -> (f32, f32);

    /// Compute relative out-of-band energy
    ///
    ///  fc     :   analysis cut-off frequency
    ///  nfft   :   fft size
    fn energy(&self, fc: f32, nfft: usize) -> f32;
}
