
#![allow(non_camel_case_types, non_snake_case)]
use std::fmt;
use std::mem::transmute;

bitflags! {

    pub struct AgcSquelchMode: u8 {

        const UNKNOWN =   0;
        const ENABLED =   1;
        const RISE  =     2;
        const SIGNALHI =  3;
        const FALL =      4;
        const SIGNALLO =  5;
        const TIMEOUT =   6;
        const DISABLED =  7;
    }
}

#[derive(Clone, Copy, Eq, PartialEq)]
pub enum AmpModemType {
    DSB,
    USB,
    LSB,
}

impl From<AmpModemType> for u32 {
    fn from(value: AmpModemType) -> u32 {
        match value {
            AmpModemType::DSB => 0,
            AmpModemType::USB => 1,
            AmpModemType::LSB => 2,
        }
    }
}

impl fmt::Debug for AmpModemType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let type_ = match self {
            AmpModemType::DSB => "double side-band",
            AmpModemType::USB => "single side-band (upper)",
            AmpModemType::LSB => "single side-band (lower)",
        };
        write!(f,"{}",type_)
    }
}

#[derive(Clone, Copy, Eq, PartialEq)]
pub enum FecScheme {
    UNKNOWN,
    NONE,
    REP3,
    REP5,
    HAMMING74,
    HAMMING84,
    HAMMING128,
    GOLAY2412,
    SECDED2216,
    SECDED3932,
    SECDED7264,
    CONV_V27,
    CONV_V29,
    CONV_V39,
    CONV_V615,
    CONV_V27P23,
    CONV_V27P34,
    CONV_V27P45,
    CONV_V27P56,
    CONV_V27P67,
    CONV_V27P78,
    CONV_V29P23,
    CONV_V29P34,
    CONV_V29P45,
    CONV_V29P56,
    CONV_V29P67,
    CONV_V29P78,
    RS_M8,
}

impl From<FecScheme> for u8 {
    fn from(value: FecScheme) -> u8 {
        unsafe { transmute::<FecScheme, u8>(value)}
    }
}

impl From<u8> for FecScheme {
    fn from(value: u8) -> Self {
        if value > 27 {
            return FecScheme::UNKNOWN;
        }
        unsafe { transmute::<u8, FecScheme>(value) }
    }
}

#[derive(Clone, Copy, Eq, PartialEq, Debug)]
pub enum CrcScheme {
    CRC_UNKNOWN,
    CRC_NONE,
    CRC_CHECKSUM,
    CRC_8,
    CRC_16,
    CRC_24,
    CRC_32,
}

impl From<CrcScheme> for u8 {
    fn from(value: CrcScheme) -> u8 {
        unsafe { transmute::<CrcScheme, u8>(value)}
    }
}

impl From<u8> for CrcScheme {
    fn from(value: u8) -> Self {
        if value > 6 {
            return CrcScheme::CRC_UNKNOWN;
        }
        unsafe { transmute::<u8, CrcScheme>(value) }
    }
}

#[derive(Clone, Copy, Eq, PartialEq, Debug)]
pub enum FftType {
    BACKWARD = -1,
    UNKNOWN = 0,
    FORWARD = 1,
    REDFT00 = 10,
    REDFT10 = 11,
    REDFT01 = 12,
    REDFT11 = 13,
    RODFT00 = 20,
    RODFT10 = 21,
    RODFT01 = 22,
    RODFT11 = 23,
    MDCT = 30,
    IMDCT = 31,
}

impl From<FftType> for i8 {
    fn from(value: FftType) -> i8 {
        unsafe { transmute::<FftType, i8>(value)}
    }
}

impl From<i8> for FftType {
    fn from(value: i8) -> Self {
        match value {
           -1 | 0 | 1 => {},
           10..=13 => {},
           20..=23 => {},
           30 | 31 => {},
           - => return FftType::UNKNOWN, 
        }
        unsafe { transmute::<i8, Self>(value) }
    }
}