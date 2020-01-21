extern crate libc;
#[macro_use]
extern crate bitflags;
extern crate liquid_dsp_sys;
extern crate num;

mod agc;
mod autocorr;
mod cbuffer;
mod channel;
mod cvsd;
mod fec;
mod fft;
mod filter;
mod tvmpch;
mod modem;

mod callbacks;
mod enums;
mod errors;
mod utils;

pub use agc::AgcCrcf;
pub use autocorr::{AutoCorrCccf, AutoCorrRrrf};
pub use cbuffer::{CbufferCf, CbufferRf};
pub use channel::ChannelCccf;
pub use cvsd::Cvsd;
pub use fec::{Fec, Interleaver, Packetizer};
pub use fft::{AsgramCf, AsgramRf, Fft, FftPlan};
pub use filter::{
    BandPass, Bessel, Butter, Cheby1, Cheby2, Ellip, Fir, FirFiltCccf, FirFiltCrcf, FirFiltRrrf,
    FirHilbt, Firdes, FirdesFilterType, Firdespm, FirdespmBtype, FirdespmWtype, HighPass,
    IirFiltCccf, IirFiltCrcf, IirFiltRrrf, IirHilbt, LowPass, Zpk,
    FftFiltCccf, FftFiltCrcf, FftFiltRrrf,
};
pub use tvmpch::TvmpchCccf;

pub use modem::{AmpModem, AmpModemType};

pub use enums::{AgcSquelchMode, CrcScheme, FecScheme, FftType};

pub use errors::LiquidError;

pub type LiquidResult<T> = Result<T, LiquidError>;
