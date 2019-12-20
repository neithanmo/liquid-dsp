use std::marker::PhantomData;
use num::complex::Complex32;

use crate::liquid_dsp_sys as raw;
use crate::utils::{ToCPointerMut, ToCValue};
use filter::transfer::Transfer;
pub enum Discrete {}
pub enum Analog {}

// define speciic filter types 
pub enum Butter {}
pub enum Cheby1 {}
pub enum Cheby2 {}
pub enum Bessel {}
pub enum Ellip {}


#[derive(Debug, Default)]
pub struct Zpk<T, D> {
    k: Complex32,
    p: Vec<Complex32>,
    z: Vec<Complex32>,
    data: PhantomData<T>,
    ftype: PhantomData<D>,
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

impl<T, D> Zpk<T, D> {
    fn new(nz: usize, np:usize, k: Complex32) -> Self {
        Self {
            k,
            z: vec![Complex32::default(); nz],
            p: vec![Complex32::default(); np],
            data: PhantomData,
            ftype: PhantomData,
        }
    }
}

impl<D> Zpk<Analog, D> {
    fn new_analog(nz: usize, np: usize, k: Complex32) -> Self {
        Self {
            k,
            z: vec![Complex32::default(); nz],
            p: vec![Complex32::default(); np],
            data: PhantomData,
            ftype: PhantomData,
        }
    }

    pub fn bilinear_zpkf(self, m: f32) -> Zpk<Discrete, D> {
        let mut new = Zpk::new(self.z.len(), self.p.len(), Complex32::default());
        let mut kd = Complex32::default();
        unsafe {
            raw::bilinear_zpkf(self.z.as_mut_slice().to_ptr_mut(), self.z.len() as _, self.p.as_mut_slice().to_ptr_mut(), 
                self.p.len() as _, self.k.to_c_value(), 
                m, new.z.as_mut_slice().to_ptr_mut(), new.p.as_mut_slice().to_ptr_mut(), new.k.to_ptr_mut() as _);
        }

        new
    }

}

impl Zpk<Analog, Butter> {

    pub fn butterf(n: usize) -> Zpk<Analog, Butter> {
        let mut f = Self::new(0, n, Complex32::default());
        unsafe {
            raw::butter_azpkf(n as _, f.z.as_mut_slice().to_ptr_mut(), f.p.as_mut_slice().to_ptr_mut(), f.k.to_ptr_mut());
        } 
        f
    }
}

impl Zpk<Analog, Cheby1> {

    pub fn cheby1(n: usize, ep: f32) -> Zpk<Analog, Cheby1> {
        let mut f = Self::new(0, n, Complex32::default());
        unsafe {
            raw::cheby1_azpkf(n as _, ep , f.z.as_mut_slice().to_ptr_mut(), f.p.as_mut_slice().to_ptr_mut(), f.k.to_ptr_mut());
        } 
        f
    }
}

impl Zpk<Analog, Cheby2> {

    pub fn cheby2(n: usize, es: f32) -> Zpk<Analog, Cheby2> {
        let mut f = Self::new(0, n, Complex32::default());
        unsafe {
            raw::cheby2_azpkf(n as _, es , f.z.as_mut_slice().to_ptr_mut(), f.p.as_mut_slice().to_ptr_mut(), f.k.to_ptr_mut());
        } 
        f
    }
}

impl Zpk<Analog, Ellip> {

    pub fn ellip(n: usize, ep: f32, es: f32) -> Zpk<Analog, Ellip> {
        let mut f = Self::new(0, n, Complex32::default());
        unsafe {
            raw::ellip_azpkf(n as _, ep, es , f.z.as_mut_slice().to_ptr_mut(), f.p.as_mut_slice().to_ptr_mut(), f.k.to_ptr_mut());
        } 
        f
    }
}

impl Zpk<Analog, Bessel> {

    pub fn bessel(n: usize) -> Zpk<Analog, Bessel> {
        let mut f = Self::new(0, n, Complex32::default());
        unsafe {
            raw::bessel_azpkf(n as _,f.z.as_mut_slice().to_ptr_mut(), f.p.as_mut_slice().to_ptr_mut(), f.k.to_ptr_mut());
        } 
        f
    }
}

/* impl<T> Zpk<Discrete, T> {
    to_tff(self) -> Transfer {
        let mut transfer = Transfer {
            a: vec![0f32; ]
        }       
    }
} */