use std::mem::transmute;


bitflags! {

    pub struct AgcSquelchMode: u32 {

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

bitflags! {
    /// Defines the types of csound bus cahnnels
    ///
    /// and if the channel is an input or an output
    pub struct KeyCallbackType: u8 {
        /// Unknown channel - use it to request the channel type
        const CSOUND_CALLBACK_KBD_EVENT = 1;
        const CSOUND_CALLBACK_KBD_TEXT =  2;
    }
}




impl From<u8> for FileTypes {
    fn from(item: u8) -> Self {
        if item > 63 {
            FileTypes::CSFTYPE_UNKNOWN
        } else {
            unsafe { transmute(item) }
        }
    }
}
