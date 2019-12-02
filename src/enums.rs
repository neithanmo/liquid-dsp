
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
