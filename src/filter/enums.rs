use std::mem::transmute;

#[derive(Clone, Copy, Eq, PartialEq, Debug)]
pub enum FirdespmBtype {
    BANDPASS,
    DIFFERENTIATOR,
    HILBERT,
}

impl From<FirdespmBtype> for u8 {
    fn from(value: FirdespmBtype) -> u8 {
        unsafe { transmute::<FirdespmBtype, u8>(value) }
    }
}

impl From<u8> for FirdespmBtype {
    fn from(value: u8) -> Self {
        if value > 3 {
            unimplemented!();
        }
        unsafe { transmute::<u8, FirdespmBtype>(value) }
    }
}

#[derive(Clone, Copy, Eq, PartialEq, Debug)]
pub enum FirdespmWtype {
    FLATWEIGHT,
    EXPWEIGHT,
    LINWEIGHT,
}

impl From<FirdespmWtype> for u8 {
    fn from(value: FirdespmWtype) -> u8 {
        unsafe { transmute::<FirdespmWtype, u8>(value) }
    }
}

impl From<u8> for FirdespmWtype {
    fn from(value: u8) -> Self {
        if value > 3 {
            unimplemented!();
        }
        unsafe { transmute::<u8, FirdespmWtype>(value) }
    }
}

#[derive(Clone, Copy, Eq, PartialEq, Debug)]
pub enum FirdesFilterType {
    UNKNOWN,
    KAISER,
    PM,
    RCOS,
    FEXP,
    FSECH,
    FARCSECH,
    ARKAISER,
    RKAISER,
    RRC,
    HM3,
    GMSKTX,
    GMSKRX,
    RFEXP,
    RFSECH,
    RFARCSECH15,
}

impl From<FirdesFilterType> for u8 {
    fn from(value: FirdesFilterType) -> u8 {
        unsafe { transmute::<FirdesFilterType, u8>(value) }
    }
}

impl From<u8> for FirdesFilterType {
    fn from(value: u8) -> Self {
        if value > 15 {
            return Self::UNKNOWN;
        }
        unsafe { transmute::<u8, FirdesFilterType>(value) }
    }
}

#[derive(Clone, Copy, Eq, PartialEq, Debug)]
pub enum IirdesFilterType {
    BUTTER,
    CHEBY1,
    CHEBY2,
    ELLIP,
    BESSEL,
}

impl From<IirdesFilterType> for u8 {
    fn from(value: IirdesFilterType) -> u8 {
        unsafe { transmute::<IirdesFilterType, u8>(value) }
    }
}

/* impl From<u8> for IirdesFilterType {
    fn from(value: u8) -> Self {
        if value > 15 {
            return Self::UNKNOWN
        }
        unsafe { transmute::<u8, IirdesFilterType>(value) }
    }
} */

#[derive(Clone, Copy, Eq, PartialEq, Debug)]
pub enum IirdesBandType {
    LOWPASS,
    HIGHPASS,
    BANDPASS,
    BANDSTOP,
}

impl From<IirdesBandType> for u8 {
    fn from(value: IirdesBandType) -> u8 {
        unsafe { transmute::<IirdesBandType, u8>(value) }
    }
}

#[derive(Clone, Copy, Eq, PartialEq, Debug)]
pub enum IirdesFormat {
    SOS,
    TF,
}

impl From<IirdesFormat> for u8 {
    fn from(value: IirdesFormat) -> u8 {
        unsafe { transmute::<IirdesFormat, u8>(value) }
    }
}
