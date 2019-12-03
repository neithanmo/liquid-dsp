use std::fmt;

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

