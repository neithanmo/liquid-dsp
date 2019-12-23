use num::complex::Complex32;
use std::marker::PhantomData;

use crate::liquid_dsp_sys as raw;
use crate::utils::{ToCPointerMut, ToCValue};
use filter::transfer::Transfer;

// filter type
pub enum Discrete {}
pub enum Analog {}

// define speciic filter types
pub enum Butter {}
pub enum Cheby1 {}
pub enum Cheby2 {}
pub enum Bessel {}
pub enum Ellip {}

// B types
pub enum LowPass {}
pub enum HighPass {}
pub enum BandPass {}
pub enum StopB {}

#[derive(Debug, Default)]
pub struct Zpk<B, T, R> {
    pub k: Complex32,
    pub p: Vec<Complex32>,
    pub z: Vec<Complex32>,
    n: usize,
    data: PhantomData<T>,
    ftype: PhantomData<R>,
    band: PhantomData<B>,
}

/* impl Zpk<Discrete> {
    pub fn new_discrete(nz: usize, np: usize, k: Complex32) -> Self {
        Self {
            k,
            z: vec![Complex32::default(); nz],
            p: vec![Complex32::default(); np],
            data: PhantomData,
        }
    }
} */

impl<B, T, R> Zpk<B, T, R> {
    fn new(n: usize, k: Complex32) -> Self {
        Self {
            k,
            z: vec![Complex32::default(); n],
            p: vec![Complex32::default(); n],
            n,
            data: PhantomData,
            ftype: PhantomData,
            band: PhantomData,
        }
    }

    pub fn len(&self) -> usize {
        self.n
    }
}

impl<B, R> Zpk<B, Analog, R> {
    /*     fn new_analog(n: usize, k: Complex32) -> Self {
        Self {
            k,
            z: vec![Complex32::default(); n],
            p: vec![Complex32::default(); n],
            n,
            data: PhantomData,
            ftype: PhantomData,
            band: PhantomData,
        }
    } */

    /// convert analog zeros, poles, gain to digital zeros, poles gain
    ///  m      :   frequency pre-warping factor
    ///
    /// The filter order is characterized by the number of analog
    /// poles.  The analog filter may have up to _npa zeros.
    /// The number of digital zeros and poles is equal to _npa.
    pub fn bilinear_zpkf(mut self, m: f32) -> Zpk<B, Discrete, R> {
        let mut new = Zpk::new(self.len(), Complex32::default());
        unsafe {
            raw::bilinear_zpkf(
                self.z.as_mut_slice().to_ptr_mut(),
                self.len() as _,
                self.p.as_mut_slice().to_ptr_mut(),
                self.p.len() as _,
                self.k.to_c_value(),
                m,
                new.z.as_mut_slice().to_ptr_mut(),
                new.p.as_mut_slice().to_ptr_mut(),
                new.k.to_ptr_mut() as _,
            );
        }
        new
    }
}

impl Zpk<LowPass, Analog, Butter> {
    /// Compute analog zeros, poles, gain of low-pass Butterworth
    /// filter, grouping complex conjugates together. If filter
    /// order is odd, the single real pole (-1) is at the end of
    /// the array.  There are no zeros for the analog Butterworth
    /// filter.  The gain is unity.
    ///  _n      :   filter order
    pub fn butterf(n: usize) -> Zpk<LowPass, Analog, Butter> {
        let mut f = Self::new(n, Complex32::default());
        unsafe {
            raw::butter_azpkf(
                n as _,
                f.z.as_mut_slice().to_ptr_mut(),
                f.p.as_mut_slice().to_ptr_mut(),
                f.k.to_ptr_mut(),
            );
        }
        f
    }
}

impl Zpk<LowPass, Analog, Cheby1> {
    /// Compute analog zeros, poles, gain of low-pass Chebyshev
    /// Type I filter, grouping complex conjugates together. If
    /// the filter order is odd, the single real pole is at the
    /// end of the array.  There are no zeros for the analog
    /// Chebyshev Type I filter.
    ///  n      :   filter order
    ///  ep     :   epsilon, related to pass-band ripple
    pub fn cheby1(n: usize, ep: f32) -> Zpk<LowPass, Analog, Cheby1> {
        let mut f = Self::new(n, Complex32::default());
        unsafe {
            raw::cheby1_azpkf(
                n as _,
                ep,
                f.z.as_mut_slice().to_ptr_mut(),
                f.p.as_mut_slice().to_ptr_mut(),
                f.k.to_ptr_mut(),
            );
        }
        f
    }
}

impl Zpk<LowPass, Analog, Cheby2> {
    /// Compute analog zeros, poles, gain of low-pass Chebyshev
    /// Type II filter, grouping complex conjugates together. If
    /// the filter order is odd, the single real pole is at the
    /// end of the array.
    ///  n      :   filter order
    ///  ep     :   epsilon, related to stop-band ripple
    pub fn cheby2(n: usize, es: f32) -> Zpk<LowPass, Analog, Cheby2> {
        let mut f = Self::new(n, Complex32::default());
        unsafe {
            raw::cheby2_azpkf(
                n as _,
                es,
                f.z.as_mut_slice().to_ptr_mut(),
                f.p.as_mut_slice().to_ptr_mut(),
                f.k.to_ptr_mut(),
            );
        }
        f
    }
}

impl Zpk<LowPass, Analog, Ellip> {
    /// ellip_azpkf()
    ///
    /// Compute analog zeros, poles, gain of low-pass elliptic
    /// filter, grouping complex conjugates together. If
    /// the filter order is odd, the single real pole is at the
    /// end of the array.
    ///  n      :   filter order
    ///  ep     :   epsilon_p, related to pass-band ripple
    ///  es     :   epsilon_s, related to stop-band ripple
    pub fn ellip(n: usize, ep: f32, es: f32) -> Zpk<LowPass, Analog, Ellip> {
        let mut f = Self::new(n, Complex32::default());
        unsafe {
            raw::ellip_azpkf(
                n as _,
                ep,
                es,
                f.z.as_mut_slice().to_ptr_mut(),
                f.p.as_mut_slice().to_ptr_mut(),
                f.k.to_ptr_mut(),
            );
        }
        f
    }
}

impl Zpk<LowPass, Analog, Bessel> {
    /// Compute analog zeros, poles, gain of low-pass Bessel
    /// filter, grouping complex conjugates together. If
    /// the filter order is odd, the single real pole is at
    /// the end of the array.  There are no zeros for the
    /// analog Bessel filter.  The gain is unity.
    ///  n      :   filter order
    pub fn bessel(n: usize) -> Zpk<LowPass, Analog, Bessel> {
        let mut f = Self::new(n, Complex32::default());
        unsafe {
            raw::bessel_azpkf(
                n as _,
                f.z.as_mut_slice().to_ptr_mut(),
                f.p.as_mut_slice().to_ptr_mut(),
                f.k.to_ptr_mut(),
            );
        }
        f
    }
}

impl<B, R> Zpk<B, Discrete, R> {
    /// convert discrete Zpk form to transfer function form
    pub fn to_tff(mut self) -> Transfer {
        let mut transfer = Transfer {
            a: vec![0f32; self.len() + 1],
            b: vec![0f32; self.len() + 1],
        };
        unsafe {
            raw::iirdes_dzpk2tff(
                self.z.as_mut_slice().to_ptr_mut(),
                self.p.as_mut_slice().to_ptr_mut(),
                self.len() as _,
                self.k.to_c_value(),
                transfer.b.as_mut_ptr(),
                transfer.a.as_mut_ptr(),
            );
        }
        transfer
    }

    pub fn to_sosf(mut self) -> Transfer {
        let r = self.len() % 2;
        let l = (self.len() - r) / 2;
        let len = (l + r) * 3;
        let mut transfer = Transfer {
            a: vec![0f32; len],
            b: vec![0f32; len],
        };
        unsafe {
            raw::iirdes_dzpk2sosf(
                self.z.as_mut_slice().to_ptr_mut(),
                self.p.as_mut_slice().to_ptr_mut(),
                self.len() as _,
                self.k.to_c_value(),
                transfer.b.as_mut_ptr(),
                transfer.a.as_mut_ptr(),
            );
        }
        transfer
    }
}

impl<R> Zpk<LowPass, Discrete, R> {
    /// digital z/p/k low-pass to high-pass transformation
    pub fn lp_to_hp(mut self) -> Zpk<HighPass, Discrete, R> {
        let mut hp = Zpk::new(self.len(), self.k);
        unsafe {
            raw::iirdes_dzpk_lp2hp(
                self.z.as_mut_slice().to_ptr_mut(),
                self.p.as_mut_slice().to_ptr_mut(),
                self.len() as _,
                hp.z.as_mut_slice().to_ptr_mut(),
                hp.p.as_mut_slice().to_ptr_mut(),
            );
        }
        hp
    }

    /// digital z/p/k low-pass to band-pass transformation
    ///  f0     :   center frequency
    pub fn lp_to_bp(mut self, f0: f32) -> Zpk<BandPass, Discrete, R> {
        let mut hp = Zpk::new(self.len(), self.k);
        unsafe {
            raw::iirdes_dzpk_lp2bp(
                self.z.as_mut_slice().to_ptr_mut(),
                self.p.as_mut_slice().to_ptr_mut(),
                self.len() as _,
                f0,
                hp.z.as_mut_slice().to_ptr_mut(),
                hp.p.as_mut_slice().to_ptr_mut(),
            );
        }
        hp
    }
}
