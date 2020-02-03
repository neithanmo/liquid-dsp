use std::fmt;
use std::mem::transmute;

#[derive(Clone, Copy, Eq, PartialEq)]
pub enum AmpModemType {
    Dsb,
    Usb,
    Lsb,
}

impl From<AmpModemType> for u8 {
    fn from(value: AmpModemType) -> u8 {
        unsafe { transmute::<AmpModemType, u8>(value) }
    }
}

impl From<u8> for AmpModemType {
    fn from(value: u8) -> Self {
        if value > 2 {
            unimplemented!();
        }
        unsafe { transmute::<u8, AmpModemType>(value) }
    }
}
impl fmt::Debug for AmpModemType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let type_ = match self {
            AmpModemType::Dsb => "double side-band",
            AmpModemType::Usb => "single side-band (upper)",
            AmpModemType::Lsb => "single side-band (lower)",
        };
        write!(f, "{}", type_)
    }
}
