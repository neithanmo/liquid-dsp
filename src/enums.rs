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

impl From<FecScheme> for u32 {
    fn from(value: FecScheme) -> u32 {
        unsafe { transmute::<FecScheme, u8>(value) as u32 }
    }
}

